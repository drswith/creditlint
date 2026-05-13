# creditlint npm wrapper

This package provides the optional npm `creditlint` command for teams that
install developer tools through npm, pnpm, or npx.

Normal npm consumers should not need Rust or Cargo. This package resolves a
platform-specific optional dependency such as `creditlint-darwin-arm64` or
`creditlint-linux-x64`, then runs the native Rust `creditlint` binary from that
package.

The implementation is a thin wrapper. It does not reimplement policy parsing or
Git metadata checks in JavaScript.

## Install

```sh
pnpm add -D creditlint
pnpm exec creditlint --help
```

## Local Development

Build the native CLI first:

```sh
cargo build
```

Then point the wrapper at the local binary:

```sh
CREDITLINT_BIN="$PWD/target/debug/creditlint" pnpm --filter creditlint run creditlint --help
```

Run wrapper tests:

```sh
pnpm --filter creditlint test
```

If `CREDITLINT_BIN` is not set, the wrapper checks for a packaged native binary
from the installed platform package, then `native/`, then repository-local Cargo
build outputs.

## First Manual Publish

Before publishing the main package, stage real release binaries into the
platform packages:

```text
packages/creditlint-darwin-arm64/bin/creditlint
packages/creditlint-darwin-x64/bin/creditlint
packages/creditlint-linux-arm64/bin/creditlint
packages/creditlint-linux-x64/bin/creditlint
packages/creditlint-windows-x64/bin/creditlint.exe
```

Then publish platform packages first, followed by the main wrapper package:

```sh
cd /path/to/creditlint

scripts/bootstrap-npm-trust-packages.sh --dry-run
scripts/bootstrap-npm-trust-packages.sh --execute

scripts/publish-npm-packages.sh --dry-run --stage-local
scripts/publish-npm-packages.sh --dry-run
scripts/publish-npm-packages.sh --execute
```

The bootstrap script publishes placeholder `0.0.0-trust.0` packages with the
`bootstrap` dist-tag so npm trusted publishing can be configured before CI
publishes real release binaries. Publish commands use each package's
`publishConfig.registry`, which points at the official npm registry.

The script stages binaries from `dist/npm/` automatically. Staged binaries in
`packages/creditlint-*/bin/` are ignored by Git.

Do not publish `creditlint` as a normal user-facing release until the matching
platform package versions are already available.
