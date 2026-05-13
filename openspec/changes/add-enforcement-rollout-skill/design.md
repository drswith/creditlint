## Context

`creditlint` already documents local hooks, CI range checks, PR text checking,
GitHub ruleset export, and merge-bot validation. What is missing is a single
operator-facing guide that explains how these layers fit together when a team
wants strong interception instead of a partial lint-only deployment.

The user specifically wants this guidance captured as a skill so it can be
reused by an agent inside another repository, and surfaced from the README with
copyable prompt text.

## Goals / Non-Goals

**Goals:**

- Create a repository-local skill that explains the complete enforcement rollout
  path for a target repository.
- Make the rollout steps explicit about what must happen outside the
  `creditlint` codebase itself, such as GitHub settings and merge workflow
  choices.
- Add a README section that gives users a ready-to-copy prompt for invoking this
  guidance through an agent.

**Non-Goals:**

- Changing `creditlint` runtime behavior.
- Adding GitHub API automation for repository settings.
- Replacing existing README integration details with the skill content.

## Decisions

### Add a repo-local skill instead of only expanding README prose

The primary artifact should be a `skills/<name>/SKILL.md` guide, with README
acting as the entrypoint.

Rationale:

- The user asked for the result in skill form.
- A skill is easier for an agent to consume consistently than a long README
  section.
- README should remain the discovery surface, not the only copy of the full
  rollout procedure.

### Organize rollout guidance by enforcement layer

The skill should explain full interception in layers: local commit path, CI
commit range path, PR title/body path, final squash/merge message path, and
repository governance settings.

Rationale:

- This matches the real control boundaries already documented in
  `creditlint`.
- It makes clear that no single layer fully replaces the others.
- It helps users decide what is mandatory versus optional for their workflow.

### Include copyable prompts in README

README should include one short, copyable prompt that asks an agent to apply the
skill to the current repository.

Rationale:

- Users asked for a prompt-based entrypoint.
- A copyable prompt lowers adoption friction more than a prose-only pointer.

## Risks / Trade-offs

- [Risk] The skill could over-promise full enforcement in workflows where GitHub
  UI controls remain manual. -> Mitigation: keep squash-merge and ruleset
  boundaries explicit, and state when merge-bot validation is still required.
- [Risk] README and skill content can drift. -> Mitigation: keep README concise
  and point it at the skill as the deeper source of guidance.
