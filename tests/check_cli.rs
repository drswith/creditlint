use std::fs;

use assert_cmd::Command;
use serde_json::Value;
use tempfile::tempdir;

fn make_repo() -> tempfile::TempDir {
    let temp_dir = tempdir().expect("tempdir");
    fs::create_dir(temp_dir.path().join(".git")).expect("git dir");
    temp_dir
}

#[test]
fn check_message_file_reports_violation_with_exit_code_one() {
    let repo = make_repo();
    let message_path = repo.path().join("message.txt");
    fs::write(&message_path, "Co-authored-by: Codex <codex@example.com>\n").expect("message");

    let output = Command::cargo_bin("creditlint")
        .expect("binary")
        .current_dir(repo.path())
        .args(["check", "--message-file"])
        .arg(&message_path)
        .output()
        .expect("run command");

    assert_eq!(output.status.code(), Some(1));
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("forbidden-ai-coauthor"),
        "stdout should contain the rule id"
    );
}

#[test]
fn check_stdin_supports_json_output() {
    let repo = make_repo();

    let output = Command::cargo_bin("creditlint")
        .expect("binary")
        .current_dir(repo.path())
        .args(["check", "--stdin", "--format", "json"])
        .write_stdin("Made with Cursor\n")
        .output()
        .expect("run command");

    assert_eq!(output.status.code(), Some(1));

    let json: Value = serde_json::from_slice(&output.stdout).expect("json output");
    assert_eq!(json["ok"], Value::Bool(false));
    assert_eq!(
        json["violations"][0]["rule_id"],
        "forbidden-made-with-marker"
    );
    assert_eq!(json["violations"][0]["source"]["kind"], "Stdin");
}

#[test]
fn clean_input_returns_exit_code_zero() {
    let repo = make_repo();

    let output = Command::cargo_bin("creditlint")
        .expect("binary")
        .current_dir(repo.path())
        .args(["check", "--stdin"])
        .write_stdin("Co-authored-by: Jane Doe <jane@example.com>\n")
        .output()
        .expect("run command");

    assert_eq!(output.status.code(), Some(0));
    assert!(output.stdout.is_empty());
    assert!(output.stderr.is_empty());
}

#[test]
fn invalid_config_returns_exit_code_two() {
    let repo = make_repo();
    let message_path = repo.path().join("message.txt");
    fs::write(
        &message_path,
        "Co-authored-by: Jane Doe <jane@example.com>\n",
    )
    .expect("message");
    fs::write(
        repo.path().join(".creditlint.yml"),
        "version: 1\nrules:\n  forbidden_trailers:\n    - key: [\n",
    )
    .expect("config");

    let output = Command::cargo_bin("creditlint")
        .expect("binary")
        .current_dir(repo.path())
        .args(["check", "--message-file"])
        .arg(&message_path)
        .output()
        .expect("run command");

    assert_eq!(output.status.code(), Some(2));
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("failed to load policy"),
        "stderr should explain the config failure"
    );
}
