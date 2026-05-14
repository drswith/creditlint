[English](README.md) | [简体中文](README.zh-CN.md)

# creditlint

[![CI](https://github.com/Drswith/creditlint/actions/workflows/ci.yml/badge.svg)](https://github.com/Drswith/creditlint/actions/workflows/ci.yml)
[![Downloads](https://img.shields.io/github/downloads/Drswith/creditlint/total.svg)](https://github.com/Drswith/creditlint/releases)
[![Crates.io](https://img.shields.io/crates/d/creditlint.svg?label=crates.io)](https://crates.io/crates/creditlint)
[![npm](https://img.shields.io/npm/dt/creditlint.svg?label=npm)](https://www.npmjs.com/package/creditlint)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-native-orange.svg)](https://www.rust-lang.org/)
[![OpenSpec](https://img.shields.io/badge/OpenSpec-spec--driven-2f6fdd.svg)](openspec/config.yaml)
[![CLI](https://img.shields.io/badge/CLI-creditlint-0f766e.svg)](README.md)

`creditlint` is a Rust-native CLI for enforcing Git credit and authorship
metadata policy before unwanted credit markers land in project history.

It is built for teams that use AI-assisted development but do not want tools,
agents, or hosted workflows to silently add authorship-like metadata such as
`Co-authored-by`, `Made with`, or generated-tool trailers to commits, pull
requests, and merge messages.

`creditlint` does not try to detect whether code was AI-generated, and it is not
a legal compliance engine.

## What It Checks

`creditlint` applies a repository policy to the Git metadata surfaces where
credit can appear:

- commit message text
- pull request title/body text when passed through `--message-file`
- Git author name/email
- Git committer name/email
- final merge or squash messages when a merge bot passes them to the CLI

It is designed to separate two concepts that often get blurred:

- Authorship and credit markers are policy-controlled.
- Provenance markers can be allowed without becoming authorship.

## Why

Coding agents, bots, IDEs, and hosted workflows can write metadata that changes
how authorship or contribution credit appears in Git history:

```text
Co-authored-by: Codex <...>
Author: Cursor Agent <cursoragent@cursor.com>
Made with Cursor
Generated with Claude
```

That metadata can be useful for audit and provenance. The problem is when it is
stored in fields that imply authorship or contribution credit without an
explicit project policy.

`creditlint` is intentionally narrow: it validates metadata placement and
policy, not the origin of the code.

## Current Status

The MVP CLI is implemented. The repository is in delivery preparation for
public package and release channels.

Implemented surfaces:

- `creditlint check --message-file`
- `creditlint check --stdin`
- `creditlint check --range`
- `creditlint audit --all`
- `creditlint init`
- `creditlint install-hook`
- `creditlint github ruleset-pattern`
- Human and JSON output
- Rust native release artifacts
- Optional npm wrapper with platform package resolution

Planned public distribution channels:

- crates.io package metadata for the Rust CLI
- GitHub Release assets for prebuilt native binaries
- optional npm packages that resolve native binaries before local fallbacks

## Install

Until public packages are published, install from this repository:

```sh
cargo install --path .
creditlint --help
```

For local development:

```sh
cargo build
./target/debug/creditlint --help
```

After public package releases are available, consumers should prefer one of:

- the `creditlint` crate from crates.io
- a prebuilt native binary from GitHub Releases
- the optional npm package for teams that already install developer tools
  through npm, pnpm, or npx

The native CLI does not require Node.js, pnpm, or npm in consuming
repositories.

## Quick Start

Create the default policy file in a Git repository:

```sh
creditlint init
```

Install the managed `commit-msg` hook:

```sh
creditlint install-hook
```

Check one message:

```sh
creditlint check --message-file .git/COMMIT_EDITMSG
printf 'Made with Cursor\n' | creditlint check --stdin
```

Check pull-request commits in CI:

```sh
creditlint check --range origin/main..HEAD
```

Audit all reachable Git history:

```sh
creditlint audit --all
```

Generate a conservative GitHub ruleset regex for final squash commit messages:

```sh
creditlint github ruleset-pattern
```

Use JSON output for automation:

```sh
creditlint check --range origin/main..HEAD --format json
```

Exit codes:

- `0`: no violations
- `1`: policy violations found
- `2`: invalid invocation, invalid config, unreadable input, or failed metadata
  collection

## Policy

Without `.creditlint.yml`, `creditlint` uses a built-in default policy. That
default blocks common AI/tool authorship and credit markers while allowing
explicit provenance trailers.

Example policy file:

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

Policy evaluation covers:

- commit message text
- pull request title/body text when passed through `--message-file`
- Git author name/email
- Git committer name/email

Forbidden rules win before allowed provenance keys. Invalid config fails closed
with exit code `2`.

## Enforcement Model

`creditlint` should run at multiple layers because no single hook or CI job sees
every Git metadata surface.

Recommended layers:

- Local `commit-msg` hook for fast feedback.
- CI required check for pull-request commits.
- Pull request title/body check by writing PR text to a temporary file and
  running `creditlint check --message-file`.
- GitHub ruleset metadata restriction for final protected-branch commit
  messages when squash merge remains enabled.
- Merge-bot validation when the repository controls the final merge message.

The boundary is important: `creditlint check --range` validates proposed commits.
It does not by itself validate a final squash merge message edited or synthesized
by the hosting platform UI.

For stronger rollout guidance in another repository, use the repository-local
skill:

```text
skills/enforcement-rollout/SKILL.md
```

Copyable agent prompt:

```text
Use the creditlint repository skill `enforcement-rollout` and help me deploy creditlint in this repository for the strongest practical interception. Check what is already covered, identify the remaining gaps across local hooks, CI commit checks, PR title/body checks, final squash/merge message enforcement, and repository settings, then give me an exact rollout plan.
```

## GitHub Actions

Range checks need enough Git history to resolve the base revision. Use
`fetch-depth: 0` or an equivalent fetch strategy.

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

      - name: Check pull request commits
        if: github.event_name == 'pull_request'
        run: |
          ./target/release/creditlint check \
            --range origin/${{ github.base_ref }}..HEAD

      - name: Audit full history on main
        if: github.event_name == 'push'
        run: ./target/release/creditlint audit --all
```

To validate pull request title/body text, write it to a file and run the same
policy engine:

```sh
printf '%s\n\n%s\n' "$PR_TITLE" "$PR_BODY" > /tmp/creditlint-pr-message.txt
creditlint check --message-file /tmp/creditlint-pr-message.txt
```

## GitHub Rulesets And Merge Bots

Use `creditlint github ruleset-pattern` when the active policy can be safely
represented as one GitHub commit-message restriction regex.

Representable as one ruleset regex:

- forbidden trailer rules with an exact trailer key
- trailer value matchers that are exact strings, unanchored regexes, or `Any`
- free-form marker rules expressed as one anchored line regex
- policies where allowed provenance keys do not overlap forbidden trailer keys

Not representable as one ruleset regex:

- Git author or committer identity rules
- policies that need forbidden/allowed precedence on the same trailer key
- forbidden rules that depend on regex-matched trailer field names
- policy logic that needs normalization or more than one regex pass

When export is unsafe, the command fails closed. Use CI range checks for commit
metadata and a controlled merge-bot validation step for the exact final merge
message:

```sh
creditlint check --message-file final-merge-message.txt
```

## npm Wrapper

The npm package is optional. It is for teams that already install developer
tools through npm, pnpm, or npx.

Normal npm consumers should not need Rust or Cargo. The `creditlint` npm package
delegates to a native binary from a platform-specific optional package such as
`creditlint-darwin-arm64` or `creditlint-linux-x64`.

Install after npm packages are published:

```sh
pnpm add -D creditlint
pnpm exec creditlint --help
```

Resolution order:

1. `CREDITLINT_BIN`
2. installed platform package binary
3. package-local `packages/creditlint/native/`
4. repository-local Cargo outputs under `target/release/` and `target/debug/`

The JavaScript wrapper does not reimplement policy logic or Git metadata
collection.

## Privacy

`creditlint` is local-first.

Default behavior:

- reads local message text, Git metadata, and `.creditlint.yml`
- does not upload commit messages or pull request text
- does not require a hosted account or background service
- does not use network access during policy evaluation

Any future hosted integration should be documented as a separate optional
behavior.

## Threat Model

The MVP is designed to catch:

- tools that append authorship-like markers
- contributors who accidentally paste AI/tool credit markers
- cloud-agent and CI paths that bypass local hooks
- platform merge paths where the final protected-branch message differs from
  checked commits

Current out of scope:

- Unicode homoglyph spoofing
- deliberately split or obfuscated markers
- administrator bypass of repository rules
- direct protected-branch writes outside the enforced workflow

## Development

Use Cargo for Rust implementation work. Use pnpm only for OpenSpec commands and
the optional npm wrapper workspace.

Common commands:

```sh
just check
just fmt
just lint
just test
just test-npm
just openspec-validate
just ci
```

Local tooling:

- stable Rust from `rust-toolchain.toml`
- `just` for project recipes
- `cargo-nextest` for the preferred test runner
- `cross` for release packaging tasks
- pnpm for OpenSpec and npm wrapper validation

OpenSpec commands:

```sh
pnpm dlx @fission-ai/openspec list
pnpm dlx @fission-ai/openspec validate --all
pnpm dlx @fission-ai/openspec status --change bootstrap-creditlint-mvp --json
pnpm dlx @fission-ai/openspec status --change add-npm-wrapper-package --json
```

Implementation work should follow the active OpenSpec change tasks before code
or user-facing behavior changes are made.

## Release And Publishing

Release preparation covers:

- crates.io for the Rust CLI
- GitHub Actions workflow artifacts for manual release runs
- GitHub Releases for tagged native binaries and `SHA256SUMS`
- optional npm platform packages plus the main npm wrapper

Useful maintainer commands:

```sh
just release-build
just cross-build x86_64-unknown-linux-gnu
just cross-build x86_64-pc-windows-msvc
just npm-trust-bootstrap-dry-run
just npm-publish-local-dry-run
just npm-publish-dry-run
```

Do not publish the main npm wrapper as a normal user-facing release until the
matching platform packages have staged native binaries.

For crates.io publishing, the GitHub release workflow expects
`CARGO_REGISTRY_TOKEN` to be configured as a repository secret.

## Versioning

The project intends to follow SemVer after the first public release.

Before that release:

- keep ongoing work in `CHANGELOG.md` under `Unreleased`
- bump package versions and cut a changelog heading in the same release change
- treat CLI flags, config schema, exit codes, and JSON output as versioned user
  contracts

## License

MIT

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=Drswith/creditlint&type=Date)](https://www.star-history.com/#Drswith/creditlint&Date)
