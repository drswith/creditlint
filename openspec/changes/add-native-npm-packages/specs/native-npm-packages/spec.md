## ADDED Requirements

### Requirement: Platform Package Layout

The npm workspace SHALL include platform-specific packages for native binaries.

#### Scenario: Platform package declares native metadata

- **WHEN** a developer reads a platform package `package.json`
- **THEN** it SHALL declare `os`, `cpu`, package metadata, and files for a
  packaged native binary

#### Scenario: Main package uses optional dependencies

- **WHEN** a developer reads `packages/creditlint/package.json`
- **THEN** it SHALL list supported platform packages in `optionalDependencies`

### Requirement: Native Package Resolution

The npm wrapper SHALL resolve the current platform package before local Cargo
build outputs.

#### Scenario: Installed platform package is used

- **WHEN** `CREDITLINT_BIN` is unset and the current platform package provides a
  binary
- **THEN** the wrapper SHALL execute that binary with the user-provided
  arguments

#### Scenario: Override remains highest priority

- **WHEN** `CREDITLINT_BIN` points to an executable
- **THEN** the wrapper SHALL execute it before checking platform packages

#### Scenario: Unsupported platform fails clearly

- **WHEN** the current `process.platform` and `process.arch` combination is not
  supported
- **THEN** the wrapper SHALL exit with code `2` and report the unsupported
  platform key

#### Scenario: Missing installed package fails clearly

- **WHEN** the current platform is supported but no platform package binary or
  local fallback binary can be found
- **THEN** the wrapper SHALL exit with code `2` and explain that the optional
  platform package may be missing

### Requirement: Publish Boundary Documentation

Documentation SHALL state that npm consumers should not need Rust.

#### Scenario: First manual publish path is documented

- **WHEN** a maintainer reads the npm wrapper README
- **THEN** it SHALL explain that platform package binaries must be staged before
  publishing and list the manual publish order

### Requirement: Ordered Publish Script

The repository SHALL provide a script for publishing npm packages in dependency
order.

#### Scenario: Missing native binaries fail before publish

- **WHEN** a maintainer runs the npm publish script without all required
  platform binaries
- **THEN** the script SHALL exit non-zero before running any publish command

#### Scenario: Dist binaries are staged automatically

- **WHEN** the configured npm dist directory contains all required platform
  binaries
- **THEN** the script SHALL copy them into the matching platform package `bin`
  directories before publishing

#### Scenario: Platform packages publish before main package

- **WHEN** a maintainer runs the npm publish script with all required platform
  binaries
- **THEN** the script SHALL publish platform packages before the main
  `creditlint` package

### Requirement: Trusted Publishing Bootstrap

The repository SHALL provide a bootstrap publish path for creating npm package
records before trusted publishing is configured.

#### Scenario: Bootstrap does not require native binaries

- **WHEN** a maintainer runs the trusted publishing bootstrap script
- **THEN** it SHALL publish placeholder packages without requiring native
  binaries

#### Scenario: Bootstrap uses non-latest prerelease

- **WHEN** the bootstrap script publishes placeholder packages
- **THEN** it SHALL use version `0.0.0-trust.0` and the `bootstrap` dist-tag

#### Scenario: Bootstrap publishes to official npm by default

- **WHEN** local user npm configuration points at a registry mirror
- **THEN** each publishable npm package SHALL provide `publishConfig.registry`
  set to `https://registry.npmjs.org/` unless a registry override is provided

#### Scenario: Bootstrap main package is not usable

- **WHEN** a user accidentally invokes the bootstrap main package binary
- **THEN** it SHALL exit with code `2` and explain that the version is only a
  trusted publishing placeholder
