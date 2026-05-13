## Context

The project intentionally switched to Rust-first because `creditlint` integrates
with many repositories that may not have a JavaScript toolchain. At the same
time, npm distribution is useful for teams that already standardize developer
tools through package managers such as npm or pnpm.

The npm package should therefore stay thin. It should locate and execute a
native binary and exit with the same status code. It must not reimplement policy
logic.

## Goals / Non-Goals

**Goals:**

- Add a pnpm-managed npm workspace.
- Add an npm package named `creditlint`.
- Provide a `creditlint` bin entry that delegates to a native executable.
- Support local development through `CREDITLINT_BIN` and Rust build output
  fallback paths.
- Keep wrapper tests deterministic and dependency-light.

**Non-Goals:**

- Publishing platform-specific native npm packages in this change.
- Downloading binaries from GitHub Releases during install.
- Reimplementing any Rust policy behavior in JavaScript.
- Making Node.js required for Cargo/crates.io/GitHub Release consumers.

## Decisions

### Keep npm packaging under `npm/creditlint`

The root package is private and only defines the pnpm workspace. The publishable
package lives under `npm/creditlint`.

Rationale:

- Keeps Rust crate files and npm wrapper files separate.
- Leaves room for future platform-specific packages if needed.
- Lets pnpm operate without making the repository root the publishable npm
  package.

### Use environment override first

The wrapper should execute `CREDITLINT_BIN` when set.

Rationale:

- Tests can inject a fake binary.
- Developers can point the npm wrapper at a local Cargo build.
- CI can validate wrapper behavior without bundling release artifacts.

### Use local Rust build fallbacks for development

When `CREDITLINT_BIN` is not set, the wrapper should look for nearby packaged
native binaries, then repository-local Cargo build outputs.

Rationale:

- Published packages can later include staged native binaries under the package
  directory.
- Local workspace users can run the wrapper after `cargo build`.

## Risks / Trade-offs

- [Risk] npm package users may expect installation to include a native binary.
  -> Mitigate with clear error text and documentation until binary bundling is
  added.
- [Risk] wrapper and Rust CLI versions can drift. -> Mitigate by keeping package
  versions aligned with `Cargo.toml`.
- [Risk] Node wrapper can hide native exit codes. -> Mitigate by forwarding exit
  status and signals directly.
