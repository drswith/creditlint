## 1. Project Bootstrap

- [x] 1.1 Create Cargo-managed Rust package metadata and `creditlint` CLI bin entry.
- [x] 1.2 Add `rust-toolchain.toml`, Rust formatting/linting/test commands, `just` recipes, and `cargo-nextest` workflow documentation.
- [x] 1.3 Create initial Rust source layout for CLI commands, policy engine, config loading, and reporters.
- [x] 1.4 Add README with problem statement, install placeholder, and first command examples.
- [x] 1.5 Add AGENTS, CONTRIBUTING, and CHANGELOG project entry documents.

## 2. Policy Engine

- [x] 2.1 Define policy, rule, source, and violation Rust types.
- [x] 2.2 Implement built-in default policy for AI/tool authorship and credit markers.
- [x] 2.3 Implement `.creditlint.yml` loading with schema validation and fail-closed invalid-config behavior.
- [x] 2.4 Implement raw message analysis for trailer-like key-value lines and free-form marker lines.
- [x] 2.5 Add unit tests for default rejects, default allows, custom rules, invalid config, and structured violations.
- [x] 2.6 Implement and test forbidden-before-allowed rule precedence.
- [x] 2.7 Implement and test conservative free-form marker matching that avoids normal prose.
- [x] 2.8 Document explicit threat model, bypass assumptions, and Unicode homoglyph out-of-scope behavior.

## 3. CLI Check Commands

- [x] 3.1 Implement `creditlint check --message-file`.
- [x] 3.2 Implement `creditlint check --stdin`.
- [x] 3.3 Implement human-readable and JSON reporters.
- [x] 3.4 Implement stable exit codes `0`, `1`, and `2`.
- [x] 3.5 Add CLI tests for file input, stdin input, output formats, invalid config, and exit codes.

## 4. Git Integration

- [x] 4.1 Implement deterministic Git commit-message collection for `check --range`.
- [x] 4.2 Include commit SHAs in violations from Git range checks.
- [x] 4.3 Implement `audit --all` for all reachable commit messages.
- [x] 4.4 Add temporary-repository integration tests for clean ranges, violating ranges, invalid ranges, and audit mode.
- [x] 4.5 Add an audit performance budget and verify commit metadata is processed incrementally.

## 5. Workflow Integrations

- [x] 5.1 Implement `creditlint init` that writes `.creditlint.yml` without overwriting existing config.
- [x] 5.2 Implement `creditlint install-hook` with stable managed-hook markers and safe handling for existing unmanaged `commit-msg` hooks.
- [x] 5.3 Add GitHub Actions documentation using the native binary or Cargo and full-history checkout for range checks.
- [x] 5.4 Add local Git hook and pre-commit documentation.
- [x] 5.5 Document pull request title/body validation by writing PR text to a file and using `check --message-file`.
- [x] 5.6 Document the default no-telemetry/no-network privacy boundary.

## 6. GitHub Squash Merge Coverage

- [x] 6.1 Implement `creditlint github ruleset-pattern` for policies that can be represented as one safe regex.
- [x] 6.2 Make unsupported ruleset conversion fail closed with a clear explanation.
- [x] 6.3 Document GitHub ruleset metadata restriction setup for final squash commit messages.
- [x] 6.4 Document merge-bot validation with `creditlint check --message-file final-merge-message.txt`.
- [x] 6.5 Document which policy subsets are representable as one GitHub ruleset regex and which require merge-bot validation.

## 7. Release Readiness

- [x] 7.1 Add CI workflow that runs Cargo formatting, linting, tests, and release builds.
- [x] 7.2 Add crates.io and GitHub Release publishing metadata for the `creditlint` package.
- [x] 7.3 Add changelog and versioning guidance.
- [x] 7.4 Add `cross`-based or equivalent cross-platform release build support.
- [x] 7.5 Run OpenSpec validation and update task status before implementation archive.

## 8. Test Hardening

- [x] 8.1 Add end-to-end tests that prove the managed `commit-msg` hook blocks violating commits and allows clean commits.
- [x] 8.2 Add CLI integration tests for repository-boundary behavior such as nested-directory config discovery and non-Git `init`/`install-hook` failures.
- [x] 8.3 Add unit and CLI tests for unsupported `github ruleset-pattern` export branches beyond allowed/forbidden overlap.
- [ ] 8.4 Run Rust and OpenSpec validation again after test hardening and update task status.
