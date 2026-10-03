//! Static module declarations only; executable build semantics remain explicit gaps.
use super::{Gap, GapCode, failure};
use crate::semantic_model::BuildSystem;
use std::collections::BTreeMap;
#[derive(Debug)]
pub struct Parsed {
    pub system: BuildSystem,
    pub declared_name: Option<String>,
    pub children: Vec<String>,
    pub parent: Option<String>,
    pub gaps: Vec<Gap>,
}
pub fn recognized(path: &str) -> bool {
    let name = path.rsplit('/').next().unwrap_or(path);
    matches!(
        name,
        "Cargo.toml"
            | "Cargo.lock"
            | "go.mod"
            | "go.sum"
            | "go.work"
            | "go.work.sum"
            | "pom.xml"
            | "settings.gradle"
            | "settings.gradle.kts"
            | "build.gradle"
            | "build.gradle.kts"
            | "gradle.properties"
            | "application.properties"
            | "application.yaml"
            | "application.yml"
    ) || path.ends_with(".cargo/config.toml")
        || path.ends_with(".mvn/maven.config")
        || path.ends_with(".mvn/jvm.config")
}
fn incomplete(path: &str, reason: &str) -> Gap {
    failure(GapCode::MissingBuildSelection, format!("{path}: {reason}"))
}
pub fn parse(path: &str, text: &str) -> Option<Parsed> {
    let name = path.rsplit('/').next()?;
    let system = match name {
        "Cargo.toml" => BuildSystem::Cargo,
        "go.mod" | "go.work" => BuildSystem::GoModules,
        "pom.xml" => BuildSystem::Maven,
        "settings.gradle" | "settings.gradle.kts" | "build.gradle" | "build.gradle.kts" => {
            BuildSystem::Gradle
        }
        _ => return None,
    };
    let mut out = Parsed {
        system,
        declared_name: None,
        children: vec![],
        parent: None,
        gaps: vec![],
    };
    match system {
        BuildSystem::Cargo => match text.parse::<toml::Value>() {
            Err(_) => out.gaps.push(incomplete(path, "invalid Cargo manifest")),
            Ok(value) => {
                out.declared_name = value
                    .get("package")
                    .and_then(|p| p.get("name"))
                    .and_then(toml::Value::as_str)
                    .map(str::to_string);
                if let Some(workspace) = value.get("workspace") {
                    if let Some(members) = workspace.get("members") {
                        if let Some(members) = members.as_array() {
                            for member in members {
                                match member.as_str() {
                                    Some(member) if !member.contains(['*', '?', '[']) => {
                                        out.children.push(format!("{member}/Cargo.toml"))
                                    }
                                    _ => out.gaps.push(incomplete(
                                        path,
                                        "nonliteral Cargo workspace member",
                                    )),
                                }
                            }
                        } else {
                            out.gaps
                                .push(incomplete(path, "invalid Cargo workspace members"))
                        }
                    }
                    if workspace.get("exclude").is_some() {
                        out.gaps.push(incomplete(
                            path,
                            "Cargo workspace excludes require explicit selection",
                        ))
                    }
                }
                if let Some(parent) = value.get("package").and_then(|p| p.get("workspace")) {
                    if let Some(parent) = parent.as_str() {
                        out.parent = Some(format!("{parent}/Cargo.toml"))
                    } else {
                        out.gaps
                            .push(incomplete(path, "nonliteral Cargo workspace parent"))
                    }
                }
            }
        },
        BuildSystem::GoModules => {
            let tokens = lex(text);
            let mut in_use = false;
            let mut i = 0;
            while i < tokens.len() {
                match tokens[i].as_str() {
                    "module" => {
                        if let Some(name) = tokens.get(i + 1) {
                            out.declared_name = Some(name.trim_start_matches('\u{1}').into());
                            i += 1
                        }
                    }
                    "use" => {
                        if tokens.get(i + 1).is_some_and(|x| x == "(") {
                            in_use = true;
                            i += 1
                        } else if let Some(p) = tokens.get(i + 1) {
                            out.children
                                .push(format!("{}/go.mod", p.trim_start_matches('\u{1}')));
                            i += 1
                        }
                    }
                    ")" => in_use = false,
                    "replace" => out.gaps.push(incomplete(
                        path,
                        "Go replacements require explicit build selection",
                    )),
                    token if in_use && token != "(" && token != "\n" => out
                        .children
                        .push(format!("{}/go.mod", token.trim_start_matches('\u{1}'))),
                    _ => {}
                }
                i += 1;
            }
            if in_use {
                out.gaps
                    .push(incomplete(path, "unterminated Go workspace use block"))
            }
            if name == "go.mod" && out.declared_name.is_none() {
                out.gaps
                    .push(incomplete(path, "missing Go module declaration"))
            }
        }
        BuildSystem::Maven => {
            use quick_xml::{Reader, events::Event};
            let mut reader = Reader::from_str(text);
            // XML text can be split by CDATA/comments/entities. Preserve chunks
            // verbatim, then trim the completed scalar rather than each chunk.
            reader.config_mut().trim_text(false);
            let mut stack = Vec::<String>::new();
            let mut values = BTreeMap::<String, String>::new();
            let mut has_parent = false;
            let mut explicit_parent = false;
            loop {
                match reader.read_event() {
                    Ok(Event::Start(e)) => {
                        let n = String::from_utf8_lossy(e.local_name().as_ref()).into_owned();
                        if n == "parent" {
                            has_parent = true
                        }
                        if n == "relativePath" {
                            explicit_parent = true
                        }
                        stack.push(n)
                    }
                    Ok(Event::Empty(e)) => {
                        if e.local_name().as_ref() == b"relativePath" {
                            explicit_parent = true;
                            out.gaps
                                .push(incomplete(path, "external Maven parent is unavailable"))
                        }
                    }
                    Ok(Event::Text(e)) => match e.unescape() {
                        Ok(value) => {
                            let key = stack.join("/");
                            values.entry(key).or_default().push_str(&value)
                        }
                        Err(_) => out.gaps.push(incomplete(path, "invalid Maven text entity")),
                    },
                    Ok(Event::CData(e)) => match e.decode() {
                        Ok(value) => {
                            values.entry(stack.join("/")).or_default().push_str(&value);
                        }
                        Err(_) => out.gaps.push(incomplete(path, "invalid Maven CDATA text")),
                    },
                    Ok(Event::End(_)) => {
                        let key = stack.join("/");
                        if let Some(value) = values.remove(&key) {
                            let value = value.trim().to_owned();
                            if value.contains("${") {
                                out.gaps.push(incomplete(path, "unresolved Maven property"))
                            } else if key == "project/modules/module" {
                                out.children.push(format!("{value}/pom.xml"))
                            } else if key == "project/parent/relativePath" {
                                out.parent = Some(value)
                            } else if key == "project/artifactId" {
                                out.declared_name = Some(value)
                            } else if key.contains("/profiles/") && key.ends_with("/module") {
                                out.gaps
                                    .push(incomplete(path, "profile-dependent Maven module"))
                            }
                        }
                        stack.pop();
                    }
                    Ok(Event::DocType(_)) => out
                        .gaps
                        .push(incomplete(path, "Maven document type is not interpreted")),
                    Ok(Event::Eof) => {
                        // The streaming reader permits EOF with unmatched start
                        // elements; a truncated document is incomplete evidence.
                        if !stack.is_empty() {
                            out.gaps.push(incomplete(path, "invalid Maven XML"));
                        }
                        break;
                    }
                    Err(_) => {
                        out.gaps.push(incomplete(path, "invalid Maven XML"));
                        break;
                    }
                    _ => {}
                }
            }
            if has_parent && !explicit_parent {
                out.parent = Some("../pom.xml".into())
            }
        }
        BuildSystem::Gradle => {
            let tokens = lex(text);
            let mut i = 0;
            while i < tokens.len() {
                if tokens[i] == "include" {
                    i += 1;
                    let mut found = false;
                    while i < tokens.len() && tokens[i] != "\n" && tokens[i] != ";" {
                        let token = &tokens[i];
                        if token == "(" || token == ")" || token == "," {
                            i += 1;
                            continue;
                        }
                        if let Some(token) = token.strip_prefix('\u{1}') {
                            let child = token.trim_start_matches(':').replace(':', "/");
                            if !child.is_empty() && !child.contains('$') {
                                out.children.push(format!("{child}/build.gradle"));
                                found = true
                            } else {
                                out.gaps
                                    .push(incomplete(path, "computed Gradle module include"))
                            }
                        } else {
                            out.gaps
                                .push(incomplete(path, "computed Gradle module include"))
                        }
                        i += 1;
                    }
                    if !found {
                        out.gaps
                            .push(incomplete(path, "unresolved Gradle module include"))
                    }
                } else if tokens[i].ends_with("projectDir")
                    || tokens[i] == "includeBuild"
                    || tokens[i] == "apply"
                    || matches!(
                        tokens[i].as_str(),
                        "if" | "for" | "while" | "when" | "def" | "fun"
                    )
                {
                    out.gaps
                        .push(incomplete(path, "dynamic Gradle project configuration"));
                    i += 1
                } else {
                    i += 1
                }
            }
        }
    }
    Some(out)
}
// Tokenizer preserves quoted tokens (prefix marker), strips comments and never evaluates expressions.
fn lex(text: &str) -> Vec<String> {
    let mut chars = text.chars().peekable();
    let mut out = Vec::new();
    while let Some(c) = chars.next() {
        if c == '\n' {
            out.push("\n".into());
            continue;
        }
        if c.is_whitespace() {
            continue;
        }
        if c == '/' && chars.peek() == Some(&'/') {
            for c in chars.by_ref() {
                if c == '\n' {
                    out.push("\n".into());
                    break;
                }
            }
            continue;
        }
        if c == '/' && chars.peek() == Some(&'*') {
            chars.next();
            let mut last = '\0';
            for next in chars.by_ref() {
                if last == '*' && next == '/' {
                    break;
                }
                last = next
            }
            continue;
        }
        if c == '\'' || c == '"' || c == '`' {
            let mut token = String::from("\u{1}");
            while let Some(next) = chars.next() {
                if next == c {
                    break;
                }
                if next == '\\' {
                    if let Some(escaped) = chars.next() {
                        token.push(escaped)
                    }
                } else {
                    token.push(next)
                }
            }
            out.push(token);
            continue;
        }
        if "(),;=".contains(c) {
            out.push(c.to_string());
            continue;
        }
        let mut token = c.to_string();
        while let Some(next) = chars.peek() {
            if next.is_whitespace() || "(),;=\"'`".contains(*next) {
                break;
            }
            token.push(chars.next().unwrap())
        }
        out.push(token);
    }
    out
}
