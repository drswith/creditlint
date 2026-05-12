use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

use clap::{ArgGroup, Args, Parser, Subcommand};
use thiserror::Error;

use crate::config::{ConfigError, load_policy_from_current_dir};
use crate::git::{GitError, collect_all_messages, collect_range_messages};
use crate::policy::{AnalysisError, Source, SourceKind};
use crate::reporter::{OutputFormat, render_violations};

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
    Audit(AuditArgs),
}

#[derive(Debug, Args)]
#[command(group(
    ArgGroup::new("input")
        .required(true)
        .args(["message_file", "stdin", "range"])
))]
struct CheckArgs {
    #[arg(long)]
    message_file: Option<PathBuf>,
    #[arg(long)]
    stdin: bool,
    #[arg(long)]
    range: Option<String>,
    #[arg(long, value_enum, default_value_t = OutputFormat::Human)]
    format: OutputFormat,
}

#[derive(Debug, Args)]
struct AuditArgs {
    #[arg(long)]
    all: bool,
    #[arg(long, value_enum, default_value_t = OutputFormat::Human)]
    format: OutputFormat,
}

pub fn run() -> Result<(), CliError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check(args) => run_check(args),
        Commands::Audit(args) => run_audit(args),
    }
}

fn run_check(args: CheckArgs) -> Result<(), CliError> {
    let policy = load_policy_from_current_dir().map_err(CliError::Config)?;
    let format = args.format;
    let violations = match (args.message_file, args.stdin, args.range) {
        (Some(path), false, None) => {
            let content = fs::read_to_string(&path).map_err(|source| CliError::ReadMessage {
                path: path.clone(),
                source,
            })?;
            let source = Source {
                kind: SourceKind::MessageFile,
                path: Some(path),
                commit_sha: None,
            };
            policy
                .analyze(source, &content)
                .map_err(CliError::AnalyzeMessage)?
        }
        (None, true, None) => {
            let mut content = String::new();
            io::stdin()
                .read_to_string(&mut content)
                .map_err(CliError::ReadStdin)?;
            let source = Source {
                kind: SourceKind::Stdin,
                path: None,
                commit_sha: None,
            };
            policy
                .analyze(source, &content)
                .map_err(CliError::AnalyzeMessage)?
        }
        (None, false, Some(range)) => collect_range_messages(&range)
            .map_err(CliError::Git)?
            .into_iter()
            .map(|commit| {
                let source = Source {
                    kind: SourceKind::Commit,
                    path: None,
                    commit_sha: Some(commit.sha),
                };
                policy.analyze(source, &commit.message)
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(CliError::AnalyzeMessage)?
            .into_iter()
            .flatten()
            .collect(),
        _ => return Err(CliError::InvalidInputSelection),
    };

    if violations.is_empty() {
        if format == OutputFormat::Json {
            let output = render_violations(format, &violations).map_err(CliError::RenderReport)?;
            println!("{output}");
        }
        return Ok(());
    }

    let output = render_violations(format, &violations).map_err(CliError::RenderReport)?;
    println!("{output}");
    Err(CliError::PolicyViolation)
}

fn run_audit(args: AuditArgs) -> Result<(), CliError> {
    if !args.all {
        return Err(CliError::InvalidInputSelection);
    }

    let policy = load_policy_from_current_dir().map_err(CliError::Config)?;
    let violations = collect_all_messages()
        .map_err(CliError::Git)?
        .into_iter()
        .map(|commit| {
            let source = Source {
                kind: SourceKind::Audit,
                path: None,
                commit_sha: Some(commit.sha),
            };
            policy.analyze(source, &commit.message)
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(CliError::AnalyzeMessage)?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    if violations.is_empty() {
        if args.format == OutputFormat::Json {
            let output =
                render_violations(args.format, &violations).map_err(CliError::RenderReport)?;
            println!("{output}");
        }
        return Ok(());
    }

    let output = render_violations(args.format, &violations).map_err(CliError::RenderReport)?;
    println!("{output}");
    Err(CliError::PolicyViolation)
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
    #[error("failed to collect commit messages from git")]
    Git(#[source] GitError),
    #[error("failed to render output")]
    RenderReport(#[source] serde_json::Error),
}

impl CliError {
    pub fn exit_code(&self) -> i32 {
        match self {
            CliError::PolicyViolation => 1,
            CliError::InvalidInputSelection
            | CliError::Config(_)
            | CliError::ReadMessage { .. }
            | CliError::ReadStdin(_)
            | CliError::AnalyzeMessage(_)
            | CliError::Git(_)
            | CliError::RenderReport(_) => 2,
        }
    }
}
