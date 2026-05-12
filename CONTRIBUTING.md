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

Use Cargo for implementation work.

Use pnpm only for OpenSpec commands shown in this document. Do not add npm,
Yarn, or Bun runtime requirements for `creditlint` unless a future OpenSpec
change requires an optional wrapper package.

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
```

Common development commands:

```sh
just check
just fmt
just lint
just test
just ci
```

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
