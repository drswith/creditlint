# creditlint

`creditlint` is a policy-focused CLI for detecting unauthorized authorship and
credit metadata in Git workflows.

It is designed for projects that allow AI-assisted development but do not want
coding agents, bots, or tools to be silently added as authors through commit
trailers or merge messages.

## Status

This repository has moved from bootstrap implementation into delivery
preparation.

Active change:

- `bootstrap-creditlint-mvp`
- `add-npm-wrapper-package`

Current implementation target:

- Implementation stack: Rust native CLI
- Build/package manager: Cargo
- Optional npm workspace/package manager: pnpm
- Rust toolchain: stable with rustfmt and clippy
- Task runner: just
- Test runner: cargo-nextest
- OpenSpec command runner: pnpm
- Primary interface: `creditlint`
- Delivery path: GitHub Actions release artifacts and GitHub Releases

## Problem

Coding agents can add markers such as:

```text
Co-authored-by: Codex <...>
Made with Cursor
Generated with Claude
```

These markers can create authorship, contribution-credit, and audit risks when
they are added without explicit maintainer approval.

`creditlint` treats authorship and provenance as separate concepts:

- Authorship markers such as `Co-authored-by` affect contribution credit.
- Provenance markers such as `AI-Assisted` or `Tool-Used` can disclose process
  without implying authorship.

## CLI

```sh
creditlint check --message-file .git/COMMIT_EDITMSG
creditlint check --stdin
creditlint check --range origin/main..HEAD
creditlint audit --all
creditlint init
creditlint install-hook
creditlint github ruleset-pattern
```

Exit codes:

- `0`: no violations
- `1`: policy violations found
- `2`: invalid invocation, invalid config, or missing required metadata

## Policy File

```yaml
version: 1

rules:
  forbidden_identities:
    - name_pattern: "(?i)(cursor agent|codex|claude|copilot|openai|anthropic|gemini)"
      email_pattern: "(?i)(cursoragent@cursor\\.com|codex|claude|copilot|openai|anthropic|gemini)"

  forbidden_trailers:
    - key: Co-authored-by
      value_pattern: "(?i)(codex|claude|cursor|copilot|openai|anthropic|gemini|ai)"
    - key_pattern: "(?i)^made[- ]with\\b.*$"
    - key_pattern: "(?i)^made[- ]on\\b.*$"
    - key_pattern: "(?i)^generated[- ]with\\b.*$"

  allowed_provenance_trailers:
    - AI-Assisted
    - Tool-Used
    - Generated-by
```

## Governance Model

`creditlint` is intended to run in multiple places:

- Local `commit-msg` hook for fast feedback.
- CI required check for pull-request commits.
- GitHub ruleset metadata restrictions for final protected-branch commit
  messages, especially when squash merge remains enabled.
- Merge-bot validation for controlled final merge messages.
- Pull request title/body checks by writing the PR text to a file and running
  `creditlint check --message-file`.

CI range checks are useful, but they do not by themselves guarantee validation
of a final squash merge message edited by the hosting platform UI.

`check --range` and `audit --all` validate both Git identity metadata
(`author`/`committer` name and email) and commit messages. This covers commits
whose rendered log output shows identities such as
`Author: Cursor Agent <cursoragent@cursor.com>` even when the commit message
itself is clean.

## GitHub Actions

For repository-local CI, the checked-in workflow builds and validates the Rust
source tree directly.

`fetch-depth: 0` is required for `check --range` because shallow history can
remove the base commits needed to resolve the range.

```yaml
name: creditlint

on:
  pull_request:
  push:
    branches:
      - main

jobs:
  creditlint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - uses: dtolnay/rust-toolchain@stable

      - name: Build creditlint
        run: cargo build --release

      - name: Check pull request commit messages
        if: github.event_name == 'pull_request'
        run: |
          ./target/release/creditlint check \
            --range origin/${{ github.base_ref }}..HEAD

      - name: Audit full history on main
        if: github.event_name == 'push'
        run: ./target/release/creditlint audit --all
```

The release workflow produces native binaries for Linux, macOS, and Windows as
workflow artifacts on manual runs and as GitHub Release assets for version tags.
It also generates a combined `SHA256SUMS` artifact and publishes it with tagged
GitHub Releases. Release jobs run formatting, clippy, tests, and OpenSpec
validation before building publishable artifacts.

