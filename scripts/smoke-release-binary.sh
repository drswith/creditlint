#!/usr/bin/env sh
set -eu

BIN_PATH="${1:-./target/release/creditlint}"
BIN_PATH="$(cd "$(dirname "$BIN_PATH")" && pwd)/$(basename "$BIN_PATH")"

if [ ! -x "$BIN_PATH" ]; then
  echo "release binary is not executable: $BIN_PATH" >&2
  exit 1
fi

TMP_DIR="$(mktemp -d)"
cleanup() {
  rm -rf "$TMP_DIR"
}
trap cleanup EXIT

REPO_DIR="$TMP_DIR/repo"
mkdir -p "$REPO_DIR"
cd "$REPO_DIR"

git init >/dev/null 2>&1
git config user.name "Creditlint Smoke Test"
git config user.email "creditlint-smoke@example.com"

"$BIN_PATH" init >/dev/null

printf 'Reviewed-by: Jane Doe <jane@example.com>\n' | "$BIN_PATH" check --stdin >/dev/null

if printf 'Co-authored-by: Codex <codex@example.com>\n' | "$BIN_PATH" check --stdin >/dev/null 2>&1; then
  echo "expected violating input to fail" >&2
  exit 1
fi

"$BIN_PATH" github ruleset-pattern >/dev/null
