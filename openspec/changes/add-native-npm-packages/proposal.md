## Why

The npm wrapper package should be usable by frontend and JavaScript-focused
teams without requiring a Rust toolchain. The current wrapper can delegate to a
local Cargo build through `CREDITLINT_BIN`, but that is a developer fallback, not
a good default for npm consumers.

## What Changes

- Add platform-specific npm package skeletons under `packages/`.
- Make the root `creditlint` npm package depend on those packages through
  optional dependencies.
- Update the wrapper to resolve the installed platform package before falling
  back to local development binaries.
- Add tests for platform package resolution and unsupported platforms.
- Document the npm distribution boundary and first-publish flow.

## Capabilities

### New Capabilities

- `native-npm-packages`: Platform package layout for npm consumers who should
  not need Rust or Cargo.

### Modified Capabilities

- `npm-wrapper-package`: The wrapper now resolves installed platform packages
  before local development fallbacks.

## Impact

- Adds package directories for Linux, macOS, and Windows x64/arm64 targets that
  match the release artifact matrix where practical.
- Updates npm package metadata, wrapper behavior, tests, and documentation.
- Does not add postinstall binary downloads.
- Does not remove `CREDITLINT_BIN`, which remains the deterministic override for
  tests and local development.
