## 1. OpenCode Workflow Expansion

- [x] 1.1 Strengthen the PR review prompt in `opencode-review.yml` to require evidence-backed, actionable findings.
- [x] 1.2 Add `opencode-critical-bug-scan.yml` with schedule and workflow_dispatch triggers, pinned OpenCode action, shared provider config, and write permissions for fix PRs.
- [x] 1.3 Add `opencode-security-review.yml` with schedule and workflow_dispatch triggers, pinned OpenCode action, shared provider config, read-only permissions, and artifact upload for local findings.

## 2. Validation

- [x] 2.1 Run workflow linting against `.github/workflows/*.yml`.
- [x] 2.2 Run OpenSpec validation and confirm the new change is complete.
