## ADDED Requirements

### Requirement: Check Command Input Modes

The CLI SHALL provide a `check` command that can validate a message file, stdin,
or a Git revision range.

#### Scenario: Check message file

- **WHEN** the user runs `creditlint check --message-file .git/COMMIT_EDITMSG`
- **THEN** the CLI SHALL validate the file contents as one message

#### Scenario: Check arbitrary text file

- **WHEN** the user runs `creditlint check --message-file pr-message.txt`
- **THEN** the CLI SHALL validate the file contents as one message regardless of
  whether the file came from a commit, pull request, or merge bot

#### Scenario: Check stdin

- **WHEN** the user runs `creditlint check --stdin` and pipes message text to
  the process
- **THEN** the CLI SHALL validate the stdin contents as one message

#### Scenario: Check Git range

- **WHEN** the user runs `creditlint check --range origin/main..HEAD`
- **THEN** the CLI SHALL validate every commit's Git author metadata, committer
  metadata, and message in that revision range

### Requirement: Audit Command

The CLI SHALL provide an `audit --all` command that scans all reachable commit
metadata and messages in the current repository.

#### Scenario: Audit all commits

- **WHEN** the user runs `creditlint audit --all`
- **THEN** the CLI SHALL evaluate all reachable commit identity metadata and
  messages and report any violations

#### Scenario: Audit outside Git repository

- **WHEN** the user runs `creditlint audit --all` outside a Git repository
- **THEN** the CLI SHALL exit with an invocation or environment error

### Requirement: Output Formats

The CLI SHALL support human-readable output and JSON output for checks and
audits.

#### Scenario: Human output reports violations

- **WHEN** violations are found with default output settings
- **THEN** the CLI SHALL print a concise human-readable report that identifies
  each violation

#### Scenario: JSON output reports violations

- **WHEN** the user passes `--format json`
- **THEN** the CLI SHALL print a stable JSON object containing `ok` and
  `violations`

#### Scenario: Clean output is quiet by default

- **WHEN** no violations are found
- **THEN** the CLI SHALL print no verbose report unless the user requested a
  verbose format

### Requirement: Exit Codes

The CLI SHALL use stable exit codes so hooks and CI can depend on them.

#### Scenario: No violations

- **WHEN** the check completes successfully with no violations
- **THEN** the CLI SHALL exit with code `0`

#### Scenario: Violations found

- **WHEN** the check completes and finds one or more violations
- **THEN** the CLI SHALL exit with code `1`

#### Scenario: Invalid invocation or missing metadata

- **WHEN** required input is missing, unreadable, invalid, or cannot be collected
- **THEN** the CLI SHALL exit with code `2`

#### Scenario: Invalid config returns usage error code

- **WHEN** configuration cannot be parsed or validated
- **THEN** the CLI SHALL exit with code `2`

### Requirement: Git Metadata Collection

The CLI SHALL collect commit SHA, author name, author email, committer name,
committer email, and commit message from Git using deterministic delimiters that
can be parsed without mixing commits together.

#### Scenario: Git range cannot be resolved

- **WHEN** the requested Git range cannot be resolved
- **THEN** the CLI SHALL fail closed with exit code `2`

#### Scenario: Commit message violation includes SHA

- **WHEN** a violation is found in a Git range check
- **THEN** the CLI SHALL include the offending commit SHA in the violation
  report

#### Scenario: Git author violation includes SHA and identity field

- **WHEN** a Git range check finds `Cursor Agent <cursoragent@cursor.com>` in
  author metadata
- **THEN** the CLI SHALL include the offending commit SHA and the matching
  author field in the violation report

#### Scenario: Audit processes large histories incrementally

- **WHEN** the CLI evaluates many commits through `audit --all`
- **THEN** it SHALL process Git metadata incrementally instead of requiring all
  commit messages to be loaded into memory at once
