## ADDED Requirements

### Requirement: OpenCode Workflows Use Immutable Action Refs
OpenCode-powered GitHub Actions workflows SHALL pin the
`anomalyco/opencode/github` action to an immutable commit SHA instead of a
floating version alias.

#### Scenario: Review workflow pins the OpenCode action
- **WHEN** `.github/workflows/opencode-review.yml` invokes OpenCode
- **THEN** the workflow SHALL reference `anomalyco/opencode/github` by commit
  SHA

#### Scenario: Triage workflow pins the OpenCode action
- **WHEN** `.github/workflows/opencode-triage.yml` invokes OpenCode
- **THEN** the workflow SHALL reference `anomalyco/opencode/github` by commit
  SHA

### Requirement: OpenCode Review Cancels Stale Runs
The pull-request review workflow SHALL avoid wasting model calls on superseded
PR revisions.

#### Scenario: New review run cancels the previous in-progress run
- **WHEN** a newer `pull_request` event starts `opencode-review` for the same
  pull request
- **THEN** the previous in-progress review run SHALL be cancelled

### Requirement: OpenCode Workflows Use Least-Privilege Token Scopes
OpenCode workflows SHALL request only the GitHub Actions token scopes needed for
their current checkout and commenting behavior.

#### Scenario: Review workflow declares read-only repository contents
- **WHEN** `.github/workflows/opencode-review.yml` checks out repository files
- **THEN** the workflow SHALL request `contents: read` and SHALL NOT request
  broader repository content permissions

#### Scenario: Triage workflow limits repository and issue scopes
- **WHEN** `.github/workflows/opencode-triage.yml` comments on or reacts to an
  issue
- **THEN** the workflow SHALL request `issues: write`, `contents: read`, and
  SHALL NOT request `contents: write` or `pull-requests: write`
