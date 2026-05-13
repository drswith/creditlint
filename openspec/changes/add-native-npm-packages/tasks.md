## 1. Specification and Package Layout

- [x] 1.1 Add OpenSpec change for native npm package distribution.
- [x] 1.2 Add platform npm package skeletons under `packages/`.
- [x] 1.3 Add platform packages as optional dependencies of `creditlint`.

## 2. Wrapper Resolution

- [x] 2.1 Resolve installed platform package binaries before local Cargo outputs.
- [x] 2.2 Preserve `CREDITLINT_BIN` as the highest-priority override.
- [x] 2.3 Fail clearly for unsupported platforms or missing platform packages.

## 3. Tests and Documentation

- [x] 3.1 Add wrapper tests for platform package resolution.
- [x] 3.2 Document npm installation without Rust and first manual publish steps.
- [x] 3.3 Run pnpm tests, Rust validation, workflow linting, and OpenSpec validation.
