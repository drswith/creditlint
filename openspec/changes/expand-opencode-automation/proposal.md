## Why

`creditlint` now has a working PR review workflow for OpenCode, but it still
only covers generic pull-request review. The repository also needs explicit
scheduled automation for critical correctness regressions and application
security review, plus a stronger PR-review prompt that raises the evidence bar
above generic code-quality feedback.

## What Changes

- Add a scheduled and manually dispatchable OpenCode workflow for critical
  correctness bug scans on recent commits.
- Add a scheduled and manually dispatchable OpenCode workflow for security review
  that writes sensitive findings to workflow artifacts instead of PR comments or
  public issues.
- Strengthen the existing `opencode-review` prompt so it prioritizes actionable
  correctness, security, and workflow-breakage findings with concrete evidence.
- Keep the new workflows pinned to the validated OpenCode action SHA and reuse
  the repository's existing OpenAI-compatible provider configuration.

## Capabilities

### New Capabilities
- `opencode-automation-workflows`: Scheduled and manually triggered OpenCode
  workflows for critical bug scans and security review, plus stronger PR review
  guidance.

### Modified Capabilities
- None.

## Impact

- Updates `.github/workflows/opencode-review.yml`.
- Adds `.github/workflows/opencode-critical-bug-scan.yml`.
- Adds `.github/workflows/opencode-security-review.yml`.
- Adds OpenSpec coverage for OpenCode automation workflow behavior and
  reporting boundaries.
