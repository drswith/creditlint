## Why

Users adopting `creditlint` often understand how to run the binary, but not how
to close the remaining enforcement gaps around hooks, CI, PR text, squash merge
messages, and repository settings. The repository needs a reusable rollout
guide that an agent or maintainer can follow end to end.

## What Changes

- Add a repository-local skill that explains the full `creditlint` enforcement
  rollout for a target repository.
- Define the operational layers required for strong interception beyond merely
  installing `creditlint`.
- Add a README entry with a copyable prompt that points users at the skill.

## Capabilities

### New Capabilities
- `enforcement-rollout-guidance`: Documents the full adoption checklist for
  hooks, CI, PR text validation, repository rulesets, and merge-message
  enforcement boundaries.

### Modified Capabilities
- `workflow-integrations`: Clarify that user-facing guidance SHALL explain the
  full interception layers required around squash merge and repository
  governance.

## Impact

- Adds a new repository-local skill under `skills/`.
- Updates `README.md` with a prompt-based entry for the rollout guide.
- Adds OpenSpec coverage for the guidance and README entrypoint behavior.
