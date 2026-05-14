## Why

The OpenCode review workflows are running, but the current setup still leaves
three avoidable gaps: floating third-party action refs, broader-than-needed
issue-triage permissions, and duplicate review runs on rapid PR updates. These
are workflow-hardening concerns, not product-behavior changes, and they are
small enough to fix directly.

## What Changes

- Pin the `anomalyco/opencode/github` action in both OpenCode workflows to an
  immutable commit SHA instead of `@latest`.
- Add explicit concurrency control to `opencode-review` so stale review runs are
  cancelled when a newer PR update arrives.
- Tighten `opencode-triage` permissions to the least privilege needed for
  checkout and issue comments/reactions.
- Declare `contents: read` explicitly for `opencode-review` because the job
  checks out repository contents before invoking OpenCode.

## Capabilities

### New Capabilities
- `opencode-workflow-hardening`: Hardening requirements for OpenCode-powered
  GitHub Actions workflows, including immutable action refs, least-privilege
  permissions, and duplicate-run control.

### Modified Capabilities
- None.

## Impact

- Updates `.github/workflows/opencode-review.yml`.
- Updates `.github/workflows/opencode-triage.yml`.
- Adds OpenSpec coverage for OpenCode workflow hardening expectations.
