//! Architectural regression gate: inspect executable syntax, not source substrings.
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    sync::LazyLock,
};
use syn::{
    Item, UseTree,
    visit::{self, Visit},
};

#[derive(Clone, Copy, PartialEq)]
enum Role {
    Algorithm,
    Admission,
    Projection,
    Root,
}
// External libraries are denied in the pure core unless their admitted use is
// structural serialization, generated values, or deterministic hashing. Discover
// normal dependencies from Cargo rather than maintaining a denylist of today's
// known IO libraries. A new dependency therefore cannot silently evade this gate.
const PURE_DEPENDENCIES: &[&str] = &[
    "serde",
    "serde_json",
    "sha2",
    "codegate_behavior",
    "codegate_contract",
    "codegate_semantic_behavior",
    "codegate_semantic_contract",
];
// Refuse the entire host/platform namespaces: std::os contains platform-specific
// sockets, raw descriptors and process APIs below several intermediate modules.
// Thread identity is contextual evidence even when it performs no filesystem IO.
const HOST_CONTEXT_MODULES: &[&str] = &["fs", "env", "net", "process", "io", "thread", "os"];
static NORMAL_DEPENDENCIES: LazyLock<BTreeSet<String>> = LazyLock::new(|| {
    fn collect(value: &toml::Value, dependencies: &mut BTreeSet<String>) {
        if let Some(table) = value.as_table() {
            for (key, value) in table {
                if key == "dependencies" {
                    let table = value.as_table().expect("Cargo dependency table");
                    dependencies.extend(table.keys().map(|key| key.replace('-', "_")));
                } else if !matches!(key.as_str(), "dev-dependencies" | "build-dependencies") {
                    collect(value, dependencies);
                }
            }
        }
    }
    let manifest =
        fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml")).unwrap();
    let value: toml::Value = manifest.parse().unwrap();
    let mut dependencies = BTreeSet::new();
    collect(&value, &mut dependencies);
    dependencies
});
fn use_paths(tree: &UseTree, prefix: Vec<String>, out: &mut BTreeMap<String, Vec<String>>) {
    match tree {
        UseTree::Path(p) => {
            let mut prefix = prefix;
            prefix.push(p.ident.to_string());
            use_paths(&p.tree, prefix, out);
        }
        UseTree::Name(n) => {
            let mut path = prefix;
            path.push(n.ident.to_string());
            out.insert(n.ident.to_string(), path);
        }
        UseTree::Rename(n) => {
            let mut path = prefix;
            path.push(n.ident.to_string());
            out.insert(n.rename.to_string(), path);
        }
        UseTree::Group(g) => {
            for item in &g.items {
                use_paths(item, prefix.clone(), out);
            }
        }
        UseTree::Glob(_) => {
            out.insert(format!("*{}", out.len()), prefix);
        }
    }
}
struct Scanner {
    role: Role,
    aliases: BTreeMap<String, Vec<String>>,
    errors: Vec<String>,
}
impl Scanner {
    fn path(&mut self, path: &syn::Path) {
        let mut parts: Vec<_> = path.segments.iter().map(|s| s.ident.to_string()).collect();
        // Resolve renamed/grouped imports before checking, including aliases of aliases.
        let mut expanded = BTreeSet::new();
        while let Some(first) = parts.first().cloned() {
            if !expanded.insert(first.clone()) {
                break;
            }
            let Some(alias) = self.aliases.get(&first) else {
                break;
            };
            let mut next = alias.clone();
            next.extend(parts.into_iter().skip(1));
            parts = next;
        }
        if let Some(dependency) = parts.first()
            && NORMAL_DEPENDENCIES.contains(dependency)
            && !PURE_DEPENDENCIES.contains(&dependency.as_str())
        {
            self.errors.push(format!(
                "external dependency not admitted to pure core: {dependency}"
            ));
        }
        if parts.iter().any(|p| {
            matches!(
                p.as_str(),
                "collection"
                    | "bindings"
                    | "foundation"
                    | "tree_sitter"
                    | "git2"
                    | "rustix"
                    | "quick_xml"
                    | "toml"
            )
        }) {
            self.errors
                .push(format!("core dependency: {}", parts.join("::")));
        }
        if parts
            .last()
            .is_some_and(|p| matches!(p.as_str(), "collect" | "collect_source" | "assess"))
        {
            self.errors
                .push("pure function calls a collection adapter".into());
        }
        if parts.first().is_some_and(|p| p == "std")
            && parts
                .get(1)
                .is_some_and(|p| HOST_CONTEXT_MODULES.contains(&p.as_str()))
            || parts
                .iter()
                .any(|p| matches!(p.as_str(), "SystemTime" | "Instant"))
        {
            self.errors.push(format!("IO/clock: {}", parts.join("::")));
        }
        if matches!(self.role, Role::Algorithm | Role::Root)
            && parts
                .iter()
                .any(|p| matches!(p.as_str(), "Language" | "Go" | "Rust" | "Java"))
        {
            self.errors.push("language-specific algorithm".into());
        }
    }
}
impl<'ast> Visit<'ast> for Scanner {
    fn visit_pat_struct(&mut self, node: &'ast syn::PatStruct) {
        if matches!(self.role, Role::Algorithm | Role::Root) {
            for field in &node.fields {
                if let syn::Member::Named(name) = &field.member
                    && matches!(
                        name.to_string().as_str(),
                        "language" | "producer" | "producer_version" | "annotation_name"
                    )
                {
                    self.errors
                        .push("language/producer destructuring in shared algorithm".into());
                }
            }
        }
        visit::visit_pat_struct(self, node);
    }
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        if matches!(
            node.method.to_string().as_str(),
            "canonicalize"
                | "read_link"
                | "metadata"
                | "symlink_metadata"
                | "read_dir"
                | "try_exists"
        ) {
            self.errors.push("filesystem method in pure code".into());
        }
        visit::visit_expr_method_call(self, node);
    }
    fn visit_item_extern_crate(&mut self, node: &'ast syn::ItemExternCrate) {
        let name = node.ident.to_string();
        self.path(&syn::parse_str::<syn::Path>(&name).unwrap());
        let alias = node
            .rename
            .as_ref()
            .map(|(_, id)| id.to_string())
            .unwrap_or_else(|| name.clone());
        self.aliases.insert(alias, vec![name]);
    }
    fn visit_item_use(&mut self, node: &'ast syn::ItemUse) {
        let mut aliases = BTreeMap::new();
        use_paths(&node.tree, vec![], &mut aliases);
        for (name, path) in aliases {
            let parsed = syn::parse_str::<syn::Path>(&path.join("::"));
            if let Ok(path) = parsed {
                self.path(&path);
            }
            self.aliases.insert(name, path);
        }
    }
    fn visit_path(&mut self, path: &'ast syn::Path) {
        self.path(path);
        visit::visit_path(self, path);
    }
    fn visit_expr_field(&mut self, node: &'ast syn::ExprField) {
        if matches!(self.role, Role::Algorithm | Role::Root)
            && let syn::Member::Named(member) = &node.member
            && matches!(
                member.to_string().as_str(),
                "language" | "producer" | "producer_version" | "annotation_name"
            )
        {
            self.errors
                .push("language/producer/annotation access in shared algorithm".into());
        }
        visit::visit_expr_field(self, node);
    }
    fn visit_expr_unsafe(&mut self, _: &'ast syn::ExprUnsafe) {
        self.errors.push("unsafe core block".into());
    }
    fn visit_item_foreign_mod(&mut self, _: &'ast syn::ItemForeignMod) {
        self.errors.push("foreign core dependency".into());
    }
    fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {
        // Module declarations are wiring, never permission to call their adapters.
        if let Some((_, items)) = &node.content {
            for item in items {
                self.visit_item(item);
            }
        }
    }
    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        let name = node
            .path
            .segments
            .last()
            .map(|s| s.ident.to_string())
            .unwrap_or_default();
        if name == "include_str"
            && self.role == Role::Projection
            && syn::parse2::<syn::LitStr>(node.tokens.clone())
                .is_ok_and(|v| v.value() == "../generated/semantic-wire/source.schema.json")
        {
            return;
        }
        if self.role == Role::Projection
            && matches!(
                name.as_str(),
                "record"
                    | "enumeration"
                    | "identity"
                    | "primitive"
                    | "bridge"
                    | "struct_bridge"
                    | "enum_bridge"
                    | "newtype_bridge"
            )
        {
            return;
        }
        if !(matches!(
            name.as_str(),
            "format"
                | "vec"
                | "json"
                | "matches"
                | "assert"
                | "assert_eq"
                | "debug_assert"
                | "unreachable"
                | "write"
                | "writeln"
        ) || name == "macro_rules" && self.role == Role::Projection)
        {
            self.errors.push(format!("uninspected core macro: {name}"));
            return;
        }
        // Rust macros can contain executable expressions. Inspect token groups as paths,
        // and field-sensitive identifiers as well; a string literal is never executable.
        fn tokens(scanner: &mut Scanner, mut cursor: syn::buffer::Cursor<'_>) {
            while !cursor.eof() {
                if let Some((ident, next)) = cursor.ident() {
                    let word = ident.to_string();
                    if HOST_CONTEXT_MODULES.contains(&word.as_str())
                        || matches!(
                            word.as_str(),
                            "collection" | "bindings" | "foundation" | "SystemTime" | "Instant"
                        )
                    {
                        scanner
                            .errors
                            .push(format!("forbidden macro token: {word}"));
                    }
                    if matches!(scanner.role, Role::Algorithm | Role::Root)
                        && matches!(
                            word.as_str(),
                            "language"
                                | "Language"
                                | "producer"
                                | "annotation_name"
                                | "Go"
                                | "Rust"
                                | "Java"
                        )
                    {
                        scanner
                            .errors
                            .push("language/producer macro dispatch".into());
                    }
                    if (scanner.aliases.contains_key(&word) || NORMAL_DEPENDENCIES.contains(&word))
                        && let Ok(path) = syn::parse_str::<syn::Path>(&word)
                    {
                        scanner.path(&path);
                    }
                    cursor = next;
                } else if let Some((inside, _, _, next)) = cursor.any_group() {
                    tokens(scanner, inside);
                    cursor = next;
                } else if let Some((_, next)) = cursor.token_tree() {
                    cursor = next;
                } else {
                    break;
                }
            }
        }
        let buffer = syn::buffer::TokenBuffer::new2(node.tokens.clone());
        tokens(self, buffer.begin());
    }
}
fn violations(source: &str, role: Role) -> Vec<String> {
    let file = match syn::parse_file(source) {
        Ok(file) => file,
        Err(error) => return vec![format!("unparsed core: {error}")],
    };
    let mut scanner = Scanner {
        role,
        aliases: BTreeMap::new(),
        errors: vec![],
    };
    for item in &file.items {
        if let Item::Use(item) = item {
            use_paths(&item.tree, vec![], &mut scanner.aliases);
        }
    }
    for item in &file.items {
        // These exact public exports wire the collection API. Aliases were
        // registered above, so an algorithm using either export is still refused.
        if role == Role::Root
            && let Item::Use(export) = item
            && matches!(export.vis, syn::Visibility::Public(_))
        {
            let mut paths = BTreeMap::new();
            use_paths(&export.tree, vec![], &mut paths);
            if !paths.is_empty()
                && paths.values().all(|path| {
                    path == &["collection", "compose", "Collector"]
                        || path == &["collection", "compose", "collect"]
                })
            {
                continue;
            }
        }
        // Explicit root orchestration is kept outside the pure algorithm check.
        if role == Role::Root
            && matches!(item,Item::Fn(function) if matches!(function.sig.ident.to_string().as_str(),"collect"|"collect_source"|"assess"))
        {
            continue;
        }
        scanner.visit_item(item);
    }
    scanner.errors.sort();
    scanner.errors.dedup();
    scanner.errors
}