For crates.io publishing, the release workflow uses:

- job-scoped `permissions: contents: write` for GitHub Release asset publishing
- repository secret `CARGO_REGISTRY_TOKEN` for `cargo publish`

Tag pushes matching `v*` publish native assets and then publish the crate to
crates.io. Manual `workflow_dispatch` runs build artifacts by default and can
opt into crates.io publishing with the `publish_crate` input.

The CI workflow also runs workflow linting for `.github/workflows/*.yml` and
validates the optional npm wrapper package and OpenSpec artifacts.

## npm Wrapper

The npm package is optional. It exists for teams that already install developer
tools through npm, pnpm, or npx. Normal npm consumers should not need Rust or
Cargo; the `creditlint` package resolves a platform-specific optional package
that contains the native Rust binary.

The JavaScript code remains a thin wrapper. It does not reimplement policy logic
or Git metadata checks.

Install from npm once packages are published:

```sh
pnpm add -D creditlint
pnpm exec creditlint --help
```

Local development uses the pnpm workspace:

```sh
pnpm install
cargo build
CREDITLINT_BIN="$PWD/target/debug/creditlint" pnpm --filter creditlint run creditlint --help
pnpm --filter creditlint test
```

Resolution order:

1. `CREDITLINT_BIN`
2. installed platform package binary, such as `creditlint-darwin-arm64`
3. package-local `packages/creditlint/native/`
4. repository-local Cargo outputs under `target/release/` and `target/debug/`

Do not publish the main npm package as a user-facing release until the matching
platform packages have staged native binaries.

## Local Hooks

Initialize a repository policy file:

```sh
creditlint init
```

Install the managed `commit-msg` hook:

```sh
creditlint install-hook
```

The installed hook runs:

```sh
creditlint check --message-file "$1"
```

`creditlint install-hook` only replaces hooks that already carry the stable
`creditlint managed hook` marker. If a repository already has an unmanaged
`commit-msg` hook, `creditlint` refuses to overwrite it.

For manual integration into an existing hook, add this line to the hook script:

```sh
creditlint check --message-file "$1"
```

If your team uses the Python `pre-commit` framework, run the same command from a
local hook entry and pass the commit message file path through the hook config.

## Pull Request Title And Body Checks

Pull request text is a separate input surface from commit messages. This matters
most when a hosting platform uses the pull request title or body while building
a final squash merge commit message.

In CI, write the pull request title and body into a temporary file and lint that
file with the same policy engine:

```sh
printf '%s\n\n%s\n' "$PR_TITLE" "$PR_BODY" > /tmp/creditlint-pr-message.txt
creditlint check --message-file /tmp/creditlint-pr-message.txt
```

For GitHub Actions, the title and body can be read from the pull request event
payload and passed through the same file-based check. `check --range` and
`check --message-file` are complementary; range checks validate commits, while
the temporary file path validates PR text that may later influence squash merge
message generation.

## GitHub Squash Merge Rulesets

If GitHub squash merge remains enabled, use repository ruleset metadata
restrictions to validate the final squash commit message that GitHub is about to
write.

Export a conservative ruleset regex from the active policy:

```sh
creditlint github ruleset-pattern
```

In GitHub branch or repository rulesets, use the exported pattern with a
commit-message restriction equivalent to:

- commit message must not match regex

This ruleset path is stronger than CI for the final squash commit because
GitHub evaluates the generated merge message at the protected-branch boundary.

`creditlint check --range` is still useful for pull request commit messages, but
it does not by itself validate a final squash message edited in the GitHub UI.

## Merge Bot Validation

For repositories that use a controlled merge bot, validate the final merge
message immediately before the bot performs the protected-branch write:

```sh
creditlint check --message-file final-merge-message.txt
```

This path is appropriate when:

- the active policy cannot be represented safely as one GitHub ruleset regex
- the repository wants one final validation step after PR checks
- the merge system already materializes the exact final commit message

The merge bot should fail closed when `creditlint` exits with `1` or `2`.

## Ruleset Export Boundary

`creditlint github ruleset-pattern` intentionally supports only a conservative
subset of policy behavior.

Currently representable as one GitHub ruleset regex:

- forbidden trailer rules with an exact trailer key
- trailer value matchers that are exact strings, unanchored regexes, or `Any`
- free-form marker rules expressed as one anchored line regex such as
  `(?i)^made[- ]with\\b.*$`
