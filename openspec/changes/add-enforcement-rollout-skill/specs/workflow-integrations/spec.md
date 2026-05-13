## MODIFIED Requirements

### Requirement: Governance Boundary Documentation

The documentation SHALL explain the enforcement boundary between CI checks,
rulesets, and merge-bot enforcement.

#### Scenario: CI boundary is documented

- **WHEN** the documentation describes `creditlint check --range`
- **THEN** it SHALL explain that range checks validate proposed commits but not a
  final squash message edited by the platform UI

#### Scenario: Squash merge path is documented

- **WHEN** the documentation describes GitHub squash merge support
- **THEN** it SHALL recommend repository ruleset metadata restrictions or a
  merge-bot validation path for the final merge message

#### Scenario: User-facing rollout guidance is documented

- **WHEN** the repository provides user-facing adoption guidance
- **THEN** it SHALL describe the local hook, CI, PR text, and final
  merge-message layers needed for strong interception
