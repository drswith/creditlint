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

## Package Manager

Use pnpm for all package operations.

Do not add lockfiles or project metadata for other package managers unless a
future OpenSpec change requires it.

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

## Pull Requests

Pull requests should describe:

- The OpenSpec task IDs completed.
- Tests run.
- Any behavior that differs from the active spec.

Do not mark an OpenSpec task complete unless the implementation and tests for
that task are actually done.
