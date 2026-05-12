use clap::ValueEnum;
use serde::Serialize;

use crate::policy::Violation;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    Human,
    Json,
}

pub fn render_violations(
    format: OutputFormat,
    violations: &[Violation],
) -> Result<String, serde_json::Error> {
    match format {
        OutputFormat::Human => Ok(render_human(violations)),
        OutputFormat::Json => render_json(violations),
    }
}

fn render_human(violations: &[Violation]) -> String {
    if violations.is_empty() {
        return String::new();
    }

    let mut lines = Vec::new();
    lines.push(format!(
        "creditlint found {} violation(s)",
        violations.len()
    ));

    for violation in violations {
        lines.push(format!("rule: {}", violation.rule_id));
        if let Some(commit_sha) = &violation.source.commit_sha {
            lines.push(format!("commit: {commit_sha}"));
        }
        if let Some(field) = &violation.field {
            lines.push(format!("field: {field}"));
        }
        if let Some(line) = violation.line {
            lines.push(format!("line: {line}"));
        }
        lines.push(format!("message: {}", violation.message));
    }

    lines.join("\n")
}

fn render_json(violations: &[Violation]) -> Result<String, serde_json::Error> {
    let report = ViolationReport {
        ok: violations.is_empty(),
        violations,
    };

    serde_json::to_string_pretty(&report)
}

#[derive(Debug, Serialize)]
struct ViolationReport<'a> {
    ok: bool,
    violations: &'a [Violation],
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{OutputFormat, render_violations};
    use crate::policy::{Source, SourceKind, Violation};

    #[test]
    fn human_output_includes_commit_sha_when_present() {
        let report = render_violations(
            OutputFormat::Human,
            &[Violation {
                source: Source {
                    kind: SourceKind::Commit,
                    path: Some(PathBuf::from(".git/COMMIT_EDITMSG")),
                    commit_sha: Some("abc123".to_string()),
                },
                rule_id: "forbidden-ai-coauthor".to_string(),
                field: Some("Co-authored-by".to_string()),
                line: Some(2),
                message: "AI/tool authorship marker is not allowed".to_string(),
            }],
        )
        .expect("render report");

        assert!(report.contains("commit: abc123"));
    }
}
