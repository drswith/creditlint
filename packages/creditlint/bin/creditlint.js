#!/usr/bin/env node
"use strict";

const { spawnSync } = require("node:child_process");
const fs = require("node:fs");
const path = require("node:path");

const PLATFORM_PACKAGES = {
  "darwin-arm64": "creditlint-darwin-arm64",
  "darwin-x64": "creditlint-darwin-x64",
  "linux-arm64": "creditlint-linux-arm64",
  "linux-x64": "creditlint-linux-x64",
  "win32-x64": "creditlint-windows-x64",
};

function executableName(platform = process.platform) {
  return platform === "win32" ? "creditlint.exe" : "creditlint";
}

function platformKey(platform = process.platform, arch = process.arch) {
  return `${platform}-${arch}`;
}

function platformPackageName(platform = process.platform, arch = process.arch) {
  return PLATFORM_PACKAGES[platformKey(platform, arch)];
}

function packageRoot(packageName, baseDir = __dirname) {
  try {
    return path.dirname(require.resolve(`${packageName}/package.json`, { paths: [baseDir] }));
  } catch {
    return undefined;
  }
}

function candidateBinaries(options = {}) {
  const platform = options.platform ?? process.platform;
  const arch = options.arch ?? process.arch;
  const baseDir = options.baseDir ?? __dirname;
  const env = options.env ?? process.env;
  const repoRoot = options.repoRoot ?? path.resolve(baseDir, "..", "..", "..");
  const binaryName = executableName(platform);
  const candidates = [];

  if (env.CREDITLINT_BIN) {
    candidates.push({
      kind: "env",
      path: env.CREDITLINT_BIN,
    });
    return candidates;
  }

  const packageName = platformPackageName(platform, arch);
  if (packageName) {
    const root = packageRoot(packageName, baseDir);
    if (root) {
      candidates.push({
        kind: "platform-package",
        path: path.join(root, "bin", binaryName),
        packageName,
      });
    }
  }

  candidates.push(
    {
      kind: "package-native",
      path: path.resolve(baseDir, "..", "native", binaryName),
    },
    {
      kind: "cargo-release",
      path: path.resolve(repoRoot, "target", "release", binaryName),
    },
    {
      kind: "cargo-debug",
      path: path.resolve(repoRoot, "target", "debug", binaryName),
    },
  );

  return candidates;
}

function resolveBinary(options = {}) {
  const candidates = candidateBinaries(options);
  const binary = candidates.find((candidate) => fs.existsSync(candidate.path));

  if (binary) {
    return { binary, candidates };
  }

  const platform = options.platform ?? process.platform;
  const arch = options.arch ?? process.arch;
  const key = platformKey(platform, arch);
  const packageName = platformPackageName(platform, arch);

  if (!packageName && !(options.env ?? process.env).CREDITLINT_BIN) {
    return {
      error: `unsupported platform: ${key}`,
      candidates,
      platformKey: key,
    };
  }

  return {
    error: packageName
      ? `missing native binary for ${key}; optional package ${packageName} may not be installed`
      : "missing native binary",
    candidates,
    platformKey: key,
    packageName,
  };
}

function main() {
  const resolved = resolveBinary();

  if (resolved.error) {
    console.error("creditlint npm wrapper could not find a native creditlint binary.");
    console.error(resolved.error);
    console.error("Install the supported optional native package, or set CREDITLINT_BIN=/path/to/creditlint.");
    console.error(`Checked: ${resolved.candidates.map((candidate) => candidate.path).join(", ")}`);
    process.exit(2);
  }

  const child = spawnSync(resolved.binary.path, process.argv.slice(2), {
    stdio: "inherit",
  });

  if (child.error) {
    console.error(`creditlint npm wrapper could not execute native binary: ${child.error.message}`);
    process.exit(2);
  }

  if (child.signal) {
    process.kill(process.pid, child.signal);
  }

  process.exit(child.status ?? 1);
}

module.exports = {
  PLATFORM_PACKAGES,
  candidateBinaries,
  executableName,
  platformKey,
  platformPackageName,
  resolveBinary,
};

if (require.main === module) {
  main();
}
