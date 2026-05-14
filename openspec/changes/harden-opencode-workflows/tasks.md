## 1. OpenCode Workflow Hardening

- [x] 1.1 Pin `anomalyco/opencode/github` to a validated immutable commit SHA in `opencode-review.yml` and `opencode-triage.yml`.
- [x] 1.2 Add `contents: read` and workflow concurrency to `opencode-review.yml`.
- [x] 1.3 Tighten `opencode-triage.yml` permissions to least privilege for checkout and issue comments/reactions.

## 2. Validation

- [x] 2.1 Run workflow linting against `.github/workflows/*.yml`.
- [x] 2.2 Run OpenSpec validation and confirm the new change is apply-ready.
