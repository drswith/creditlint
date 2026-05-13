# creditlint npm wrapper

This package provides the optional npm `creditlint` command for teams that
install developer tools through npm, pnpm, or npx.

The implementation is a thin wrapper around the native Rust `creditlint` CLI.
It does not reimplement policy parsing or Git metadata checks in JavaScript.

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
under `native/`, then falls back to repository-local Cargo build outputs.
