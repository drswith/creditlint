[English](README.md) | [简体中文](README.zh-CN.md)

# creditlint

[![CI](https://github.com/Drswith/creditlint/actions/workflows/ci.yml/badge.svg)](https://github.com/Drswith/creditlint/actions/workflows/ci.yml) [![许可证：MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE) [![Rust](https://img.shields.io/badge/Rust-native-orange.svg)](https://www.rust-lang.org/) [![OpenSpec](https://img.shields.io/badge/OpenSpec-spec--driven-2f6fdd.svg)](openspec/config.yaml) [![CLI](https://img.shields.io/badge/CLI-creditlint-0f766e.svg)](README.zh-CN.md)

[![下载量](https://img.shields.io/github/downloads/Drswith/creditlint/total.svg)](https://github.com/Drswith/creditlint/releases) [![crates.io](https://img.shields.io/crates/d/creditlint.svg?label=crates.io)](https://crates.io/crates/creditlint) [![npm](https://img.shields.io/npm/dt/creditlint.svg?label=npm)](https://www.npmjs.com/package/creditlint)

`creditlint` 是一个 Rust 原生命令行工具，用于在不需要的贡献归属标记进入项目历史之前强制执行 Git 署名和贡献元数据策略。

它面向使用 AI 辅助开发的团队，但不希望工具、代理或托管工作流悄悄把 `Co-authored-by`、`Made with` 或生成工具尾注这类类似署名的元数据写入提交、拉取请求和合并消息。

`creditlint` 不尝试检测代码是否由 AI 生成，也不是法律合规引擎。

## 检查范围

`creditlint` 会把仓库策略应用到可能出现贡献归属的 Git 元数据表面：

- 提交消息文本
- 通过 `--message-file` 传入的拉取请求标题/正文文本
- Git 作者姓名/邮箱
- Git 提交者姓名/邮箱
- 合并机器人传给 CLI 的最终合并或压缩提交消息

它刻意区分两个经常被混淆的概念：

- 署名和贡献归属标记受策略控制。
- 溯源标记可以被允许，但不等同于署名。

## 为什么需要

编码代理、机器人、IDE 和托管工作流可能会写入改变 Git 历史中署名或贡献归属方式的元数据：

```text
Co-authored-by: Codex <codex@example.com>
Made with Cursor
Generated with Claude
Author: Cursor Agent <cursoragent@cursor.com>
```

这些元数据对审计和溯源可能有用。问题在于：如果它们被写进暗示署名或贡献归属的字段，却没有经过明确的项目策略控制，就会改变项目历史的含义。

`creditlint` 的边界很窄：它验证元数据放置位置和策略，不判断代码来源。

## 当前状态

MVP CLI 已实现。仓库正在为公开包和发布渠道做交付准备。

已实现的功能：

- `creditlint check --message-file`
- `creditlint check --stdin`
- `creditlint check --range`
- `creditlint audit --all`
- `creditlint init`
- `creditlint install-hook`
- `creditlint github ruleset-pattern`
- 人类可读和 JSON 输出
- Rust 原生发布产物
- 可选的 npm 包装器，支持平台包解析

计划的公开分发渠道：

- crates.io 上 Rust CLI 的包元数据
- GitHub Release 的预构建原生二进制文件
- 可选的 npm 包，在本地回退前解析原生二进制文件

## 安装

在公共包发布前，从本仓库安装：

```sh
cargo install --path .
creditlint --help
```

本地开发：

```sh
cargo build
./target/debug/creditlint --help
```

在公共包发布后，用户应优先选择以下方式之一：

- crates.io 上的 `creditlint` crate
- GitHub Releases 的预构建原生二进制文件
- 可选的 npm 包，适用于已通过 npm、pnpm 或 npx 安装开发工具的团队

原生命令行工具在消费仓库中不需要 Node.js、pnpm 或 npm。

## 快速开始

在 Git 仓库中创建默认策略文件：

```sh
creditlint init
```

安装托管的 `commit-msg` 钩子：

```sh
creditlint install-hook
```

检查单条消息：

```sh
creditlint check --message-file .git/COMMIT_EDITMSG
printf 'Made with Cursor\n' | creditlint check --stdin
```

在 CI 中检查拉取请求提交：

```sh
creditlint check --range origin/main..HEAD
```

审计所有可达的 Git 历史：

```sh
creditlint audit --all
```

为最终压缩提交消息生成保守的 GitHub 规则集正则表达式：

```sh
creditlint github ruleset-pattern
```

使用 JSON 输出进行自动化：

```sh
creditlint check --range origin/main..HEAD --format json
```

退出代码：

- `0`：无违规
- `1`：发现策略违规
- `2`：调用无效、配置无效、输入不可读或元数据收集失败

## 策略

如果没有 `.creditlint.yml`，`creditlint` 使用内置默认策略。默认策略会阻止常见的 AI/工具署名和贡献归属标记，同时允许显式的溯源尾注。

策略文件示例：

```yaml
version: 1

rules:
  forbidden_identities:
    - name_pattern: "(?i)(cursor agent|codex|claude|copilot|openai|anthropic|gemini)"
      email_pattern: "(?i)(cursoragent@cursor\\.com|codex|claude|copilot|openai|anthropic|gemini)"

  forbidden_trailers:
    - key: Co-authored-by
      value_pattern: "(?i)(codex|claude|cursor|copilot|openai|anthropic|gemini|ai)"
    - key_pattern: "(?i)^made[- ]with\\b.*$"
    - key_pattern: "(?i)^made[- ]on\\b.*$"
    - key_pattern: "(?i)^generated[- ]with\\b.*$"

  allowed_provenance_trailers:
    - AI-Assisted
    - Tool-Used
    - Generated-by
```

策略评估涵盖：

- 提交消息文本
- 通过 `--message-file` 传递的拉取请求标题/正文文本
- Git 作者姓名/邮箱
- Git 提交者姓名/邮箱

禁止规则优先于允许的溯源键。无效配置会导致失败关闭，退出代码为 `2`。

## 执行模型

`creditlint` 应该在多个层级运行，因为没有单个钩子或 CI 任务能看到所有 Git 元数据表面。

推荐层级：

- 本地 `commit-msg` 钩子用于快速反馈。
- CI 必需检查用于拉取请求提交。
- 通过将 PR 文本写入临时文件并运行 `creditlint check --message-file` 来检查拉取请求标题/正文。
- 当压缩合并仍然启用时，GitHub 规则集元数据限制用于最终受保护分支提交消息。
- 当仓库控制最终合并消息时，合并机器人验证。

边界很重要：`creditlint check --range` 验证提议的提交。它本身不验证由托管平台 UI 编辑或合成的最终压缩合并消息。

有关在其他仓库中的更强部署指导，请使用仓库本地技能：

```text
skills/enforcement-rollout/SKILL.md
```

可复制的代理提示：

```text
使用 creditlint 仓库技能 `enforcement-rollout` 并帮助我在此仓库中部署 creditlint 以实现最实际的拦截。检查已覆盖的内容，识别本地钩子、CI 提交检查、PR 标题/正文检查、最终压缩/合并消息执行和仓库设置中的剩余差距，然后给我一个确切的部署计划。
```

## GitHub Actions

范围检查需要足够的 Git 历史来解析基础修订。使用 `fetch-depth: 0` 或等效的获取策略。

```yaml
name: creditlint

on:
  pull_request:
  push:
    branches:
      - main

jobs:
  creditlint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - uses: dtolnay/rust-toolchain@stable

      - name: Build creditlint
        run: cargo build --release

      - name: Check pull request commits
        if: github.event_name == 'pull_request'
        run: |
          ./target/release/creditlint check \
            --range origin/${{ github.base_ref }}..HEAD

      - name: Audit full history on main
        if: github.event_name == 'push'
        run: ./target/release/creditlint audit --all
```

要验证拉取请求标题/正文文本，请将其写入文件并运行相同的策略引擎：

```sh
printf '%s\n\n%s\n' "$PR_TITLE" "$PR_BODY" > /tmp/creditlint-pr-message.txt
creditlint check --message-file /tmp/creditlint-pr-message.txt
```

## GitHub 规则集和合并机器人

当活动策略可以安全地表示为一个 GitHub 提交消息限制正则表达式时，使用 `creditlint github ruleset-pattern`。

可表示为单个规则集正则表达式：

- 具有精确尾注键的禁止尾注规则
- 精确字符串、非锚定正则表达式或 `Any` 的尾注值匹配器
- 表达为单个锚定行正则表达式的自由格式标记规则
- 允许的溯源键不与禁止尾注键重叠的策略

不可表示为单个规则集正则表达式：

- Git 作者或提交者身份规则
- 需要在同一尾注键上禁止/允许优先级的策略
- 依赖正则表达式匹配尾注字段名称的禁止规则
- 需要规范化或多个正则表达式遍的策略逻辑

当导出不安全时，命令会失败关闭。对提交元数据使用 CI 范围检查，对确切的最终合并消息使用受控的合并机器人验证步骤：

```sh
creditlint check --message-file final-merge-message.txt
```

## npm 包装器

npm 包是可选的。它适用于已通过 npm、pnpm 或 npx 安装开发工具的团队。

普通 npm 用户不需要 Rust 或 Cargo。`creditlint` npm 包委托给平台特定可选包（如 `creditlint-darwin-arm64` 或 `creditlint-linux-x64`）的原生二进制文件。

在 npm 包发布后安装：

```sh
pnpm add -D creditlint
pnpm exec creditlint --help
```

解析顺序：

1. `CREDITLINT_BIN`
2. 已安装的平台包二进制文件
3. 包本地 `packages/creditlint/native/`
4. 仓库本地 Cargo 输出 `target/release/` 和 `target/debug/`

JavaScript 包装器不重新实现策略逻辑或 Git 元数据收集。

## 隐私

`creditlint` 是本地优先的。

默认行为：

- 读取消息文本、Git 元数据和 `.creditlint.yml`
- 不上传提交消息或拉取请求文本
- 不需要托管账户或后台服务
- 策略评估期间不使用网络访问

任何未来的托管集成应作为单独的可选行为记录。

## 威胁模型

MVP 设计用于捕获：

- 附加类似署名标记的工具
- 意外粘贴 AI/工具贡献标记的贡献者
- 绕过本地钩子的云代理和 CI 路径
- 最终受保护分支消息与检查提交不同的平台合并路径

当前范围外：

- Unicode 同形字欺骗
- 故意拆分或混淆的标记
- 管理员绕过仓库规则
- 在强制工作流之外直接写入受保护分支

## 开发

使用 Cargo 进行 Rust 实现工作。仅对 OpenSpec 命令和可选的 npm 包装器工作区使用 pnpm。

常用命令：

```sh
just check
just fmt
just lint
just test
just test-npm
just openspec-validate
just ci
```

本地工具：

- `rust-toolchain.toml` 中的稳定 Rust
- `just` 用于项目配方
- `cargo-nextest` 作为首选测试运行器
- `cross` 用于发布打包任务
- pnpm 用于 OpenSpec 和 npm 包装器验证

OpenSpec 命令：

```sh
pnpm dlx @fission-ai/openspec list
pnpm dlx @fission-ai/openspec validate --all
pnpm dlx @fission-ai/openspec status --change bootstrap-creditlint-mvp --json
pnpm dlx @fission-ai/openspec status --change add-npm-wrapper-package --json
```

在代码或面向用户的行为更改之前，实现工作应遵循活动的 OpenSpec 变更任务。

## 发布和发布

发布准备涵盖：

- crates.io 上的 Rust CLI
- GitHub Actions 工作流产物用于手动发布运行
- GitHub Releases 用于标记的原生二进制文件和 `SHA256SUMS`
- 可选的 npm 平台包加上主 npm 包装器

有用的维护者命令：

```sh
just release-build
just cross-build x86_64-unknown-linux-gnu
just cross-build x86_64-pc-windows-msvc
just npm-trust-bootstrap-dry-run
just npm-publish-local-dry-run
just npm-publish-dry-run
```

在匹配的平台包具有暂存的原生二进制文件之前，不要将主 npm 包装器作为普通用户面向发布发布。

对于 crates.io 发布，GitHub 发布工作流期望 `CARGO_REGISTRY_TOKEN` 配置为仓库密钥。

## 版本控制

项目打算在首次公开发布后遵循 SemVer。

在该发布之前：

- 将进行中的工作保留在 `CHANGELOG.md` 的 `Unreleased` 下
- 在同一发布变更中提升包版本并添加变更日志标题
- 将 CLI 标志、配置模式、退出代码和 JSON 输出视为版本化的用户契约

## 许可证

MIT

## Star 趋势

[![Star History Chart](https://api.star-history.com/svg?repos=Drswith/creditlint&type=Date)](https://www.star-history.com/#Drswith/creditlint&Date)
