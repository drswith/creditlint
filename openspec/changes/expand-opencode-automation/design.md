## Context

The current OpenCode integration covers PR review and issue triage. That is a
good baseline, but it does not give the repository a dedicated scheduled path
for deeper bug hunting or security review. OpenCode's GitHub integration
supports `schedule` and `workflow_dispatch`, and for those event types the
prompt becomes the primary control surface. In this repository, that means the
workflow design has to make the reporting and permission boundary explicit.

## Goals / Non-Goals

**Goals:**
- Add a scheduled critical-bug scan that may open a PR only when it finds and
  fixes a high-confidence critical regression.
- Add a scheduled security review that never opens a PR and never posts
  externally, because the current GitHub Actions environment does not provide a
  private Slack or memory tool.
- Strengthen the default PR review prompt so it focuses on actionable,
  evidence-backed findings instead of broad generic review chatter.
- Reuse the existing OpenAI-compatible provider wiring so the new workflows do
  not introduce another model-configuration path.

**Non-Goals:**
- Build a durable cross-run vulnerability memory system in this repository.
- Add private Slack reporting or GitHub Security Advisory publication.
- Rework `opencode-triage` as part of this change.

## Decisions

- Use two separate workflows instead of folding all prompts into
  `opencode-review`.
  Rationale: the review trigger, risk model, permissions, and output behavior
  differ materially between PR review, scheduled bug hunting, and scheduled
  security review.

- Give the critical-bug scan write permissions for `contents` and
  `pull-requests`.
  Rationale: its prompt explicitly allows a minimal fix and PR when it finds a
  real critical bug; without write scopes the workflow could not complete that
  path.

- Keep the security review on read-only repository contents and prohibit PR
  creation in the prompt.
  Rationale: its job is validated vulnerability discovery, not automated code
  modification, and the findings may be sensitive.

- Adapt the security-review reporting path to local workflow artifacts under
  `.opencode-output/`.
  Rationale: the original prompt assumed automation memory and a Slack posting
  tool, neither of which is available in the current GitHub Actions setup.

- Strengthen the PR review prompt around evidence, trigger scenarios, and
  severity focus instead of replacing it with one of the scheduled prompts.
  Rationale: PR review is still a different job from scheduled bug hunts or
  vulnerability review.

## Risks / Trade-offs

- [Scheduled scans may consume more model budget] -> Run them weekly and add
  manual dispatch for ad hoc use instead of making them part of every PR.
- [Critical bug scan may open an unnecessary PR] -> Keep the prompt strict about
  only acting on high-confidence critical findings and treating "no critical
  bugs found" as the expected outcome.
- [Security findings stored as artifacts are not cross-run memory] -> Document
  that this is an environment constraint and keep the workflow read-only with no
  external posting.
