# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog, and this project intends to follow
semantic versioning after the first release.

## Unreleased

No unreleased changes.

## 0.1.2 - 2026-05-14

### Changed

- Refined README positioning, badges, and Star History presentation.
- Added GitHub Releases, crates.io, and npm download badges.

## 0.1.1 - 2026-05-13

### Added

- Initialized OpenSpec planning for the `creditlint` MVP.
- Added project README and contributor guidance.
- Added Rust CLI commands for message checks, Git range audits, config init,
  managed hook installation, and conservative GitHub ruleset export.
- Added CI workflow scaffolding and release metadata for native packaging.
- Added release-binary smoke coverage and a delivery workflow for native
  release artifacts.
- Added crates.io publishing support to the release workflow via
  `CARGO_REGISTRY_TOKEN`.

## Versioning Guidance

- Keep unreleased work in the `Unreleased` section until a tag is cut.
- Move shipped entries into a versioned heading such as `## 0.1.0 - 2026-05-12`
  at release time.
- Prefer SemVer once the first public release is published:
  - patch for bug fixes and non-breaking behavior corrections
  - minor for backward-compatible features
  - major for breaking CLI, config, or policy-contract changes
