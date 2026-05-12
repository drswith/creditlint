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

ci:
    cargo fmt --all --check
    cargo clippy --all-targets --all-features -- -D warnings
    cargo nextest run
