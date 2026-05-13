## Why

`creditlint` is Rust-first and should not require Node.js for normal use, but an
npm package is still valuable for teams that already install developer tools
through npm, pnpm, or npx. The npm package should be an optional wrapper around
the native CLI, not a second implementation.

## What Changes

- Add a pnpm workspace at the repository root.
- Add an npm package structure under `packages/creditlint`.
- Provide a `creditlint` npm bin wrapper that delegates to a native
  `creditlint` binary.
- Add wrapper tests that run through pnpm.
- Document npm wrapper usage and its boundary.

## Capabilities

### New Capabilities

- `npm-wrapper-package`: Optional npm/pnpm wrapper package for invoking the
  native `creditlint` CLI.

### Modified Capabilities

- None. The Rust CLI remains the source of truth.

## Impact

- Adds `package.json`, `pnpm-workspace.yaml`, and `pnpm-lock.yaml`.
- Adds `packages/creditlint` package files and tests.
- Updates README and contributor guidance.
- Does not add Node.js as a runtime requirement for the native `creditlint`
  binary.
