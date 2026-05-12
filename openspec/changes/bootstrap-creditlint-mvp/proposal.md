## Why

Coding agents increasingly add authorship-like markers such as
`Co-authored-by`, `Made with`, and `Generated with` without explicit human
approval. This creates contribution-credit, authorship, compliance, and audit
risks, especially when local hooks, cloud agents, CI, and squash merge paths do
not share one enforceable policy.

`creditlint` should start as a small CLI that lets maintainers separate
legitimate tool/provenance disclosure from authorship claims and enforce that
policy in Git workflows.

## What Changes

- Add the first `creditlint` product specification and implementation plan.
- Define a Rust native CLI MVP that does not require Node.js, npm, or pnpm in
  consuming repositories.
- Define default policy behavior for authorship and provenance metadata.
- Define CLI commands for checking raw messages, commit message files, Git
  ranges, and full-history audits.
- Define local hook, CI, GitHub ruleset, and merge-bot integration requirements.
- Establish fail-closed behavior for invalid config, missing required input, and
  failed Git metadata collection.

## Capabilities

### New Capabilities

- `policy-engine`: Credit/authorship policy configuration, default rules, message
  analysis, and structured violation reporting.
- `cli-checks`: User-facing CLI commands, input modes, output formats, and exit
  codes.
- `workflow-integrations`: Local Git hooks, CI usage, GitHub squash-merge
  coverage, and merge-message validation workflows.

### Modified Capabilities

- None. This is the initial OpenSpec change for an empty repository.

## Impact

- Adds OpenSpec project governance for `creditlint`.
- Establishes Rust + Cargo as the initial implementation stack.
- Keeps pnpm only as a convenient runner for OpenSpec commands during
  repository planning.
- Future implementation will add package metadata, source files, tests, README
  usage, and release automation.
- GitHub ruleset support will require documentation because it depends on
  repository/platform capabilities outside the CLI process.
