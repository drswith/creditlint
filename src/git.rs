use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use thiserror::Error;

const FIELD_SEPARATOR: char = '\u{001f}';
const RECORD_SEPARATOR: char = '\u{001e}';

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitRecord {
    pub sha: String,
    pub author_name: String,
    pub author_email: String,
    pub committer_name: String,
    pub committer_email: String,
    pub message: String,
}

pub fn collect_range_messages(range: &str) -> Result<Vec<CommitRecord>, GitError> {
    collect_git_messages(
        [
            "log",
            "--format=%H%x1f%an%x1f%ae%x1f%cn%x1f%ce%x1f%B%x1e",
            range,
        ],
        range,
    )
}

pub fn collect_all_messages() -> Result<Vec<CommitRecord>, GitError> {
    collect_git_messages(
        [
            "log",
            "--format=%H%x1f%an%x1f%ae%x1f%cn%x1f%ce%x1f%B%x1e",
            "--all",
        ],
        "--all",
    )
}

pub fn commit_msg_hook_path() -> Result<PathBuf, GitError> {
    let output = Command::new("git")
        .args(["rev-parse", "--git-path", "hooks/commit-msg"])
        .output()
        .map_err(GitError::Spawn)?;

    if !output.status.success() {
        return Err(GitError::CommandFailed {
            range: "hooks/commit-msg".to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).trim().to_string(),
        });
    }

    Ok(PathBuf::from(
        String::from_utf8(output.stdout)
            .map_err(GitError::InvalidUtf8)?
            .trim(),
    ))
}

fn collect_git_messages<const N: usize>(
    args: [&str; N],
    scope: &str,
) -> Result<Vec<CommitRecord>, GitError> {
    let mut child = Command::new("git")
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(GitError::Spawn)?;

    let stdout = child.stdout.take().ok_or(GitError::MissingStdout)?;
    let messages = parse_git_log_stream(BufReader::new(stdout))?;
    let status = child.wait().map_err(GitError::Wait)?;

    let mut stderr = String::new();
    if let Some(mut stderr_pipe) = child.stderr.take() {
        stderr_pipe
            .read_to_string(&mut stderr)
            .map_err(GitError::ReadStderr)?;
    }

    if !status.success() {
        return Err(GitError::CommandFailed {
            range: scope.to_string(),
            stderr: stderr.trim().to_string(),
        });
    }

    Ok(messages)
}

#[cfg(test)]
fn parse_git_log_output(stdout: &str) -> Vec<CommitRecord> {
    parse_git_log_records(stdout.split(RECORD_SEPARATOR).map(str::to_string))
}

fn parse_git_log_stream<R: BufRead>(reader: R) -> Result<Vec<CommitRecord>, GitError> {
    let records = reader
        .split(RECORD_SEPARATOR as u8)
        .map(|chunk| {
            let bytes = chunk.map_err(GitError::ReadStdout)?;
            String::from_utf8(bytes).map_err(GitError::InvalidUtf8)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(parse_git_log_records(records))
}

fn parse_git_log_records<I>(records: I) -> Vec<CommitRecord>
where
    I: IntoIterator<Item = String>,
{
    records
        .into_iter()
        .filter_map(|record| {
            let trimmed = record.trim_matches('\n');
            if trimmed.is_empty() {
                return None;
            }

            let mut fields = trimmed.splitn(6, FIELD_SEPARATOR);
            let sha = fields.next()?;
            let author_name = fields.next()?;
            let author_email = fields.next()?;
            let committer_name = fields.next()?;
            let committer_email = fields.next()?;
            let message = fields.next()?;

            Some(CommitRecord {
                sha: sha.to_string(),
                author_name: author_name.to_string(),
                author_email: author_email.to_string(),
                committer_name: committer_name.to_string(),
                committer_email: committer_email.to_string(),
                message: message.trim_end_matches('\n').to_string(),
            })
        })
        .collect()
}

#[derive(Debug, Error)]
pub enum GitError {
    #[error("failed to execute git")]
    Spawn(#[source] std::io::Error),
    #[error("failed while waiting for git to finish")]
    Wait(#[source] std::io::Error),
    #[error("git process did not expose stdout")]
    MissingStdout,
    #[error("git log failed for range `{range}`: {stderr}")]
    CommandFailed { range: String, stderr: String },
    #[error("failed to read git stdout")]
    ReadStdout(#[source] std::io::Error),
    #[error("failed to read git stderr")]
    ReadStderr(#[source] std::io::Error),
    #[error("git output was not valid UTF-8")]
    InvalidUtf8(#[source] std::string::FromUtf8Error),
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::{parse_git_log_output, parse_git_log_stream};

    #[test]
    fn parses_git_log_output_with_record_separator() {
        let parsed = parse_git_log_output(
            "abc123\u{001f}Jane Doe\u{001f}jane@example.com\u{001f}Creditlint Test\u{001f}creditlint@example.com\u{001f}subject line\nbody line\u{001e}def456\u{001f}John Doe\u{001f}john@example.com\u{001f}Creditlint Test\u{001f}creditlint@example.com\u{001f}second subject\u{001e}",
        );

        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].sha, "abc123");
        assert_eq!(parsed[0].author_name, "Jane Doe");
        assert_eq!(parsed[0].author_email, "jane@example.com");
        assert_eq!(parsed[0].committer_name, "Creditlint Test");
        assert_eq!(parsed[0].committer_email, "creditlint@example.com");
        assert_eq!(parsed[0].message, "subject line\nbody line");
        assert_eq!(parsed[1].sha, "def456");
        assert_eq!(parsed[1].message, "second subject");
    }

    #[test]
    fn parses_git_log_stream_incrementally() {
        let parsed = parse_git_log_stream(Cursor::new(
            b"abc123\x1fJane Doe\x1fjane@example.com\x1fCreditlint Test\x1fcreditlint@example.com\x1fsubject line\nbody line\x1edef456\x1fJohn Doe\x1fjohn@example.com\x1fCreditlint Test\x1fcreditlint@example.com\x1fsecond subject\x1e".to_vec(),
        ))
        .expect("parse stream");

        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].sha, "abc123");
        assert_eq!(parsed[1].sha, "def456");
    }
}
