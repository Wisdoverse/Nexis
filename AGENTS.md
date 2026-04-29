# Repository Guidelines

## Project Structure & Module Organization

Wisdoverse Nexus is a mixed Rust and TypeScript monorepo. Rust workspace crates live under `crates/`, with shared protocol, gateway, AI, plugin, billing, and related modules declared in `Cargo.toml`. Integration, security, stress, and end-to-end Rust tests live under `tests/`. Frontend clients are in `apps/web/` and `apps/mobile/`. SDKs are split between `sdk/typescript/` and `sdk/python/`. Documentation and public site sources are in `docs/`; deployment assets are in `deploy/`, `docker/`, `k8s/`, and `ops/`.

## Build, Test, and Development Commands

Install JavaScript dependencies from the repository root:

```bash
pnpm install --frozen-lockfile --ignore-scripts
```

Use Node.js 24.x with pnpm `>=10.30.0`. Use `cargo check --workspace` for Rust compile checks, `cargo test --workspace` for Rust tests, and `cargo clippy --workspace --all-targets -- -D warnings` for strict linting. Run `cargo fmt --all -- --check` before submitting Rust changes. Build the web app with `pnpm --filter @wisdoverse/nexus-web build`, the TypeScript SDK with `pnpm --filter @wisdoverse/nexus-sdk build`, and docs with `pnpm --dir docs docs:build`. Mobile checks use `pnpm --filter @wisdoverse/nexus-mobile typecheck`, `pnpm --filter @wisdoverse/nexus-mobile test`, and `pnpm --filter @wisdoverse/nexus-mobile exec expo install --check`.

## Coding Style & Naming Conventions

Rust uses edition 2021, four-space indentation, Unix newlines, 100-column formatting, and `unsafe_code = "forbid"`. Keep crate and module names in kebab/snake style consistent with existing `nexis-*` crates. TypeScript uses strict project `tsconfig` files; prefer PascalCase React components, camelCase functions and variables, and colocated CSS modules where already used.

## Testing Guidelines

Place Rust integration tests in `tests/` or crate-local test modules when behavior is crate-specific. Name tests by behavior, for example `rejects_invalid_tenant_token`. Mobile unit tests use Vitest and live in `__tests__/` directories. Web end-to-end tests are under `apps/web/e2e/tests/`.

## Commit & Pull Request Guidelines

The repository uses Conventional Commits in history, for example `docs: redesign GitHub Pages documentation site`. Prefer scoped forms such as `feat(gateway): add tenant-aware room lookup` or `fix(mobile): align Expo dependencies`. Pull requests should explain what changed, link issues, include verification commands and results, and add screenshots or logs for UI, CLI, or deployment behavior changes. Update `CHANGELOG.md` under `Unreleased` for user-visible changes.

## Security & Configuration Tips

Do not commit secrets. Start from `.env.example` or `deploy/.env.example`, and run pre-commit hooks when available for `gitleaks`, `detect-secrets`, formatting, and Clippy checks. Follow `SECURITY.md` for vulnerability reports.