- policies where allowed provenance keys do not overlap any forbidden trailer
  key

Not representable safely as one GitHub ruleset regex:

- Git author or committer identity rules; enforce those with `creditlint
  check --range`, `creditlint audit --all`, or platform identity restrictions
- policies that need forbidden/allowed precedence on the same trailer key
- forbidden trailer rules that depend on regex-matched trailer field names
- free-form rules that are not a single anchored line regex
- policies that would require normalization or logic beyond one regex pass

When the active message-policy subset falls outside the safe subset,
`creditlint github ruleset-pattern` fails closed and points the repository to
merge-bot validation for final squash messages.

## Privacy

The planned CLI is local-first. By default, `creditlint` should not upload commit
messages, pull request text, or policy files to any hosted service.

Current default boundary:

- `creditlint` reads local message text, Git metadata, and repository config.
- `creditlint` does not send commit messages or pull request text to a remote
  API.
- `creditlint` does not require a hosted account or background service.
- Network access is not part of the default policy-evaluation path.

If a future change introduces optional hosted integrations, that behavior should
be documented separately instead of being folded into the default CLI contract.

## Threat Model

The MVP is designed to catch:

- Tools that automatically append authorship-like markers.
- Contributors who accidentally paste AI/tool credit markers into commit or
  pull request text.
- Cloud-agent and CI paths that bypass local developer hooks.
- Platform merge paths where the final protected-branch message can differ from
  checked commits.

Current out-of-scope evasions:

- Unicode homoglyph spoofing such as visually similar non-ASCII characters.
- Deliberately split or obfuscated markers intended to bypass simple line-based
  detection.
- Administrator bypass of repository rules.
- Direct protected-branch writes outside the enforced workflow.

## Performance

The current Git collection path is intended to stream `git log` output
record-by-record instead of first loading the full raw log into memory.

Current budget:

- Memory growth should be bounded by the current record being parsed plus the
  accumulated violation list, not the full raw `git log` output.
- `audit --all` should remain practical on normal repository histories without
  requiring the full commit-message stream to be buffered at once.

## Development

Use Cargo for implementation work. The OpenSpec CLI is currently invoked through
`pnpm dlx`, but consuming projects should not need Node.js or pnpm to run
`creditlint`.

Planned Rust tooling:

- `rust-toolchain.toml` pins stable Rust with `rustfmt` and `clippy`.
- `just` provides short project commands.
- `cargo-nextest` is the preferred test runner.
- `cargo-watch` is optional for local edit/test loops.
- `cross` is reserved for release builds.

Common commands:

```sh
just check
just fmt
just lint
just test
just ci
just release-build
just cross-build x86_64-unknown-linux-gnu
```

Local prerequisites for the planned Rust workflow:

```sh
cargo install just
cargo install cargo-nextest
cargo install cross
```

OpenSpec commands:

```sh
pnpm dlx @fission-ai/openspec list
pnpm dlx @fission-ai/openspec validate --all
pnpm dlx @fission-ai/openspec show bootstrap-creditlint-mvp
```

Implementation work should follow:

```text
openspec/changes/bootstrap-creditlint-mvp/tasks.md
```

## Packaging

`creditlint` now uses a delivery-oriented native packaging path through:

- crates.io for the `creditlint` crate
- GitHub Actions workflow artifacts for manual release runs
- GitHub Releases for tagged prebuilt binaries, `SHA256SUMS`, and release notes

The published metadata points back to this repository and the future `docs.rs`
documentation page for the crate.

For cross-platform release artifacts, use `cross` through the checked-in
`just` recipes:

```sh
just cross-build x86_64-unknown-linux-gnu
just cross-build x86_64-pc-windows-msvc
```

For consumers who do not want a Rust toolchain, prefer downloading the matching
native binary artifact from GitHub Releases and verifying it against
`SHA256SUMS`.

For maintainers, the crates.io publish path requires creating a crates.io API
token and storing it in the repository as the `CARGO_REGISTRY_TOKEN` Actions
secret.

## Versioning

The project intends to follow SemVer after the first public release.

Before that release:

- keep ongoing work in `CHANGELOG.md` under `Unreleased`
- bump `Cargo.toml` and cut a dated changelog heading in the same release change
- treat CLI flags, config schema, exit codes, and JSON output as versioned user
  contracts

## License

MIT
