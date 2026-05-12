## ADDED Requirements

### Requirement: Project Initialization

The CLI SHALL provide an `init` command that creates a default project policy
file.

#### Scenario: Initialize missing config

- **WHEN** the user runs `creditlint init` in a repository without
  `.creditlint.yml`
- **THEN** the CLI SHALL create `.creditlint.yml` with the default policy

#### Scenario: Existing config is preserved

- **WHEN** the user runs `creditlint init` and `.creditlint.yml` already exists
- **THEN** the CLI SHALL refuse to overwrite it unless an explicit future
  overwrite option is provided

### Requirement: Local Git Hook Integration

The CLI SHALL support installation of a local `commit-msg` hook that validates
the pending commit message.

#### Scenario: Install hook into repository

- **WHEN** the user runs `creditlint install-hook`
- **THEN** the CLI SHALL install a `commit-msg` hook that invokes
  `creditlint check --message-file`

#### Scenario: Existing unmanaged hook is preserved

- **WHEN** a `commit-msg` hook already exists and is not managed by `creditlint`
- **THEN** the CLI SHALL refuse to overwrite it and explain the manual
  integration path

#### Scenario: Managed hook marker is used

- **WHEN** `creditlint install-hook` creates a hook
- **THEN** the hook file SHALL include a stable `creditlint managed hook` marker
  with a version field

#### Scenario: Managed hook can be upgraded

- **WHEN** `creditlint install-hook` finds an existing hook with a compatible
  `creditlint managed hook` marker
- **THEN** the CLI SHALL be allowed to replace that managed hook

### Requirement: CI Integration

The CLI SHALL support use as a required pull-request check.

#### Scenario: Pull request range check

- **WHEN** CI runs `creditlint check --range <base>..HEAD`
- **THEN** the CLI SHALL validate the pull request commit messages and fail the
  job when violations are found

#### Scenario: Pull request title and body check

- **WHEN** CI writes a pull request title and body into a temporary text file and
  runs `creditlint check --message-file <file>`
- **THEN** the CLI SHALL validate that text using the same policy engine as
  commit-message checks

#### Scenario: Fetch depth requirement is documented

- **WHEN** documentation shows GitHub Actions usage
- **THEN** it SHALL include a full-history checkout or equivalent fetch strategy
  needed for range checks

### Requirement: GitHub Squash Merge Coverage

The CLI SHALL document and support a path for repositories that keep GitHub
squash merge enabled.

#### Scenario: Ruleset regex generation

- **WHEN** the user runs `creditlint github ruleset-pattern`
- **THEN** the CLI SHALL generate a GitHub commit-message ruleset pattern when
  the active policy can be represented safely as one regex

#### Scenario: Ruleset representable subset is documented

- **WHEN** documentation describes GitHub ruleset export
- **THEN** it SHALL list which policy features can and cannot be represented as
  a single commit-message regex

#### Scenario: Unsupported ruleset conversion fails closed

- **WHEN** the active policy cannot be represented safely as a GitHub ruleset
  regex
- **THEN** the CLI SHALL fail and explain that CI alone cannot guarantee final
  squash message validation

#### Scenario: Merge bot validates final message

- **WHEN** a merge bot provides the final squash commit message via
  `creditlint check --message-file`
- **THEN** the CLI SHALL validate that final message before the bot performs the
  merge

### Requirement: Governance Boundary Documentation

Documentation SHALL clearly distinguish local feedback, CI checks, repository
rulesets, and merge-bot enforcement.

#### Scenario: CI limitation is documented

- **WHEN** the documentation describes `creditlint check --range`
- **THEN** it SHALL state that range checks do not by themselves validate a
  final squash message edited by the platform UI

#### Scenario: Strongest enforcement path is documented

- **WHEN** the documentation describes GitHub squash merge support
- **THEN** it SHALL recommend repository ruleset metadata restrictions or a
  controlled merge bot for final message enforcement

#### Scenario: Privacy boundary is documented

- **WHEN** documentation describes CLI behavior
- **THEN** it SHALL state that the default CLI does not upload commit messages or
  pull request text to a hosted service
