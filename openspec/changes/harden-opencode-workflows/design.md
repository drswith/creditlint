## Context

`creditlint` now has two OpenCode-backed workflows: one for pull-request review
and one for issue triage. They already run successfully, so this change is not
about recovering broken behavior. It is a narrow hardening pass driven by the
latest review feedback and by the repository's existing bias toward explicit,
least-privilege workflow design.

## Goals / Non-Goals

**Goals:**
- Remove floating third-party action refs from OpenCode workflows.
- Prevent duplicate PR review runs from piling up on rapid pushes.
- Reduce workflow token permissions to the minimum required for current issue
  triage behavior.
- Keep the current trust model: OpenCode GitHub App for review comments and the
  configured model provider for inference.

**Non-Goals:**
- Add support for fork PR review through `pull_request_target`.
- Redesign the issue-age filter into a spam-closing policy.
- Change the current model/provider selection beyond keeping the existing review
  workflow functional.

## Decisions

- Pin `anomalyco/opencode/github` to the currently validated commit SHA used in
  successful runs instead of a floating `@latest` ref.
  Rationale: this reduces supply-chain drift while preserving the exact action
  behavior already proven in CI.

- Add a workflow-level concurrency group to `opencode-review`.
  Rationale: the review workflow is driven by PR updates and should prefer the
  newest commit state over queued stale reviews.

- Tighten `opencode-triage` permissions to `contents: read` and `issues: write`,
  while removing `pull-requests: write`.
  Rationale: the workflow only checks out the repo and comments/reacts on
  issues; it does not modify repository contents or PRs.

- Declare `contents: read` explicitly in `opencode-review`.
  Rationale: even though the current run succeeds, explicit read scope matches
  the workflow's actual checkout behavior and keeps the permission contract
  obvious.

## Risks / Trade-offs

- [Pinned action SHA can go stale] -> Updating OpenCode features later will
  require an intentional SHA bump instead of automatic drift.
- [Concurrency can cancel an in-progress review] -> This is acceptable because
  the latest PR head is the only review state that matters.
- [Permission tightening may miss a hidden action need] -> Validate with the
  live workflow run after the change lands.
