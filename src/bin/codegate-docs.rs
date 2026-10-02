//! Build the bounded, script-free public documentation and its publication manifest.
use clap::{Parser, Subcommand};
use serde_json::{Value, json};
use std::{collections::BTreeSet, fs, path::PathBuf, process::ExitCode};

const HTML: &str = include_str!("../../website/index.html");
const CSS: &str = include_str!("../../website/styles.css");
const CANONICAL: &str = "https://beyond10x.github.io/codegate/";
const STYLESHEET: &str = "/codegate/styles.css";

#[derive(Parser)]
#[command(
    name = "codegate-docs",
    about = "Validate and build Codegate documentation"
)]
struct Cli {
    #[command(subcommand)]
    command: Action,
}

#[derive(Subcommand)]
enum Action {
    /// Validate the authored page without writing files or accessing the network.
    Check,
    /// Write the static site and bind its manifest to a full Git commit.
    Build {
        #[arg(long)]
        out: PathBuf,
        #[arg(long)]
        commit: String,
    },
}

// The authored site uses lowercase tags and double-quoted attributes. This
// validator deliberately checks that bounded source format, not arbitrary HTML.
fn attributes<'a>(html: &'a str, name: &str) -> Vec<&'a str> {
    let prefix = format!(" {name}=\"");
    html.split(&prefix)
        .skip(1)
        .filter_map(|tail| tail.split('"').next())
        .collect()
}

fn require(condition: bool, message: impl Into<String>) -> Result<(), String> {
    if condition {
        Ok(())
    } else {
        Err(message.into())
    }
}

fn check(html: &str, css: &str) -> Result<(), String> {
    require(
        html.starts_with("<!doctype html>")
            && html.contains("<html lang=\"en\">")
            && html.contains("name=\"viewport\""),
        "missing document declaration, language or mobile viewport",
    )?;
    let ids = attributes(html, "id");
    let unique: BTreeSet<_> = ids.iter().copied().collect();
    require(ids.len() == unique.len(), "duplicate anchor")?;
    require(!unique.contains(""), "empty anchor")?;
    let mut canonical_count = 0;
    let mut stylesheet_count = 0;
    for tag in html.split('<').skip(1).filter_map(|s| s.split('>').next()) {
        let name = tag.split_whitespace().next().unwrap_or_default();
        require(
            !["script", "iframe", "object", "embed", "base"].contains(&name),
            format!("unsupported element {name}"),
        )?;
        if name == "link" {
            let relations = attributes(tag, "rel");
            let links = attributes(tag, "href");
            match relations.as_slice() {
                ["canonical"] => {
                    require(links == [CANONICAL], "wrong canonical URL")?;
                    canonical_count += 1;
                }
                ["stylesheet"] => {
                    require(links == [STYLESHEET], "wrong stylesheet route")?;
                    stylesheet_count += 1;
                }
                _ => return Err("unsupported link relation".into()),
            }
        }
    }
    require(canonical_count == 1, "expected one canonical link")?;
    require(stylesheet_count == 1, "expected one stylesheet link")?;
    for href in attributes(html, "href") {
        if let Some(anchor) = href.strip_prefix('#') {
            require(unique.contains(anchor), format!("missing anchor {anchor}"))?;
        } else {
            require(
                (href.starts_with("https://") || href == STYLESHEET)
                    && !href.contains(".engineering")
                    && !href.contains(".scratch")
                    && !href.contains("/home/")
                    && !href.contains("AGENTS.md")
                    && !href.contains("../"),
                format!("unexpected or internal route {href}"),
            )?;
        }
    }
    require(
        attributes(html, "src").is_empty(),
        "unregistered source asset; the site only ships HTML and CSS",
    )?;
    require(
        !css.trim().is_empty() && !css.contains("url(") && !css.contains("@import"),
        "stylesheet is empty or references an unregistered asset",
    )?;
    Ok(())
}

fn manifest(commit: &str) -> Result<Value, String> {
    require(
        commit.len() == 40
            && commit.bytes().all(|byte| byte.is_ascii_hexdigit())
            && commit.bytes().any(|byte| byte != b'0'),
        "commit must be a nonzero full 40-character Git revision",
    )?;
    Ok(json!({
        "schema": "b10x-project-site/v1",
        "repository": "codegate",
        "commit": commit,
        "baseUrl": "/codegate/"
    }))
}

