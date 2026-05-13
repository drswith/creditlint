#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  scripts/bootstrap-npm-trust-packages.sh --dry-run
  scripts/bootstrap-npm-trust-packages.sh --execute

Publishes placeholder npm packages so npm trusted publishing can be configured
before CI publishes real native binaries.

The placeholder version is 0.0.0-trust.0 and the dist-tag is bootstrap. This
script does not publish a usable creditlint release and does not require native
binaries.

Options:
  --dry-run   Run npm publish --dry-run from generated placeholder packages.
  --execute   Actually publish placeholder packages to npm with tag bootstrap.
  -h, --help  Show this help.
USAGE
}

mode=""
bootstrap_version="0.0.0-trust.0"
tag="bootstrap"

while [ "$#" -gt 0 ]; do
  case "$1" in
    --dry-run)
      mode="dry-run"
      ;;
    --execute)
      mode="execute"
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

packages=(
  "creditlint-darwin-arm64"
  "creditlint-darwin-x64"
  "creditlint-linux-arm64"
  "creditlint-linux-x64"
  "creditlint-windows-x64"
  "creditlint"
)

tmp_dir="$(mktemp -d "${TMPDIR:-/tmp}/creditlint-npm-bootstrap.XXXXXX")"
cleanup() {
  rm -rf "$tmp_dir"
}
trap cleanup EXIT

node - "$repo_root" "$tmp_dir" "$bootstrap_version" <<'NODE'
const fs = require("node:fs");
const path = require("node:path");

const repoRoot = process.argv[2];
const tmpDir = process.argv[3];
const version = process.argv[4];

const packages = [
  "creditlint-darwin-arm64",
  "creditlint-darwin-x64",
  "creditlint-linux-arm64",
  "creditlint-linux-x64",
  "creditlint-windows-x64",
  "creditlint",
];

for (const packageName of packages) {
  const sourceDir = path.join(repoRoot, "packages", packageName);
  const targetDir = path.join(tmpDir, packageName);
  fs.mkdirSync(targetDir, { recursive: true });

  const packageJson = JSON.parse(fs.readFileSync(path.join(sourceDir, "package.json"), "utf8"));
  packageJson.version = version;
  delete packageJson.scripts;

  if (packageName === "creditlint") {
    packageJson.optionalDependencies = Object.fromEntries(
      Object.keys(packageJson.optionalDependencies ?? {}).map((name) => [name, version]),
    );
    packageJson.bin = { creditlint: "bin/creditlint.js" };
    packageJson.files = ["bin/", "README.md"];
    fs.mkdirSync(path.join(targetDir, "bin"), { recursive: true });
    fs.writeFileSync(
      path.join(targetDir, "bin", "creditlint.js"),
      [
        "#!/usr/bin/env node",
        "console.error('creditlint 0.0.0-trust.0 is a bootstrap placeholder for npm trusted publishing. Install a real release version instead.');",
        "process.exit(2);",
        "",
      ].join("\n"),
    );
    fs.chmodSync(path.join(targetDir, "bin", "creditlint.js"), 0o755);
  } else {
    packageJson.files = ["README.md"];
  }

  fs.writeFileSync(
    path.join(targetDir, "package.json"),
    `${JSON.stringify(packageJson, null, 2)}\n`,
  );

  const readmePath = path.join(sourceDir, "README.md");
  if (fs.existsSync(readmePath)) {
    fs.copyFileSync(readmePath, path.join(targetDir, "README.md"));
  } else {
    fs.writeFileSync(path.join(targetDir, "README.md"), `# ${packageName}\n`);
  }
}
NODE

publish_args=(--tag "$tag")
if [ "$mode" = "dry-run" ]; then
  publish_args+=(--dry-run)
fi

for package in "${packages[@]}"; do
  echo "publishing bootstrap placeholder $package@$bootstrap_version (${mode})"
  (cd "$tmp_dir/$package" && npm publish "${publish_args[@]}")
done

cat <<EOF

Bootstrap packages published with tag '$tag'.

Next, configure npm trusted publishing for each package, then let CI publish the
real release version. Do not promote $bootstrap_version to latest.
EOF
