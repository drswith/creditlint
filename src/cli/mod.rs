use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

use clap::{ArgGroup, Args, Parser, Subcommand};
use thiserror::Error;

use crate::config::{ConfigError, load_policy_from_current_dir};
use crate::policy::{AnalysisError, Source, SourceKind, Violation};

#[derive(Debug, Parser)]
#[command(name = "creditlint")]
#[command(about = "CLI for enforcing Git credit and authorship metadata policy")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Check(CheckArgs),
}

#[derive(Debug, Args)]
#[command(group(
    ArgGroup::new("input")
        .required(true)
        .args(["message_file", "stdin"])
))]
struct CheckArgs {
    #[arg(long)]
    message_file: Option<PathBuf>,
    #[arg(long)]
    stdin: bool,
}

pub fn run() -> Result<(), CliError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check(args) => run_check(args),
    }
}

fn run_check(args: CheckArgs) -> Result<(), CliError> {
    let policy = load_policy_from_current_dir().map_err(CliError::Config)?;
    let (content, source) = match (args.message_file, args.stdin) {
        (Some(path), false) => {
            let content = fs::read_to_string(&path).map_err(|source| CliError::ReadMessage {
                path: path.clone(),
                source,
            })?;
            let source = Source {
                kind: SourceKind::MessageFile,
                path: Some(path),
                commit_sha: None,
            };
            (content, source)
        }
        (None, true) => {
            let mut content = String::new();
            io::stdin()
                .read_to_string(&mut content)
                .map_err(CliError::ReadStdin)?;
            let source = Source {
                kind: SourceKind::Stdin,
                path: None,
                commit_sha: None,
            };
            (content, source)
        }
        _ => return Err(CliError::InvalidInputSelection),
    };

    let violations = policy
        .analyze(source, &content)
        .map_err(CliError::AnalyzeMessage)?;

    if violations.is_empty() {
        return Ok(());
    }

    print_human_violations(&violations);
    Err(CliError::PolicyViolation)
}

fn print_human_violations(violations: &[Violation]) {
    println!("creditlint found {} violation(s)", violations.len());

    for violation in violations {
        println!("rule: {}", violation.rule_id);
        if let Some(field) = &violation.field {
            println!("field: {field}");
        }
        if let Some(line) = violation.line {
            println!("line: {line}");
        }
        println!("message: {}", violation.message);
    }
}

#[derive(Debug, Error)]
pub enum CliError {
    #[error("policy violations found")]
    PolicyViolation,
    #[error("exactly one input source must be selected")]
    InvalidInputSelection,
    #[error("failed to load policy")]
    Config(#[source] ConfigError),
    #[error("failed to read message file at {path}")]
    ReadMessage {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to read message text from stdin")]
    ReadStdin(#[source] std::io::Error),
    #[error("failed to analyze message")]
    AnalyzeMessage(#[source] AnalysisError),
}
