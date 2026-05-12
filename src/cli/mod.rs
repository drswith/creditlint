use std::fs;
use std::io::{self, Read};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use clap::{ArgGroup, Args, Parser, Subcommand};
use thiserror::Error;

use crate::config::{
    ConfigError, default_config_contents, init_config_path_from_current_dir,
    load_policy_from_current_dir,
};
use crate::git::{
    CommitRecord, GitError, collect_all_messages, collect_range_messages, commit_msg_hook_path,
};
use crate::github::{RulesetExportError, export_ruleset_pattern};
use crate::policy::{AnalysisError, Identity, IdentityRole, Policy, Source, SourceKind, Violation};
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
    Init,
    InstallHook,
    Github(GithubArgs),
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

#[derive(Debug, Args)]
struct GithubArgs {
    #[command(subcommand)]
    command: GithubCommands,
}

#[derive(Debug, Subcommand)]
enum GithubCommands {
    RulesetPattern,
}

pub fn run() -> Result<(), CliError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check(args) => run_check(args),
        Commands::Audit(args) => run_audit(args),
        Commands::Init => run_init(),
        Commands::InstallHook => run_install_hook(),
        Commands::Github(args) => run_github(args),
    }
}

fn run_github(args: GithubArgs) -> Result<(), CliError> {
    match args.command {
        GithubCommands::RulesetPattern => run_github_ruleset_pattern(),
    }
}

fn run_github_ruleset_pattern() -> Result<(), CliError> {
    let policy = load_policy_from_current_dir().map_err(CliError::Config)?;
    let pattern = export_ruleset_pattern(&policy).map_err(CliError::RulesetExport)?;
    println!("{pattern}");
    Ok(())
}

const MANAGED_HOOK_MARKER: &str = "creditlint managed hook";
const MANAGED_HOOK_VERSION: &str = "version: 1";
const COMMIT_MSG_HOOK_CONTENTS: &str = r#"#!/bin/sh
# creditlint managed hook
# version: 1

creditlint check --message-file "$1"
"#;

fn run_init() -> Result<(), CliError> {
    let path = init_config_path_from_current_dir().map_err(CliError::Config)?;

    if path.exists() {
        return Err(CliError::ConfigAlreadyExists { path });
    }

    fs::write(&path, default_config_contents()).map_err(|source| CliError::WriteConfig {
        path: path.clone(),
        source,
    })?;

    println!("created {}", path.display());
    Ok(())
}

fn run_install_hook() -> Result<(), CliError> {
    let path = commit_msg_hook_path().map_err(CliError::Git)?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| CliError::WriteHook {
            path: parent.to_path_buf(),
            source,
        })?;
    }

    if path.exists() {
        let existing = fs::read_to_string(&path).map_err(|source| CliError::ReadHook {
            path: path.clone(),
            source,
        })?;

        if !is_managed_hook(&existing) {
            return Err(CliError::UnmanagedHookExists { path });
        }
    }

    fs::write(&path, COMMIT_MSG_HOOK_CONTENTS).map_err(|source| CliError::WriteHook {
        path: path.clone(),
        source,
    })?;
    set_hook_permissions(&path).map_err(|source| CliError::WriteHook {
        path: path.clone(),
        source,
    })?;

    println!("installed {}", path.display());
    Ok(())
}

fn is_managed_hook(contents: &str) -> bool {
    contents.contains(MANAGED_HOOK_MARKER) && contents.contains(MANAGED_HOOK_VERSION)
}

#[cfg(unix)]
fn set_hook_permissions(path: &PathBuf) -> Result<(), std::io::Error> {
    let mut permissions = fs::metadata(path)?.permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions)
}

#[cfg(not(unix))]
fn set_hook_permissions(_path: &PathBuf) -> Result<(), std::io::Error> {
    Ok(())
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
            .map(|commit| analyze_commit(&policy, SourceKind::Commit, commit))
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
        .map(|commit| analyze_commit(&policy, SourceKind::Audit, commit))
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

fn analyze_commit(
    policy: &Policy,
    kind: SourceKind,
    commit: CommitRecord,
) -> Result<Vec<Violation>, AnalysisError> {
    let source = Source {
        kind,
        path: None,
        commit_sha: Some(commit.sha),
    };

    let mut violations = Vec::new();
    violations.extend(policy.analyze_identity(
        source.clone(),
        &Identity {
            role: IdentityRole::Author,
            name: commit.author_name,
            email: commit.author_email,
        },
    )?);
    violations.extend(policy.analyze_identity(
        source.clone(),
        &Identity {
            role: IdentityRole::Committer,
            name: commit.committer_name,
            email: commit.committer_email,
        },
    )?);
    violations.extend(policy.analyze(source, &commit.message)?);

    Ok(violations)
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
    #[error("config file already exists at {path}")]
    ConfigAlreadyExists { path: PathBuf },
    #[error("failed to write config file at {path}")]
    WriteConfig {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to read existing hook at {path}")]
    ReadHook {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error(
        "existing unmanaged commit-msg hook found at {path}; integrate creditlint manually or remove it first"
    )]
    UnmanagedHookExists { path: PathBuf },
    #[error("failed to install commit-msg hook at {path}")]
    WriteHook {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to export GitHub ruleset pattern: {0}")]
    RulesetExport(#[source] RulesetExportError),
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
            | CliError::RenderReport(_)
            | CliError::ConfigAlreadyExists { .. }
            | CliError::WriteConfig { .. }
            | CliError::ReadHook { .. }
            | CliError::UnmanagedHookExists { .. }
            | CliError::WriteHook { .. }
            | CliError::RulesetExport(_) => 2,
        }
    }
}
