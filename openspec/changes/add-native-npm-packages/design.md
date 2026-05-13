## Context

`creditlint` is Rust-native, but npm users should not be forced to install Rust.
The npm distribution should follow the common native-tool pattern used by tools
such as esbuild: a small JavaScript wrapper package plus platform-specific
optional packages containing prebuilt binaries.

## Goals / Non-Goals

**Goals:**

- Keep `creditlint` as the primary npm package name.
- Add platform package skeletons that can carry native binaries.
- Resolve installed platform packages before Cargo build fallbacks.
- Keep npm installs reproducible and enterprise-friendly by avoiding dynamic
  postinstall downloads.
- Keep local tests deterministic without requiring release binaries.

**Non-Goals:**

- Building real native binaries into npm packages in this change.
- Publishing automation to npm in this change.
- Supporting every CPU/libc combination immediately.

## Decisions

### Use optional platform packages

The main package declares optional dependencies for supported platform packages.
Each platform package uses npm `os` and `cpu` metadata so npm/pnpm installs only
the applicable package and ignores unsupported packages.

Rationale:

- npm users install `creditlint` and get a native executable for their platform.
- Failed optional dependencies on other platforms do not break installation.
- Package contents are auditable and lockfile-friendly.

### Keep postinstall downloads out of the default path

The wrapper should not download GitHub Release assets during install or first
run.

Rationale:

- Avoids proxy and offline install failures.
- Keeps supply-chain scanning focused on npm package artifacts.
- Makes installs deterministic.

### Resolve packages before local Cargo output

Resolution order:

1. `CREDITLINT_BIN`
2. installed platform package binary
3. package-local `native/` binary
4. repository-local `target/release` and `target/debug`

Rationale:

- Tests and developers retain a deterministic override.
- Normal npm consumers use the installed platform package.
- Repository contributors can still run the wrapper after `cargo build`.

## Platform Package Names

Initial packages:

- `creditlint-darwin-arm64`
- `creditlint-darwin-x64`
- `creditlint-linux-x64`
- `creditlint-linux-arm64`
- `creditlint-windows-x64`

Windows arm64 is intentionally excluded until the release workflow builds that
artifact.

## Risks / Trade-offs

- [Risk] Platform package skeletons do not contain binaries yet. -> Mitigate by
  documenting that publishing the main package requires staging binaries first.
- [Risk] Linux musl/gnu differences may matter later. -> Start with the current
  release target naming and split Linux packages if the release matrix expands.
- [Risk] npm package version drift. -> Keep all package versions aligned with
  `Cargo.toml` and the main npm package.
