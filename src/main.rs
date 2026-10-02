use clap::{Parser, Subcommand};
use codegate::{
    Evaluator, exit_code,
    model::{Evaluate, obligations::EvaluateBehavior},
    wire,
};
use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::ExitCode,
};
#[derive(Parser)]
#[command(
    name = "codegate",
    version,
    about = "Evaluate normalized dependency facts against a language-neutral policy"
)]
struct Cli {
    #[command(subcommand)]
    command: Action,
}
#[derive(Subcommand)]
enum Action {
    Evaluate {
        #[arg(long)]
        facts: PathBuf,
        #[arg(long)]
        policy: PathBuf,
        #[arg(long)]
        out: Option<PathBuf>,
    },
}
fn read_bounded(path: &Path) -> Result<Vec<u8>, String> {
    let mut bytes = vec![];
    File::open(path)
        .map_err(|e| format!("{}: {e}", path.display()))?
        .take(wire::MAX_DOCUMENT_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    if bytes.len() > wire::MAX_DOCUMENT_BYTES {
        return Err(format!("{}: document exceeds 4 MiB limit", path.display()));
    }
    Ok(bytes)
}
fn run(cli: Cli) -> Result<u8, String> {
    let Action::Evaluate { facts, policy, out } = cli.command;
    let input = Evaluate {
        snapshot: wire::decode_snapshot(&read_bounded(&facts)?)?,
        policy: wire::decode_policy(&read_bounded(&policy)?)?,
    };
    let mut evaluator = Evaluator::default();
    evaluator.evaluate(input).map_err(|e| format!("{e:?}"))?;
    let report = evaluator
        .take_response()
        .ok_or("evaluator returned no response")?
        .report;
    let status = exit_code(&report);
    let mut bytes =
        serde_json::to_vec_pretty(&wire::report_value(report)?).map_err(|e| e.to_string())?;
    bytes.push(b'\n');
    match out {
        Some(path) => {
            std::fs::write(&path, bytes).map_err(|e| format!("{}: {e}", path.display()))?
        }
        None => std::io::stdout()
            .lock()
            .write_all(&bytes)
            .map_err(|e| e.to_string())?,
    }
    Ok(status)
}
fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(code) => ExitCode::from(code),
        Err(error) => {
            eprintln!("codegate: {error}");
            ExitCode::from(2)
        }
    }
}
