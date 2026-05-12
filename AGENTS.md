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

Current active change:

- `openspec/changes/bootstrap-creditlint-mvp/`

Useful commands:

```sh
pnpm dlx @fission-ai/openspec list
pnpm dlx @fission-ai/openspec validate --all
pnpm dlx @fission-ai/openspec status --change bootstrap-creditlint-mvp --json
```

## Tooling

Use Cargo for implementation work. Use `pnpm dlx @fission-ai/openspec` only for
OpenSpec commands until the repository provides a pinned OpenSpec runner.

Do not introduce npm, Yarn, or Bun runtime requirements for `creditlint` unless a
future OpenSpec change explicitly calls for an optional wrapper package.

Expected Rust tooling:

- Use the checked-in `rust-toolchain.toml`.
- Prefer `just` recipes once a `justfile` exists.
- Prefer `cargo nextest run` for tests once configured.
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
