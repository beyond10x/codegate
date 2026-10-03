//! Syntax-aware physical lines over exact collected UTF-8 bytes.
use crate::semantic_model::*;
use tree_sitter::{Node, Parser, Tree};

pub struct ClassifiedSource {
    pub lines: Vec<SourceLine>,
    pub tree: Tree,
    pub gaps: Vec<Gap>,
}
fn range(path: &str, node: Node<'_>) -> SourceRange {
    let start = node.start_position();
    let end = node.end_position();
    SourceRange {
        path: path.into(),
        start_byte: node.start_byte() as i64,
        end_byte: node.end_byte() as i64,
        start_line: start.row as i64,
        start_column: start.column as i64,
        end_line: end.row as i64,
        end_column: end.column as i64,
    }
}
pub fn parse_source(language: Language, path: &str, source: &str) -> Result<ClassifiedSource, Gap> {
    let grammar = match language {
        Language::Go => tree_sitter_go::LANGUAGE,
        Language::Rust => tree_sitter_rust::LANGUAGE,
        Language::Java => tree_sitter_java::LANGUAGE,
    };
    let mut parser = Parser::new();
    parser.set_language(&grammar.into()).map_err(|e| Gap {
        code: GapCode::ToolFailure,
        reason: format!("source grammar unavailable: {e}"),
        location: None,
    })?;
    let tree = parser.parse(source, None).ok_or_else(|| Gap {
        code: GapCode::PartialCollection,
        reason: "source parser returned no tree".into(),
        location: None,
    })?;
    let mut comments = Vec::new();
    let mut literals = Vec::new();
    let mut gaps = Vec::new();
    let mut pending = vec![tree.root_node()];
    while let Some(node) = pending.pop() {
        if node.is_error() || node.is_missing() {
            gaps.push(Gap {
                code: GapCode::SyntaxError,
                reason: format!("syntax error in {path}"),
                location: Some(range(path, node)),
            });
        }
        if matches!(node.kind(), "comment" | "line_comment" | "block_comment") {
            comments.push((node.start_byte(), node.end_byte()));
            continue;
        }
        if matches!(
            node.kind(),
            "string_literal"
                | "raw_string_literal"
                | "interpreted_string_literal"
                | "character_literal"
                | "char_literal"
                | "text_block"
        ) {
            literals.push((node.start_byte(), node.end_byte()));
            continue;
        }
        let mut cursor = node.walk();
        pending.extend(node.children(&mut cursor));
    }
    comments.sort_unstable();
    literals.sort_unstable();
    if tree.root_node().has_error() && gaps.is_empty() {
        gaps.push(Gap {
            code: GapCode::SyntaxError,
            reason: format!("syntax error in {path}"),
            location: Some(range(path, tree.root_node())),
        });
    }
    gaps.sort_by_key(|g| g.location.as_ref().map(|r| (r.start_byte, r.end_byte)));
    let mut offset = 0;
    let mut lines = Vec::new();
    let mut comment_index = 0;
    let mut literal_index = 0;
    for (number, line) in source.split_inclusive('\n').enumerate() {
        let end = offset + line.len();
        while comment_index < comments.len() && comments[comment_index].1 <= offset {
            comment_index += 1
        }
        while literal_index < literals.len() && literals[literal_index].1 <= offset {
            literal_index += 1
        }
        let has_comment = comments
            .get(comment_index)
            .is_some_and(|(start, _)| *start < end);
        let in_literal = literals
            .get(literal_index)
            .is_some_and(|(start, _)| *start < end);
        let mut active_comment = comment_index;
        let has_code = in_literal
            || line.char_indices().any(|(i, c)| {
                while active_comment < comments.len() && comments[active_comment].1 <= offset + i {
                    active_comment += 1;
                }
                !c.is_whitespace()
                    && !comments
                        .get(active_comment)
                        .is_some_and(|(start, _)| *start <= offset + i)
            });
        let classification = match (has_code, has_comment) {
            (true, true) => LineClass::Mixed,
            (true, false) => LineClass::Code,
            (false, true) => LineClass::Comment,
            (false, false) => LineClass::Blank,
        };
        lines.push(SourceLine {
            number: number as i64,
            start_byte: offset as i64,
            end_byte: end as i64,
            classification,
        });
        offset = end;
    }
    Ok(ClassifiedSource { lines, tree, gaps })
}
