use std::fs;
use std::process::{Command as ProcessCommand, Stdio};

use assert_cmd::Command;
use serde_json::Value;
use tempfile::tempdir;

fn make_repo() -> tempfile::TempDir {
    let temp_dir = tempdir().expect("tempdir");
    fs::create_dir(temp_dir.path().join(".git")).expect("git dir");
    temp_dir
}

fn init_git_repo() -> tempfile::TempDir {
    let temp_dir = tempdir().expect("tempdir");
    run_git(temp_dir.path(), ["init"]);
    run_git(temp_dir.path(), ["config", "user.name", "Creditlint Test"]);
    run_git(
        temp_dir.path(),
        ["config", "user.email", "creditlint@example.com"],
    );
    temp_dir
}

fn run_git<const N: usize>(repo: &std::path::Path, args: [&str; N]) {
    let status = ProcessCommand::new("git")
        .current_dir(repo)
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("run git");

    assert!(status.success(), "git command should succeed");
}

fn write_and_commit(
    repo: &std::path::Path,
    filename: &str,
    contents: &str,
    subject: &str,
    body: Option<&str>,
) {
    fs::write(repo.join(filename), contents).expect("write file");
    run_git(repo, ["add", filename]);

    let mut command = ProcessCommand::new("git");
    command.current_dir(repo).args(["commit", "-m", subject]);
    if let Some(body) = body {
        command.args(["-m", body]);
    }
    command.stdout(Stdio::null()).stderr(Stdio::null());

    let status = command.status().expect("git commit");
    assert!(status.success(), "git commit should succeed");
}

fn head_sha(repo: &std::path::Path) -> String {
    let output = ProcessCommand::new("git")
        .current_dir(repo)
        .args(["rev-parse", "HEAD"])
        .output()
        .expect("rev-parse");

    assert!(output.status.success(), "rev-parse should succeed");
    String::from_utf8(output.stdout)
        .expect("sha utf8")
        .trim()
        .to_string()
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
fn init_writes_default_config_file() {
    let repo = make_repo();

    let output = Command::cargo_bin("creditlint")
        .expect("binary")
        .current_dir(repo.path())
        .arg("init")
        .output()
        .expect("run command");

    assert_eq!(output.status.code(), Some(0));

    let config_path = repo.path().join(".creditlint.yml");
    let config = fs::read_to_string(&config_path).expect("config file");
    assert!(config.contains("version: 1"));
    assert!(config.contains("key: Co-authored-by"));
    assert!(config.contains("allowed_provenance_trailers"));
}

#[test]
fn init_refuses_to_overwrite_existing_config() {
    let repo = make_repo();
    let config_path = repo.path().join(".creditlint.yml");
    fs::write(
        &config_path,
        "version: 1\nrules:\n  forbidden_trailers: []\n",
    )
    .expect("config");

    let output = Command::cargo_bin("creditlint")
        .expect("binary")
        .current_dir(repo.path())
        .arg("init")
        .output()
        .expect("run command");

    assert_eq!(output.status.code(), Some(2));
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("config file already exists"),
        "stderr should explain the existing config"
    );
    assert_eq!(
        fs::read_to_string(&config_path).expect("config file"),
        "version: 1\nrules:\n  forbidden_trailers: []\n"
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

#[test]
fn check_range_clean_commits_returns_zero() {
    let repo = init_git_repo();
    write_and_commit(repo.path(), "first.txt", "first\n", "first commit", None);
    write_and_commit(repo.path(), "second.txt", "second\n", "second commit", None);

    let output = Command::cargo_bin("creditlint")
        .expect("binary")
        .current_dir(repo.path())
        .args(["check", "--range", "HEAD~1..HEAD"])
        .output()
        .expect("run command");

    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn check_range_violating_commit_returns_one_and_includes_sha() {
    let repo = init_git_repo();
    write_and_commit(repo.path(), "first.txt", "first\n", "first commit", None);
    write_and_commit(
        repo.path(),
        "second.txt",
        "second\n",
        "second commit",
        Some("Co-authored-by: Codex <codex@example.com>"),
    );
    let violating_sha = head_sha(repo.path());

    let output = Command::cargo_bin("creditlint")
        .expect("binary")
        .current_dir(repo.path())
        .args(["check", "--range", "HEAD~1..HEAD"])
        .output()
        .expect("run command");

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("forbidden-ai-coauthor"));
    assert!(stdout.contains(&violating_sha));
}

#[test]
fn check_range_invalid_range_returns_two() {
    let repo = init_git_repo();
    write_and_commit(repo.path(), "first.txt", "first\n", "first commit", None);

    let output = Command::cargo_bin("creditlint")
        .expect("binary")
        .current_dir(repo.path())
        .args(["check", "--range", "missing..HEAD"])
        .output()
        .expect("run command");

    assert_eq!(output.status.code(), Some(2));
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("failed to collect commit messages from git"),
        "stderr should explain the git range failure"
    );
}

#[test]
fn audit_all_reports_violations() {
    let repo = init_git_repo();
    write_and_commit(repo.path(), "first.txt", "first\n", "first commit", None);
    write_and_commit(
        repo.path(),
        "second.txt",
        "second\n",
        "second commit",
        Some("Made with Cursor"),
    );

    let output = Command::cargo_bin("creditlint")
        .expect("binary")
        .current_dir(repo.path())
        .args(["audit", "--all", "--format", "json"])
        .output()
        .expect("run command");

    assert_eq!(output.status.code(), Some(1));
    let json: Value = serde_json::from_slice(&output.stdout).expect("json output");
    assert_eq!(json["ok"], Value::Bool(false));
    assert_eq!(
        json["violations"][0]["rule_id"],
        "forbidden-made-with-marker"
    );
}
