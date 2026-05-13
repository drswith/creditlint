# Contributing

`creditlint` uses OpenSpec for specification-driven development.

## Before Coding

Read the active change:

```sh
pnpm dlx @fission-ai/openspec show bootstrap-creditlint-mvp
```

Validate the current OpenSpec state:

```sh
pnpm dlx @fission-ai/openspec validate --all
```

Implementation should follow:

```text
openspec/changes/bootstrap-creditlint-mvp/tasks.md
```

## Tooling

Use Cargo for Rust implementation work.

Use pnpm for OpenSpec commands and for the optional npm wrapper package under
`packages/creditlint`. Do not add Yarn or Bun runtime requirements for `creditlint`.
Do not make Node.js required for users who consume the native binary, crates.io
crate, or GitHub Release artifacts.

Expected Rust tooling:

- Rust stable from `rust-toolchain.toml`
- `rustfmt` and `clippy`
- `just` for project command shortcuts once available
- `cargo-nextest` for the preferred test workflow once configured

Optional local tools:

- `cargo-watch` for edit/test loops
- `bacon` if you personally prefer it
- `cargo-edit` for local dependency editing

Release-only tooling:

- `cross` or an equivalent cross-platform release builder

Recommended local setup:

```sh
cargo install just
cargo install cargo-nextest
cargo install cross
```

Common development commands:

```sh
just check
just fmt
just lint
just test
just test-npm
just ci
just release-build
just cross-build x86_64-unknown-linux-gnu
```

Optional npm wrapper commands:

```sh
pnpm install
pnpm --filter creditlint test
cargo build
CREDITLINT_BIN="$PWD/target/debug/creditlint" pnpm --filter creditlint run creditlint --help
```

The npm wrapper should remain usable by frontend and JavaScript-focused users
without a Rust toolchain. Publishable npm releases must provide native binaries
through platform optional packages under `packages/creditlint-*`; `CREDITLINT_BIN`
and Cargo build output fallback paths are for tests and repository development.

## Commit Metadata

This project is specifically about credit and authorship metadata. Contributors
should avoid adding tool-authorship markers such as:

```text
Co-authored-by: Codex <...>
Made with ...
Generated with ...
```

If process disclosure is needed, prefer explicit provenance metadata documented
by the project rather than authorship trailers.

## Threat Model Notes

The current MVP is intended to stop default or accidental credit/authorship
markers in normal Git workflows.

It is not yet intended to fully defeat deliberate evasion techniques such as:

- Unicode homoglyph spoofing
- Obfuscated or split markers
- Administrator bypass of repository rules
- Direct protected-branch writes

## Pull Requests

Pull requests should describe:

- The OpenSpec task IDs completed.
- Tests run.
- Any behavior that differs from the active spec.

Do not mark an OpenSpec task complete unless the implementation and tests for
that task are actually done.

## GitHub Actions Notes

When documenting or updating CI examples:

- Use Cargo or a native binary artifact path. Do not introduce Node-based
  wrappers as the default integration path.
- Keep npm wrapper validation separate from Rust-native usage examples.
- Use full-history checkout for range checks:

```yaml
- uses: actions/checkout@v4
  with:
    fetch-depth: 0
```

- Keep the range check explicit, for example:

```sh
./target/release/creditlint check --range origin/${{ github.base_ref }}..HEAD
```

Shallow fetches can make `check --range` fail because the base commit is not
available locally.

## Delivery Notes

The repository is now in delivery preparation, not just bootstrap development.

Delivery expectations:

- CI validates workflow syntax, source builds, tests, OpenSpec artifacts, and
  release-binary smoke execution.
- The release workflow builds native artifacts for Linux, macOS, and Windows.
- Tagged releases should publish those binaries and `SHA256SUMS` as GitHub
  Release assets.
- Tagged releases should publish the crate to crates.io when
  `CARGO_REGISTRY_TOKEN` is configured.
- Users should not need a Rust toolchain when consuming prebuilt binaries.

Required delivery configuration:

- job-scoped `permissions: contents: write` for GitHub Release asset upload
- repository secret `CARGO_REGISTRY_TOKEN` for `cargo publish`

## Local Hook Notes

Prefer the managed hook path for local testing:

```sh
creditlint init
creditlint install-hook
```

The managed installer is intentionally conservative:

- It writes a `commit-msg` hook that runs `creditlint check --message-file`.
- It replaces only hooks that already contain the stable `creditlint managed
  hook` marker and version field.
- It refuses to overwrite unmanaged hooks.

When a repository already owns its `commit-msg` hook, document manual
integration by adding this command to the existing hook:

```sh
creditlint check --message-file "$1"
```

## Pull Request Text Notes

Document pull request title/body validation as a separate check surface from
commit ranges.

Recommended pattern:

```sh
printf '%s\n\n%s\n' "$PR_TITLE" "$PR_BODY" > /tmp/creditlint-pr-message.txt
creditlint check --message-file /tmp/creditlint-pr-message.txt
```

This is especially important for squash-merge workflows where platform-generated
final commit messages can inherit pull request text.

## Privacy Notes

Keep the default privacy boundary explicit in docs and reviews:

- No hosted service is required for normal CLI use.
- Commit messages, pull request text, and policy files stay local by default.
- Network access is not part of the default evaluation flow.

Do not casually add telemetry, background syncing, or remote policy lookups
without a separate OpenSpec change.

## GitHub Ruleset Notes

When documenting GitHub squash-merge protection, distinguish these layers:

- `creditlint check --range` validates pull request commit messages.
- `creditlint github ruleset-pattern` exports a conservative regex for GitHub
  commit-message metadata restrictions.
- The GitHub ruleset is the platform-side control for the final squash message.

Do not describe a pull request range check as if it guarantees the final squash
commit message produced or edited in the GitHub UI.

## Merge Bot Notes

When a repository uses a controlled merge bot, document final-message
validation explicitly:

```sh
creditlint check --message-file final-merge-message.txt
```

The merge bot should run this against the exact message it is about to write,
then fail closed on exit code `1` or `2`.

## Ruleset Export Notes

Keep the documented GitHub ruleset export boundary aligned with the current
implementation.

Supported subset:

- exact forbidden trailer keys
- trailer value exact strings, unanchored regexes, or `Any`
- free-form rules expressed as one anchored line regex
- no overlap between allowed provenance keys and forbidden trailer keys

Unsupported subset:

- precedence-sensitive overlap on the same trailer key
- regex-matched trailer field names
- non-anchored free-form prose matching
- normalization or multi-pass logic

## Versioning And Changelog Notes

Until the first public release, keep release notes in `CHANGELOG.md` under
`Unreleased`.

When cutting a release:

- move shipped items into a dated version heading
- bump `Cargo.toml` version in the same change
- keep SemVer discipline for CLI behavior, config schema, and output contracts
