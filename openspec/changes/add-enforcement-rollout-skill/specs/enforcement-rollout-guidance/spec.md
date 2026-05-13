## ADDED Requirements

### Requirement: Repository Skill Defines Full Enforcement Rollout

The repository SHALL provide a skill that explains how to deploy `creditlint`
for strong interception across the main commit and merge entry points.

#### Scenario: Skill lists enforcement layers

- **WHEN** a user opens the rollout skill
- **THEN** it SHALL describe the local hook, CI commit check, PR title/body
  check, final merge-message check, and repository governance layers

#### Scenario: Skill explains actions outside creditlint integration

- **WHEN** a user follows the rollout skill
- **THEN** it SHALL identify the required repository-hosting actions beyond
  installing `creditlint`, including GitHub settings or equivalent governance
  configuration

### Requirement: Rollout Guidance States Squash-Merge Boundary

The rollout guidance SHALL explain the limits of local hooks and CI checks for
platform-generated final merge messages.

#### Scenario: Squash merge boundary is explicit

- **WHEN** the skill discusses complete interception
- **THEN** it SHALL explain that local hooks and PR commit checks do not by
  themselves validate a final squash merge message edited in the hosting
  platform UI

#### Scenario: Final merge-message control path is recommended

- **WHEN** the skill discusses repositories that keep squash merge enabled
- **THEN** it SHALL recommend repository ruleset restrictions or merge-bot
  validation for the final merge message
