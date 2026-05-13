## ADDED Requirements

### Requirement: CI Workflow Linting

The CI workflow SHALL statically lint GitHub Actions workflow files.

#### Scenario: Workflow lint runs on pull requests

- **WHEN** CI runs for a pull request
- **THEN** it SHALL run a workflow linter against `.github/workflows/*.yml`

### Requirement: CI Project Validation

The CI workflow SHALL validate Rust code and OpenSpec artifacts.

#### Scenario: Rust and OpenSpec validation pass

- **WHEN** CI runs
- **THEN** it SHALL run formatting, clippy, tests, release binary smoke testing,
  and OpenSpec validation

### Requirement: Release Validation Gate

The release workflow SHALL validate the repository before building publishable
artifacts.

#### Scenario: Release artifacts wait for validation

- **WHEN** the release workflow runs
- **THEN** artifact build jobs SHALL depend on a validation job that runs
  formatting, clippy, tests, and OpenSpec validation

### Requirement: Least-Privilege Workflow Permissions

Workflows SHALL default to read-only repository permissions and grant write
permissions only to jobs that publish release assets.

#### Scenario: Release asset publishing has write permission

- **WHEN** a tag release publishes GitHub Release assets
- **THEN** only the jobs that need to write release assets SHALL request
  `contents: write`

### Requirement: Release Checksums

The release workflow SHALL generate checksums for native artifacts.

#### Scenario: SHA256SUMS is generated

- **WHEN** all native release artifacts are built
- **THEN** the workflow SHALL generate a combined `SHA256SUMS` file

#### Scenario: SHA256SUMS is published for tags

- **WHEN** the release workflow runs for a version tag
- **THEN** the workflow SHALL upload `SHA256SUMS` as a GitHub Release asset

### Requirement: Artifact Architecture Checks

The release workflow SHALL verify Unix artifact architectures before publishing.

#### Scenario: macOS arm64 artifact is checked

- **WHEN** the macOS arm64 release artifact is built
- **THEN** the workflow SHALL verify the binary reports an arm64 architecture

#### Scenario: macOS Intel artifact is checked

- **WHEN** the macOS Intel release artifact is built
- **THEN** the workflow SHALL verify the binary reports an x86_64 architecture

### Requirement: Workflow Concurrency

Workflows SHALL define concurrency groups to avoid duplicate runs racing each
other.

#### Scenario: CI cancels stale runs

- **WHEN** a newer CI run starts for the same ref
- **THEN** the previous in-progress CI run for that ref SHALL be cancelled

#### Scenario: Release runs do not cancel in-progress publishes

- **WHEN** a release run is already publishing for a ref
- **THEN** a later release run for the same ref SHALL not cancel it mid-publish
