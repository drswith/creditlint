## ADDED Requirements

### Requirement: Default Credit Policy

The system SHALL provide a default credit/authorship policy when no project
configuration file is present.

#### Scenario: Reject AI tool co-author trailer

- **WHEN** a message contains `Co-authored-by: Codex <codex@example.com>`
- **THEN** the system SHALL report a violation for an unauthorized authorship
  marker

#### Scenario: Allow human co-author trailer by default

- **WHEN** a message contains `Co-authored-by: Jane Doe <jane@example.com>`
- **THEN** the system SHALL not report a violation from the default policy

#### Scenario: Reject made-with marker

- **WHEN** a message contains `Made with Cursor`
- **THEN** the system SHALL report a violation for a prohibited credit marker

#### Scenario: Allow provenance disclosure trailer

- **WHEN** a message contains `AI-Assisted: true`
- **THEN** the system SHALL not report a violation from the default policy

#### Scenario: Reject AI tool Git author identity

- **WHEN** Git metadata identifies the author as
  `Cursor Agent <cursoragent@cursor.com>`
- **THEN** the system SHALL report a violation for an unauthorized Git author
  identity

#### Scenario: Reject AI tool Git committer identity

- **WHEN** Git metadata identifies the committer with an AI/tool identity
- **THEN** the system SHALL report a violation for an unauthorized Git committer
  identity

#### Scenario: Allow human Git author identity by default

- **WHEN** Git metadata identifies the author as `Jane Doe <jane@example.com>`
- **THEN** the system SHALL not report a violation from the default policy

### Requirement: Configurable Policy

The system SHALL load policy configuration from `.creditlint.yml` when present.

#### Scenario: Config discovery starts at current working directory

- **WHEN** the user runs `creditlint` from a subdirectory
- **THEN** the system SHALL search from the current working directory upward
  until it finds `.creditlint.yml` or reaches the repository root

#### Scenario: Repository root bounds config discovery

- **WHEN** no `.creditlint.yml` exists between the current working directory and
  the repository root
- **THEN** the system SHALL stop discovery at the repository root and use the
  built-in default policy

#### Scenario: Invalid config fails closed

- **WHEN** `.creditlint.yml` is syntactically invalid
- **THEN** the system SHALL fail with an invalid-config error instead of passing
  the check

#### Scenario: Custom forbidden trailer applies

- **WHEN** `.creditlint.yml` defines a custom forbidden trailer rule
- **THEN** the system SHALL evaluate messages against that custom rule

#### Scenario: Custom forbidden identity applies

- **WHEN** `.creditlint.yml` defines a custom forbidden identity rule
- **THEN** the system SHALL evaluate Git author and committer metadata against
  that custom rule

#### Scenario: Missing config uses defaults

- **WHEN** no `.creditlint.yml` file exists
- **THEN** the system SHALL evaluate messages and Git identity metadata with the
  built-in default policy

### Requirement: Rule Precedence

The system SHALL apply forbidden rules before allowed provenance declarations.

#### Scenario: Forbidden rule wins over allowed provenance key

- **WHEN** a message contains `Generated-by: Codex` and the active policy allows
  `Generated-by` as a provenance key but forbids the value `Codex`
- **THEN** the system SHALL report a violation

#### Scenario: Allowed provenance key passes without forbidden match

- **WHEN** a message contains `Generated-by: internal-build-script` and no
  forbidden rule matches the key or value
- **THEN** the system SHALL not report a violation for that provenance key

### Requirement: Message Analysis

The system SHALL analyze raw message text for both trailer-like key-value lines
and common free-form credit markers.

#### Scenario: Trailer-like line is detected

- **WHEN** a message contains a matching `Key: value` line
- **THEN** the system SHALL evaluate the key and value against trailer policy
  rules

#### Scenario: Free-form marker is detected

- **WHEN** a message contains a matching free-form marker such as `Generated with
  Claude`
- **THEN** the system SHALL evaluate the line against marker policy rules

#### Scenario: Free-form marker does not match prose

- **WHEN** a message body contains `The fix was made with care.`
- **THEN** the system SHALL not report a `made-with` violation

#### Scenario: Free-form marker matches explicit marker line

- **WHEN** a message contains a standalone line `Made with Cursor`
- **THEN** the system SHALL report a prohibited credit marker violation

#### Scenario: Violation includes location

- **WHEN** the system reports a violation
- **THEN** the violation SHALL include the source type, rule identifier, message,
  and line number when a line number is available

### Requirement: Structured Violations

The system SHALL return policy results as structured violation objects before
formatting them for the terminal.

#### Scenario: Violation object is produced

- **WHEN** a forbidden marker is detected
- **THEN** the checker SHALL return a violation object containing `source`,
  `rule`, `field` when applicable, and a human-readable message

#### Scenario: Commit metadata is preserved

- **WHEN** a violation comes from a Git commit range
- **THEN** the violation SHALL include the commit SHA

#### Scenario: Identity field is preserved

- **WHEN** a violation comes from Git author or committer metadata
- **THEN** the violation SHALL include the matching field such as `author.name`,
  `author.email`, `committer.name`, or `committer.email`
