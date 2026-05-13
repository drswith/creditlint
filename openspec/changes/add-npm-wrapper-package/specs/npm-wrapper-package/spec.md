## ADDED Requirements

### Requirement: Workspace Package Layout

The repository SHALL define a pnpm workspace for npm packaging.

#### Scenario: Root workspace is private

- **WHEN** a developer reads the root `package.json`
- **THEN** it SHALL be private and point pnpm to npm package workspaces

#### Scenario: creditlint npm package exists

- **WHEN** a developer reads `packages/creditlint/package.json`
- **THEN** it SHALL define a package named `creditlint` with a `creditlint` bin
  entry

### Requirement: Native Wrapper Execution

The npm bin wrapper SHALL delegate execution to a native `creditlint` binary.

#### Scenario: CREDITLINT_BIN override is used

- **WHEN** `CREDITLINT_BIN` points to an executable
- **THEN** the wrapper SHALL execute that binary with the user-provided
  arguments

#### Scenario: Native exit code is forwarded

- **WHEN** the native binary exits with a non-zero code
- **THEN** the wrapper SHALL exit with the same code

#### Scenario: Missing native binary fails clearly

- **WHEN** no native binary can be found
- **THEN** the wrapper SHALL exit with code `2` and explain how to provide a
  binary

### Requirement: Wrapper Tests

The npm wrapper package SHALL include tests runnable through pnpm.

#### Scenario: pnpm package test passes

- **WHEN** a developer runs `pnpm --filter creditlint test`
- **THEN** the wrapper tests SHALL pass without requiring a real release binary

### Requirement: Documentation Boundary

Documentation SHALL distinguish Rust-native installation from optional npm
wrapper usage.

#### Scenario: npm wrapper is documented as optional

- **WHEN** a user reads the README
- **THEN** it SHALL state that npm usage is optional and delegates to the native
  binary
