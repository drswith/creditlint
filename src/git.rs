use std::process::Command;

use thiserror::Error;

const FIELD_SEPARATOR: char = '\u{001f}';
const RECORD_SEPARATOR: char = '\u{001e}';

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitMessage {
    pub sha: String,
    pub message: String,
}

pub fn collect_range_messages(range: &str) -> Result<Vec<CommitMessage>, GitError> {
    collect_git_messages(["log", "--format=%H%x1f%B%x1e", range], range)
}

pub fn collect_all_messages() -> Result<Vec<CommitMessage>, GitError> {
    collect_git_messages(["log", "--format=%H%x1f%B%x1e", "--all"], "--all")
}

fn collect_git_messages<const N: usize>(
    args: [&str; N],
    scope: &str,
) -> Result<Vec<CommitMessage>, GitError> {
    let output = Command::new("git")
        .args(args)
        .output()
        .map_err(GitError::Spawn)?;

    if !output.status.success() {
        return Err(GitError::CommandFailed {
            range: scope.to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).trim().to_string(),
        });
    }

    let stdout = String::from_utf8(output.stdout).map_err(GitError::InvalidUtf8)?;
    Ok(parse_git_log_output(&stdout))
}

fn parse_git_log_output(stdout: &str) -> Vec<CommitMessage> {
    stdout
        .split(RECORD_SEPARATOR)
        .filter_map(|record| {
            let trimmed = record.trim_matches('\n');
            if trimmed.is_empty() {
                return None;
            }

            let (sha, message) = trimmed.split_once(FIELD_SEPARATOR)?;
            Some(CommitMessage {
                sha: sha.to_string(),
                message: message.trim_end_matches('\n').to_string(),
            })
        })
        .collect()
}

#[derive(Debug, Error)]
pub enum GitError {
    #[error("failed to execute git")]
    Spawn(#[source] std::io::Error),
    #[error("git log failed for range `{range}`: {stderr}")]
    CommandFailed { range: String, stderr: String },
    #[error("git output was not valid UTF-8")]
    InvalidUtf8(#[source] std::string::FromUtf8Error),
}

#[cfg(test)]
mod tests {
    use super::parse_git_log_output;

    #[test]
    fn parses_git_log_output_with_record_separator() {
        let parsed = parse_git_log_output(
            "abc123\u{001f}subject line\nbody line\u{001e}def456\u{001f}second subject\u{001e}",
        );

        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].sha, "abc123");
        assert_eq!(parsed[0].message, "subject line\nbody line");
        assert_eq!(parsed[1].sha, "def456");
        assert_eq!(parsed[1].message, "second subject");
    }
}
