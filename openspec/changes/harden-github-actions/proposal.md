## Why

The current GitHub Actions workflows cover the MVP path, but the release path
does not yet enforce the same validation gate as CI and does not publish
checksums for native artifacts. Since `creditlint` is a low-level policy tool,
release automation should be explicit, repeatable, and harder to misuse.

## What Changes

- Add workflow linting to CI.
- Add OpenSpec validation to CI and release validation.
- Add a release validation job that runs before artifact publishing.
- Add workflow concurrency controls.
- Narrow default workflow permissions and grant write access only where needed.
- Make macOS release runner architecture explicit and verify Unix artifact
  architecture.
- Generate and publish release checksums for native artifacts.

## Capabilities

### New Capabilities

- `github-actions-hardening`: CI and release workflow validation, permissions,
  concurrency, artifact architecture checks, and release checksums.

### Modified Capabilities

- None. This change hardens delivery automation without changing `creditlint`
  runtime behavior.

## Impact

- Updates `.github/workflows/ci.yml`.
- Updates `.github/workflows/release.yml`.
- Updates project documentation for the strengthened release path.
- Adds OpenSpec coverage for GitHub Actions hardening.
