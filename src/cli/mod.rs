use std::fs;
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
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
struct CheckArgs {
    #[arg(long)]
    message_file: PathBuf,
}

pub fn run() -> Result<(), CliError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check(args) => run_check(args),
    }
}

fn run_check(args: CheckArgs) -> Result<(), CliError> {
    let policy = load_policy_from_current_dir().map_err(CliError::Config)?;
    let content =
        fs::read_to_string(&args.message_file).map_err(|source| CliError::ReadMessage {
            path: args.message_file.clone(),
            source,
        })?;
    let source = Source {
        kind: SourceKind::MessageFile,
        path: Some(args.message_file),
        commit_sha: None,
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
    #[error("failed to load policy")]
    Config(#[source] ConfigError),
    #[error("failed to read message file at {path}")]
    ReadMessage {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to analyze message")]
    AnalyzeMessage(#[source] AnalysisError),
}
