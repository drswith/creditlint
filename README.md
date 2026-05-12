# creditlint

`creditlint` is a policy-focused CLI for detecting unauthorized authorship and
credit metadata in Git workflows.

It is designed for projects that allow AI-assisted development but do not want
coding agents, bots, or tools to be silently added as authors through commit
trailers or merge messages.

## Status

This repository is in the OpenSpec planning and bootstrap phase.

Active change:

- `bootstrap-creditlint-mvp`

Current implementation target:

- Implementation stack: Rust native CLI
- Build/package manager: Cargo
- Rust toolchain: stable with rustfmt and clippy
- Task runner: just
- Test runner: cargo-nextest
- OpenSpec command runner: pnpm
- Primary interface: `creditlint`

## Problem

Coding agents can add markers such as:

```text
Co-authored-by: Codex <...>
Made with Cursor
Generated with Claude
```

These markers can create authorship, contribution-credit, and audit risks when
they are added without explicit maintainer approval.

`creditlint` treats authorship and provenance as separate concepts:

- Authorship markers such as `Co-authored-by` affect contribution credit.
- Provenance markers such as `AI-Assisted` or `Tool-Used` can disclose process
  without implying authorship.

## Planned CLI

```sh
creditlint check --message-file .git/COMMIT_EDITMSG
creditlint check --stdin
creditlint check --range origin/main..HEAD
creditlint audit --all
creditlint init
creditlint install-hook
creditlint github ruleset-pattern
```

Planned exit codes:

- `0`: no violations
- `1`: policy violations found
- `2`: invalid invocation, invalid config, or missing required metadata

## Planned Policy File

```yaml
version: 1

rules:
  forbidden_trailers:
    - key: Co-authored-by
      value_pattern: "(?i)(codex|claude|cursor|copilot|openai|anthropic|gemini|ai)"
    - key_pattern: "(?i)made[- ]?(with|on)"
    - key_pattern: "(?i)generated[- ]?with"

  allowed_provenance_trailers:
    - AI-Assisted
    - Tool-Used
    - Generated-by
```

## Governance Model

`creditlint` is intended to run in multiple places:

- Local `commit-msg` hook for fast feedback.
- CI required check for pull-request commits.
- GitHub ruleset metadata restrictions for final protected-branch commit
  messages, especially when squash merge remains enabled.
- Merge-bot validation for controlled final merge messages.
- Pull request title/body checks by writing the PR text to a file and running
  `creditlint check --message-file`.

CI range checks are useful, but they do not by themselves guarantee validation
of a final squash merge message edited by the hosting platform UI.

## Privacy

The planned CLI is local-first. By default, `creditlint` should not upload commit
messages, pull request text, or policy files to any hosted service.

## Development

Use Cargo for implementation work. The OpenSpec CLI is currently invoked through
`pnpm dlx`, but consuming projects should not need Node.js or pnpm to run
`creditlint`.

Planned Rust tooling:

- `rust-toolchain.toml` pins stable Rust with `rustfmt` and `clippy`.
- `just` provides short project commands.
- `cargo-nextest` is the preferred test runner.
- `cargo-watch` is optional for local edit/test loops.
- `cross` is reserved for release builds.

OpenSpec commands:

```sh
pnpm dlx @fission-ai/openspec list
pnpm dlx @fission-ai/openspec validate --all
pnpm dlx @fission-ai/openspec show bootstrap-creditlint-mvp
```

Implementation work should follow:

```text
openspec/changes/bootstrap-creditlint-mvp/tasks.md
```

## License

MIT
