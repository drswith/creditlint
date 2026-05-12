# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog, and this project intends to follow
semantic versioning after the first release.

## Unreleased

### Added

- Initialized OpenSpec planning for the `creditlint` MVP.
- Added project README and contributor guidance.
- Added Rust CLI commands for message checks, Git range audits, config init,
  managed hook installation, and conservative GitHub ruleset export.
- Added CI workflow scaffolding and release metadata for native packaging.

## Versioning Guidance

- Keep unreleased work in the `Unreleased` section until a tag is cut.
- Move shipped entries into a versioned heading such as `## 0.1.0 - 2026-05-12`
  at release time.
- Prefer SemVer once the first public release is published:
  - patch for bug fixes and non-breaking behavior corrections
  - minor for backward-compatible features
  - major for breaking CLI, config, or policy-contract changes
