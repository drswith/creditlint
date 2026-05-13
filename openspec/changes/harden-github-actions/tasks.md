## 1. CI Workflow

- [x] 1.1 Add workflow linting to CI.
- [x] 1.2 Add OpenSpec validation to CI.
- [x] 1.3 Add CI permissions and concurrency controls.

## 2. Release Workflow

- [x] 2.1 Add a release validation job for formatting, clippy, tests, and OpenSpec.
- [x] 2.2 Make release artifact jobs depend on validation.
- [x] 2.3 Scope release workflow permissions to read by default and write only where needed.
- [x] 2.4 Use explicit macOS runner labels and verify Unix binary architectures.
- [x] 2.5 Generate, upload, and tag-publish a combined `SHA256SUMS` file.
- [x] 2.6 Add release workflow concurrency controls.

## 3. Documentation and Validation

- [x] 3.1 Update README/CONTRIBUTING delivery documentation.
- [x] 3.2 Run Rust, OpenSpec, and workflow validation where available.
