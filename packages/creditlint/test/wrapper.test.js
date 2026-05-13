"use strict";

const assert = require("node:assert/strict");
const { spawnSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const wrapper = path.resolve(__dirname, "..", "bin", "creditlint.js");

function makeFakeBinary() {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "creditlint-wrapper-"));
  const fakeBinary = path.join(tempDir, "creditlint-fake");

  fs.writeFileSync(
    fakeBinary,
    [
      "#!/usr/bin/env node",
      "const args = process.argv.slice(2);",
      "console.log(JSON.stringify(args));",
      "process.exit(args.includes('--fail') ? 7 : 0);",
      "",
    ].join("\n"),
  );
  fs.chmodSync(fakeBinary, 0o755);

  return { fakeBinary, tempDir };
}

test("CREDITLINT_BIN override receives user arguments", () => {
  const { fakeBinary, tempDir } = makeFakeBinary();

  try {
    const result = spawnSync(process.execPath, [wrapper, "check", "--stdin"], {
      env: { ...process.env, CREDITLINT_BIN: fakeBinary },
      encoding: "utf8",
    });

    assert.equal(result.status, 0);
    assert.match(result.stdout, /\["check","--stdin"\]/);
  } finally {
    fs.rmSync(tempDir, { recursive: true, force: true });
  }
});

test("native exit code is forwarded", () => {
  const { fakeBinary, tempDir } = makeFakeBinary();

  try {
    const result = spawnSync(process.execPath, [wrapper, "--fail"], {
      env: { ...process.env, CREDITLINT_BIN: fakeBinary },
      encoding: "utf8",
    });

    assert.equal(result.status, 7);
  } finally {
    fs.rmSync(tempDir, { recursive: true, force: true });
  }
});

test("missing native binary exits with code 2", () => {
  const missingBinary = path.join(os.tmpdir(), "creditlint-missing-native");
  const result = spawnSync(process.execPath, [wrapper, "--version"], {
    env: { ...process.env, CREDITLINT_BIN: missingBinary },
    encoding: "utf8",
  });

  assert.equal(result.status, 2);
  assert.match(result.stderr, /could not find a native creditlint binary/);
  assert.match(result.stderr, /CREDITLINT_BIN/);
});
