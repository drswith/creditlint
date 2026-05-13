## Context

`creditlint` already has a CI workflow and a release workflow. CI validates Rust
formatting, linting, tests, release build, and a binary smoke test. Release
builds native artifacts for Linux, macOS, and Windows and can publish crates.io.

The release workflow should not rely on prior CI having passed because tagged
releases and manual dispatches are independent entry points. It should validate
the repository state before building and publishing assets.

## Goals / Non-Goals

**Goals:**

- Ensure release runs execute the same core validation as CI.
- Publish checksum material alongside native release artifacts.
- Keep workflow permissions least-privilege by default.
- Make macOS artifact architecture expectations explicit.
- Add workflow syntax/static linting to CI.

**Non-Goals:**

- Replacing GitHub-hosted runners with self-hosted runners.
- Adding signing/notarization for binaries.
- Changing `creditlint` runtime behavior.
- Implementing package-manager installers such as Homebrew in this change.

## Decisions

### Add release validation before artifact publishing

Release should run formatting, clippy, tests, and OpenSpec validation before
building publishable artifacts.

Rationale:

- Tag and manual release workflows can be invoked independently of PR CI.
- A release should fail before publishing when source validation fails.

### Generate one combined SHA256SUMS artifact

The release workflow should download all native build artifacts into one job,
generate a combined `SHA256SUMS` file, upload it as a workflow artifact, and add
it to GitHub Releases for tag runs.

Rationale:

- A single checksum file is the familiar distribution shape for CLI binaries.
- Generating it after all matrix artifacts finish avoids per-platform naming
  drift.

### Keep macOS runner architecture explicit

Use explicit macOS labels for arm64 and Intel runners, and verify Unix binary
architecture with `file` during release builds.

Rationale:

- `macos-latest` can move over time.
- Release asset names should correspond to the architecture actually built.

## Risks / Trade-offs

- [Risk] Workflow linting introduces a third-party action. -> Use a widely used
  actionlint action and keep it isolated in CI.
- [Risk] OpenSpec validation requires pnpm in workflows. -> Use pnpm only for
  repository validation, not as a `creditlint` runtime dependency.
- [Risk] macOS runner labels can change over time. -> Use explicit labels based
  on current GitHub-hosted runner documentation and keep architecture checks in
  the workflow.
