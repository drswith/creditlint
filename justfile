default:
    @just --list

check:
    cargo check --all-targets

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all --check

lint:
    cargo clippy --all-targets --all-features -- -D warnings

test:
    cargo nextest run

test-unit:
    cargo test

test-npm:
    pnpm --filter creditlint test

openspec-validate:
    pnpm dlx @fission-ai/openspec validate --all

ci:
    cargo fmt --all --check
    cargo clippy --all-targets --all-features -- -D warnings
    cargo nextest run
    pnpm --filter creditlint test
    pnpm dlx @fission-ai/openspec validate --all

release-build:
    cargo build --release

cross-build target:
    cross build --release --target {{target}}

npm-publish-dry-run:
    ./scripts/publish-npm-packages.sh --dry-run

npm-publish-local-dry-run:
    ./scripts/publish-npm-packages.sh --dry-run --stage-local

npm-publish:
    ./scripts/publish-npm-packages.sh --execute

npm-trust-bootstrap-dry-run:
    ./scripts/bootstrap-npm-trust-packages.sh --dry-run

npm-trust-bootstrap:
    ./scripts/bootstrap-npm-trust-packages.sh --execute
