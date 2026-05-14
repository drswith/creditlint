## ADDED Requirements

### Requirement: OpenCode PR Review Uses Evidence-Driven Prompting
The pull-request review workflow SHALL use a prompt that prioritizes
evidence-backed correctness, security, regression, and workflow-breakage
findings over generic review commentary.

#### Scenario: PR review prompt requires concrete justification
- **WHEN** `.github/workflows/opencode-review.yml` invokes OpenCode for a pull
  request
- **THEN** the prompt SHALL instruct OpenCode to report only actionable findings
  it can justify with a concrete code path, trigger scenario, or permission
  boundary

### Requirement: Critical Bug Scan Workflow Exists
The repository SHALL provide a scheduled and manually triggered OpenCode
workflow for high-severity correctness bug hunting on recent commits.

#### Scenario: Critical bug scan can run on a schedule
- **WHEN** `.github/workflows/opencode-critical-bug-scan.yml` is loaded
- **THEN** the workflow SHALL support a `schedule` trigger and SHALL provide a
  prompt focused on critical correctness bugs in recent commits

#### Scenario: Critical bug scan can open a fix PR
- **WHEN** the critical bug scan finds a real critical bug and chooses to fix it
- **THEN** the workflow SHALL have the repository write permissions needed to
  commit changes and open a pull request

### Requirement: Security Review Workflow Preserves Sensitive Findings
The repository SHALL provide a scheduled and manually triggered OpenCode
security-review workflow that keeps findings inside the workflow run instead of
posting them publicly.

#### Scenario: Security review is read-only
- **WHEN** `.github/workflows/opencode-security-review.yml` runs
- **THEN** the workflow SHALL request only read access to repository contents
  and SHALL NOT request pull-request or issue write scopes

#### Scenario: Security findings are written to artifacts
- **WHEN** the security review identifies validated findings
- **THEN** the workflow SHALL direct OpenCode to write the findings to local
  files for artifact upload instead of posting them as PR comments or issues

#### Scenario: Security review can be run manually
- **WHEN** a maintainer wants to trigger the security review on demand
- **THEN** `.github/workflows/opencode-security-review.yml` SHALL support
  `workflow_dispatch`