fn run(cli: Cli) -> Result<(), String> {
    check(HTML, CSS)?;
    match cli.command {
        Action::Check => println!("Codegate documentation: anchors, routes and assets valid"),
        Action::Build { out, commit } => {
            let mut manifest_bytes =
                serde_json::to_vec_pretty(&manifest(&commit)?).map_err(|e| e.to_string())?;
            manifest_bytes.push(b'\n');
            fs::create_dir_all(out.join(".well-known")).map_err(|e| e.to_string())?;
            for (name, bytes) in [
                ("index.html", HTML.as_bytes()),
                ("styles.css", CSS.as_bytes()),
                (".nojekyll", b"".as_slice()),
                (".well-known/b10x-site.json", manifest_bytes.as_slice()),
            ] {
                let path = out.join(name);
                fs::write(&path, bytes).map_err(|e| format!("{}: {e}", path.display()))?;
            }
            println!("Codegate documentation built at {}", out.display());
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("codegate-docs: {error}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authored_site_and_publication_identity_are_valid() {
        check(HTML, CSS).unwrap();
        let commit = "1234567890abcdef1234567890abcdef12345678";
        assert_eq!(
            manifest(commit).unwrap(),
            json!({"schema":"b10x-project-site/v1", "repository":"codegate",
                "commit":commit, "baseUrl":"/codegate/"})
        );
    }

    #[test]
    fn broken_or_duplicate_anchors_are_refused() {
        assert!(check(&HTML.replace("id=\"quickstart\"", "id=\"missing\""), CSS).is_err());
        assert!(check(&HTML.replace("id=\"quickstart\"", "id=\"main\""), CSS).is_err());
    }

    #[test]
    fn wrong_routes_assets_and_internal_source_links_are_refused() {
        assert!(check(&HTML.replace(STYLESHEET, "/styles.css"), CSS).is_err());
        assert!(check(&HTML.replace(CANONICAL, "https://example.com/"), CSS).is_err());
        assert!(
            check(
                &HTML.replace("href=\"#quickstart\"", "href=\"README.md\""),
                CSS
            )
            .is_err()
        );
        assert!(
            check(
                &HTML.replace("href=\"#quickstart\"", "href=\"https://github.com/beyond10x/codegate/blob/main/.engineering/private.md\""),
                CSS
            ).is_err()
        );
        assert!(check(&format!("{HTML}<img src=\"/missing.png\">"), CSS).is_err());
        assert!(check(HTML, "body { background: url(missing.png); }").is_err());
        assert!(check(&format!("{HTML}<script></script>"), CSS).is_err());
    }

    #[test]
    fn invalid_commits_are_refused() {
        for commit in ["", "main", "1234567", &"0".repeat(40), &"g".repeat(40)] {
            assert!(manifest(commit).is_err(), "accepted {commit}");
        }
    }

    fn example(label: &str) -> &str {
        HTML.split(&format!("aria-label=\"{label}\"><code>"))
            .nth(1)
            .unwrap()
            .split("</code>")
            .next()
            .unwrap()
    }

    #[test]
    fn published_json_example_reaches_the_real_evaluator() {
        use codegate::{
            Evaluator, exit_code, model::Evaluate, model::obligations::EvaluateBehavior, wire,
        };
        let snapshot =
            wire::decode_snapshot(example("Complete facts JSON example").as_bytes()).unwrap();
        let policy =
            wire::decode_policy(example("Complete policy JSON example").as_bytes()).unwrap();
        let mut evaluator = Evaluator::default();
        evaluator.evaluate(Evaluate { snapshot, policy }).unwrap();
        let report = evaluator.take_response().unwrap().report;
        assert_eq!(exit_code(&report), 0);
        let report = wire::report_value(report).unwrap();
        assert_eq!(report["verdict"], "Pass");
        assert_eq!(
            report["fan_out"],
            json!([
                {"unit_id":"core", "value":1}, {"unit_id":"io", "value":0}
            ])
        );
        let snapshot =
            wire::decode_snapshot(example("Complete facts JSON example").as_bytes()).unwrap();
        let mut policy: Value =
            serde_json::from_str(example("Complete policy JSON example")).unwrap();
        policy["forbidden"] = json!([{"source":"core","target":"io","kind":"Runtime"}]);
        evaluator
            .evaluate(Evaluate {
                snapshot,
                policy: wire::decode_policy(&serde_json::to_vec(&policy).unwrap()).unwrap(),
            })
            .unwrap();
        let report = evaluator.take_response().unwrap().report;
        assert_eq!(exit_code(&report), 1);
        let report = wire::report_value(report).unwrap();
        assert_eq!(report["verdict"], "Fail");
        assert_eq!(report["findings"][0]["edge_id"], "e1");
    }
}