#[test]
fn injected_io_binding_and_dispatch_are_rejected_by_ast() {
    for source in [
        "fn evaluate(){ std::fs::read(\"x\"); }",
        "use std::{fs as disk}; fn evaluate(){ disk::read(\"x\"); }",
        "use crate::bindings::go as parser; fn evaluate(){ parser::extract(); }",
        "fn evaluate(){ crate::collection::collect(); }",
        "fn evaluate(){ std::process::Command::new(\"go\"); }",
        "fn evaluate(){ std::env::var(\"HOME\"); }",
        "fn evaluate(){ std::time::SystemTime::now(); }",
        "fn evaluate(){ std::thread::sleep(duration); }",
        "fn evaluate(x: Fact){ match x.language { Language::Rust=>1, _=>0 }; }",
        "fn evaluate(x: Fact){ let language=x.language; if language==Language::Rust {} }",
        "fn evaluate(x: Fact){ if matches!(x.language, Language::Rust) {} }",
        "fn evaluate(x: Fact){ if x.producer==\"parser\" {} }",
        "fn evaluate(x: Fact){let Fact{language: selected,..}=x; if selected==configured {}}",
        "fn evaluate(){ include!(\"unchecked.rs\"); }",
        "fn evaluate(){ let _=env!(\"HOME\"); }",
        "extern crate std as platform; fn evaluate(){platform::fs::read(\"x\");}",
        "fn evaluate(){crate::collect();}",
        "fn evaluate(){ unsafe { external_call(); } }",
    ] {
        assert!(
            !violations(source, Role::Algorithm).is_empty(),
            "not rejected: {source}"
        );
    }
}
#[test]
fn wiring_projection_and_neutral_admission_are_distinct_from_algorithms() {
    assert!(violations("pub mod collection; pub mod bindings; pub mod foundation; fn evaluate(){ crate::admit::admit(); }",Role::Root).is_empty());
    assert!(violations("fn encode(x:Language){match x {Language::Rust=>\"Rust\",Language::Go=>\"Go\",Language::Java=>\"Java\"};}",Role::Projection).is_empty());
    assert!(
        violations(
            "fn validate(x:Fact,y:Fact){ let _=x.language==y.language; }",
            Role::Admission
        )
        .is_empty()
    );
    assert!(
        !violations(
            "pub mod collection; fn evaluate(){ collection::collect(); }",
            Role::Root
        )
        .is_empty()
    );
    assert!(
        !violations(
            "impl Evaluator { fn evaluate(&self){ std::fs::read(\"x\"); } }",
            Role::Root
        )
        .is_empty()
    );
    assert!(!violations("fn encode(){std::fs::read(\"x\");}", Role::Projection).is_empty());
}
#[test]
fn new_and_inline_core_modules_cannot_escape_discovery() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(".scratch/unit/boundary-discovery");
    fs::create_dir_all(&root).unwrap();
    fs::write(root.join("lib.rs"), "mod new_analysis;").unwrap();
    fs::write(
        root.join("new_analysis.rs"),
        "fn check(){std::fs::read(\"x\");}",
    )
    .unwrap();
    assert!(!scan_tree(&root).unwrap().is_empty());
    assert!(!violations("mod inline {fn check(){std::fs::read(\"x\");}}", Role::Root).is_empty());
}
fn scan_tree(root: &Path) -> Result<Vec<String>, String> {
    fn walk(
        path: PathBuf,
        role: Role,
        seen: &mut BTreeSet<PathBuf>,
        errors: &mut Vec<String>,
    ) -> Result<(), String> {
        if !seen.insert(path.clone()) {
            return Err("cyclic Rust module paths".into());
        }
        let source = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        errors.extend(
            violations(&source, role)
                .into_iter()
                .map(|error| format!("{}: {error}", path.file_name().unwrap().to_string_lossy())),
        );
        let file = syn::parse_file(&source).map_err(|e| e.to_string())?;
        let parent = path.parent().ok_or("module parent missing")?;
        let module_dir = if path
            .file_name()
            .is_some_and(|n| n == "lib.rs" || n == "mod.rs")
        {
            parent.to_path_buf()
        } else {
            parent.join(path.file_stem().unwrap())
        };
        fn children(
            items: &[Item],
            dir: &Path,
            role: Role,
            seen: &mut BTreeSet<PathBuf>,
            errors: &mut Vec<String>,
        ) -> Result<(), String> {
            for item in items {
                if let Item::Mod(module) = item {
                    let name = module.ident.to_string();
                    if matches!(name.as_str(), "collection" | "bindings" | "foundation") {
                        continue;
                    }
                    let child_role = match name.as_str() {
                        "source_identity" | "semantic_wire" | "wire" => Role::Projection,
                        "admit" => Role::Admission,
                        _ => {
                            if role == Role::Root {
                                Role::Algorithm
                            } else {
                                role
                            }
                        }
                    };
                    if let Some((_, items)) = &module.content {
                        children(items, &dir.join(&name), child_role, seen, errors)?;
                        continue;
                    }
                    let custom = module.attrs.iter().find_map(|attr| {
                        if attr.path().is_ident("path") {
                            if let syn::Meta::NameValue(n) = &attr.meta
                                && let syn::Expr::Lit(l) = &n.value
                                && let syn::Lit::Str(s) = &l.lit
                            {
                                return Some(s.value());
                            }
                            None
                        } else {
                            None
                        }
                    });
                    let child = if let Some(path) = custom {
                        dir.join(path)
                    } else {
                        let direct = dir.join(format!("{name}.rs"));
                        if direct.exists() {
                            direct
                        } else {
                            dir.join(&name).join("mod.rs")
                        }
                    };
                    walk(child, child_role, seen, errors)?;
                }
            }
            Ok(())
        }
        children(&file.items, &module_dir, role, seen, errors)
    }
    let mut errors = vec![];
    walk(
        root.join("lib.rs"),
        Role::Root,
        &mut BTreeSet::new(),
        &mut errors,
    )?;
    Ok(errors)
}
#[test]
fn public_collection_exports_are_wiring_but_core_calls_remain_forbidden() {
    let exports = "pub use collection::compose::{Collector, collect};";
    assert!(violations(exports, Role::Root).is_empty());
    for algorithm in [
        "fn evaluate() { collect(input); }",
        "fn evaluate() { Collector::default(); }",
    ] {
        assert!(!violations(&format!("{exports}{algorithm}"), Role::Root).is_empty());
    }
    assert!(!violations(exports, Role::Algorithm).is_empty());
    assert!(!violations("pub use collection::compose::*;", Role::Root).is_empty());
    assert!(
        !violations(
            "pub use collection::compose::collect as adapter; fn evaluate() { adapter(input); }",
            Role::Root,
        )
        .is_empty()
    );
}
#[test]
fn all_shared_modules_and_root_algorithms_obey_boundary() {
    let result = scan_tree(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src")).unwrap();
    assert!(result.is_empty(), "{result:#?}");
}

#[test]
fn indirect_process_argument_io_is_rejected() {
    for source in [
        "fn evaluate(){clap::Command::new(\"x\").get_matches();}",
        "use clap::Command as Args; fn evaluate(){Args::new(\"x\").get_matches();}",
    ] {
        assert!(
            !violations(source, Role::Algorithm).is_empty(),
            "IO dependency escaped: {source}"
        );
    }
}

#[test]
fn nested_platform_io_and_thread_context_are_rejected() {
    for source in [
        "fn evaluate(){std::os::unix::net::UnixStream::connect(\"/tmp/socket\");}",
        "fn evaluate(){let _=std::thread::current().id();}",
    ] {
        assert!(
            !violations(source, Role::Algorithm).is_empty(),
            "platform effect escaped: {source}"
        );
    }
}

#[test]
fn platform_and_thread_aliases_share_the_effect_guard() {
    for path in [
        "std::os::unix::net::UnixStream",
        "std::os::unix::fs::OpenOptionsExt",
        "std::os::windows::io::OwnedHandle",
        "std::os::fd::OwnedFd",
        "std::thread",
    ] {
        for source in [
            format!("fn evaluate(){{{path}::operation();}}"),
            format!("use {path} as context; fn evaluate(){{context::operation();}}"),
            format!("fn evaluate(){{format!(\"{{:?}}\", {path}::operation());}}"),
        ] {
            assert!(
                !violations(&source, Role::Algorithm).is_empty(),
                "host context escaped: {source}"
            );
        }
    }
    assert!(
        violations(
            "fn evaluate(){std::collections::BTreeSet::<String>::new();}",
            Role::Algorithm
        )
        .is_empty()
    );
}

#[test]
fn all_non_admitted_dependencies_and_alias_forms_are_rejected() {
    let mut denied = 0;
    for dependency in NORMAL_DEPENDENCIES
        .iter()
        .filter(|name| !PURE_DEPENDENCIES.contains(&name.as_str()))
    {
        for source in [
            format!("fn evaluate(){{ {dependency}::operation(); }}"),
            format!("use {dependency} as input; fn evaluate(){{ input::operation(); }}"),
            format!("use {dependency}::{{Command as Input}}; fn evaluate(){{ Input::new(); }}"),
            format!("extern crate {dependency} as input; fn evaluate(){{ input::operation(); }}"),
            format!("fn evaluate(){{format!(\"{{:?}}\", {dependency}::operation());}}"),
        ] {
            assert!(
                !violations(&source, Role::Algorithm).is_empty(),
                "dependency escaped: {source}"
            );
        }
        denied += 1;
    }
    assert!(
        denied > 0,
        "the gate must exercise the normal IO/parser dependencies"
    );
    assert!(
        violations(
            "fn encode(value: Value){serde_json::to_vec(&value);}",
            Role::Projection
        )
        .is_empty()
    );
}
