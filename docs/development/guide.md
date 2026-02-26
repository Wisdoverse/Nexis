# Development Guide / 开发指南

## Table of Contents / 目录

- [Environment Setup / 环境搭建](#environment-setup--环境搭建)
- [Local Commands / 本地常用命令](#local-commands--本地常用命令)
- [Code Quality / 质量门禁](#code-quality--质量门禁)
- [Branch and PR / 分支与 PR](#branch-and-pr--分支与-pr)

## Environment Setup / 环境搭建

```bash
git clone https://github.com/schorsch888/Nexis.git
cd Nexis
cargo build --workspace
```

## Local Commands / 本地常用命令

```bash
# format
cargo fmt --all

# lint
cargo clippy --workspace --all-targets -- -D warnings

# test
cargo test --workspace

# run gateway
cargo run -p nexis-gateway

# run cli help
cargo run -p nexis-cli -- --help
```

## Code Quality / 质量门禁

- No new warnings in lint.
- Keep tests green for affected crates.
- Update docs when behavior changes.

- 不新增 lint 警告。
- 受影响 crate 的测试必须通过。
- 行为变化必须同步更新文档。

## Branch and PR / 分支与 PR

- Branch naming: `feat/*`, `fix/*`, `docs/*`
- Commit style: Conventional Commits
- PR requires clear scope and test evidence

- 分支命名：`feat/*`、`fix/*`、`docs/*`
- 提交信息：Conventional Commits
- PR 需包含改动范围与测试证据
