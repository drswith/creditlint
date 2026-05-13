#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  scripts/publish-npm-packages.sh --dry-run
  scripts/publish-npm-packages.sh --execute [--access public]

Publishes npm platform packages first, then the main creditlint wrapper package.

Options:
  --dry-run        Validate and run pnpm publish --dry-run for every package.
  --execute        Actually publish packages to npm.
  --access public  Pass --access public to pnpm publish. Usually only needed for scoped packages.
  -h, --help       Show this help.

Required native binaries before --execute:
  packages/creditlint-darwin-arm64/bin/creditlint
  packages/creditlint-darwin-x64/bin/creditlint
  packages/creditlint-linux-arm64/bin/creditlint
  packages/creditlint-linux-x64/bin/creditlint
  packages/creditlint-windows-x64/bin/creditlint.exe
USAGE
}

mode=""
access_args=()

while [ "$#" -gt 0 ]; do
  case "$1" in
    --dry-run)
      mode="dry-run"
      ;;
    --execute)
      mode="execute"
      ;;
    --access)
      if [ "${2:-}" != "public" ]; then
        echo "error: only --access public is supported" >&2
        exit 2
      fi
      access_args=(--access public)
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "error: unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
  shift
done

if [ -z "$mode" ]; then
  echo "error: choose --dry-run or --execute" >&2
  usage >&2
  exit 2
fi

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

required_binaries=(
  "packages/creditlint-darwin-arm64/bin/creditlint"
  "packages/creditlint-darwin-x64/bin/creditlint"
  "packages/creditlint-linux-arm64/bin/creditlint"
  "packages/creditlint-linux-x64/bin/creditlint"
  "packages/creditlint-windows-x64/bin/creditlint.exe"
)

packages=(
  "creditlint-darwin-arm64"
  "creditlint-darwin-x64"
  "creditlint-linux-arm64"
  "creditlint-linux-x64"
  "creditlint-windows-x64"
  "creditlint"
)

missing=0
for binary in "${required_binaries[@]}"; do
  if [ ! -f "$binary" ]; then
    echo "missing native binary: $binary" >&2
    missing=1
  fi
done

if [ "$missing" -ne 0 ]; then
  echo "error: stage all native binaries before publishing npm packages" >&2
  exit 2
fi

pnpm install --frozen-lockfile
pnpm --filter creditlint test

publish_args=()
if [ "$mode" = "dry-run" ]; then
  publish_args+=(--dry-run)
fi
if [ "${#access_args[@]}" -gt 0 ]; then
  publish_args+=("${access_args[@]}")
fi

for package in "${packages[@]}"; do
  echo "publishing $package (${mode})"
  pnpm --filter "$package" publish --no-git-checks "${publish_args[@]}"
done
