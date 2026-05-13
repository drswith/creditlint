#!/usr/bin/env node
"use strict";

const { spawnSync } = require("node:child_process");
const fs = require("node:fs");
const path = require("node:path");

const executableName = process.platform === "win32" ? "creditlint.exe" : "creditlint";

function candidateBinaries() {
  if (process.env.CREDITLINT_BIN) {
    return [process.env.CREDITLINT_BIN];
  }

  return [
    path.resolve(__dirname, "..", "native", executableName),
    path.resolve(__dirname, "..", "..", "..", "target", "release", executableName),
    path.resolve(__dirname, "..", "..", "..", "target", "debug", executableName),
  ];
}

const candidates = candidateBinaries();
const binary = candidates.find((candidate) => fs.existsSync(candidate));

if (!binary) {
  console.error("creditlint npm wrapper could not find a native creditlint binary.");
  console.error("Set CREDITLINT_BIN=/path/to/creditlint or install a package that includes native binaries.");
  console.error(`Checked: ${candidates.join(", ")}`);
  process.exit(2);
}

const child = spawnSync(binary, process.argv.slice(2), {
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
