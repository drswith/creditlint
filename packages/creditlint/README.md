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

pnpm --filter creditlint-darwin-arm64 publish
pnpm --filter creditlint-darwin-x64 publish
pnpm --filter creditlint-linux-arm64 publish
pnpm --filter creditlint-linux-x64 publish
pnpm --filter creditlint-windows-x64 publish

pnpm --filter creditlint publish
```

Do not publish `creditlint` as a normal user-facing release until the matching
platform package versions are already available.
