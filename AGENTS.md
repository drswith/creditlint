# AGENTS.md

Guidance for coding agents working in this repository.

## Project

`creditlint` is a Rust native CLI for enforcing Git credit and authorship
metadata policy.

The product boundary is narrow:

- Detect and report unauthorized authorship or credit metadata.
- Support Git hooks, CI checks, repository rulesets, and merge-bot validation.
- Do not claim to detect whether code was AI-generated.
- Do not act as a legal compliance engine.

## Workflow

Use OpenSpec as the source of truth for planned behavior.

Before implementing a feature:

1. Read `openspec/config.yaml`.
2. Read the active change under `openspec/changes/`.
3. Read the relevant `specs/*/spec.md` files.
4. Implement only tasks listed in `tasks.md`.
5. Mark completed tasks by changing `- [ ]` to `- [x]`.
6. Run OpenSpec validation before closing the work.

Current active changes:

- `openspec/changes/bootstrap-creditlint-mvp/`
- `openspec/changes/add-npm-wrapper-package/`

Useful commands:

```sh
pnpm dlx @fission-ai/openspec list
pnpm dlx @fission-ai/openspec validate --all
pnpm dlx @fission-ai/openspec status --change bootstrap-creditlint-mvp --json
pnpm dlx @fission-ai/openspec status --change add-npm-wrapper-package --json
```

## Tooling

Use Cargo for Rust implementation work. Use pnpm for OpenSpec commands and for
the optional npm wrapper package under `packages/creditlint`.

Do not introduce Yarn or Bun runtime requirements for `creditlint`. Do not make
Node.js required for users who consume the native binary, crates.io crate, or
GitHub Release artifacts.

Expected Rust tooling:

- Use the checked-in `rust-toolchain.toml`.
- Prefer `just` recipes once a `justfile` exists.
- Prefer `cargo nextest run` for tests once configured.
- Use `pnpm --filter creditlint test` for the optional npm wrapper package.
- Treat `cargo-watch` as optional local convenience.
- Use `cross` only for release packaging tasks.
- Do not make `bacon` or `cargo-edit` required project tooling.

## Implementation Conventions

- Keep the Rust policy engine independent from CLI command handlers.
- Prefer structured violation objects before formatting terminal output.
- Preserve fail-closed behavior for invalid config, unreadable input, and failed
  Git metadata collection.
- Keep GitHub squash merge limitations explicit in docs and code comments.
- Use deterministic tests for policy behavior before adding workflow glue.
- Keep npm releases usable without requiring Rust or Cargo by resolving native
  binaries from platform optional packages before local development fallbacks.

## Files

- `README.md`: public project overview.
- `openspec/config.yaml`: project context for OpenSpec.
- `openspec/changes/bootstrap-creditlint-mvp/proposal.md`: why this change
  exists.
- `openspec/changes/bootstrap-creditlint-mvp/design.md`: technical approach.
- `openspec/changes/bootstrap-creditlint-mvp/specs/`: required behavior.
- `openspec/changes/bootstrap-creditlint-mvp/tasks.md`: implementation checklist.

## Boundaries

- Do not rewrite Git history unless explicitly asked.
- Do not bypass checks to make a merge succeed.
- Do not remove OpenSpec artifacts when implementation starts.
- Do not add generated attribution such as `Co-authored-by`, `Made with`, or
  similar markers to commits or pull requests unless the user explicitly asks.
