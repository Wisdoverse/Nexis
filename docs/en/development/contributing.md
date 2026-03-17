# Contributing to Nexis

Thank you for your interest in contributing! This guide covers everything you need to get started.

## Development Environment Setup

### Prerequisites

- **Rust** ≥ 1.75 (`rustup default stable`)
- **Node.js** ≥ 18 (for frontend)
- **Docker** ≥ 20.10 (for local services like Redis, PostgreSQL)
- **just** (task runner, optional)

### Setup

```bash
git clone https://github.com/gbrothersgroup/Nexis.git
cd Nexis

# Start infrastructure dependencies
docker-compose -f docker-compose.dev.yml up -d

# Build & run backend
cargo build
cargo test
cargo run -p nexis-gateway

# Build & run frontend (in a separate terminal)
cd apps/web
npm install
npm run dev
```

### Project Structure

```
Nexis/
├── crates/              # Rust backend
│   ├── nexis-gateway/   # WebSocket gateway (entry point)
│   ├── nexis-auth/      # Authentication & JWT
│   ├── nexis-api/       # REST API layer
│   ├── nexis-core/      # Core types & protocols
│   ├── nexis-ai/        # AI/LLM integration
│   └── nexis-plugins/   # Plugin runtime & SDK
├── apps/
│   ├── web/             # React + Vite frontend
│   ├── mobile/          # React Native app
│   └── desktop/         # Tauri desktop app
├── sdk/
│   ├── typescript/      # @nexis/sdk
│   └── python/          # nexis-sdk
├── docs/                # VitePress documentation
└── deploy/
    ├── docker/          # Dockerfiles
    └── k8s/             # Kubernetes manifests & Helm charts
```

## Code Standards

### Rust

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Run `cargo fmt` before every commit
- Run `cargo clippy -- -D warnings` — zero warnings allowed
- All public APIs must have doc comments (`///`)
- Write tests for new functionality (`cargo test`)

### TypeScript / Frontend

- ESLint + Prettier configured — run `npm run lint` and `npm run format`
- Use functional components with hooks (React)
- Prefer Zod for runtime type validation

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat(gateway): add rate limiting middleware
fix(auth): resolve JWT refresh token race condition
docs(quick-start): add Python SDK example
test(core): add connection pool stress tests
chore(deps): bump tokio to 1.40
```

## PR Process

### 1. Create a Branch

```bash
git checkout main
git pull
git checkout -b feat/your-feature-name
```

### 2. Make Changes

- Keep PRs small and focused (one concern per PR)
- Add tests for new behavior
- Update documentation if needed

### 3. Submit

```bash
git push origin feat/your-feature-name
# Open a PR on GitHub targeting `main`
```

### PR Checklist

- [ ] `cargo fmt` + `cargo clippy` pass
- [ ] `cargo test` passes (all 270+ tests green)
- [ ] New tests added for changed code
- [ ] Documentation updated (if applicable)
- [ ] Breaking changes noted in PR description (if any)

### Review Process

1. At least 1 approval required
2. CI must pass (lint, test, build, security scan)
3. Squash merge to `main` — maintainers will handle this

## Plugin Development

Nexis has a built-in plugin system for extending functionality. Plugins run in an isolated WebAssembly (Wasm) runtime for safety.

### Plugin Structure

```rust
use nexis_plugins::{Plugin, PluginContext, Message, PluginResult};

#[derive(Default)]
struct MyPlugin;

#[nexis_plugins::plugin(name = "my-plugin", version = "0.1.0")]
impl Plugin for MyPlugin {
    fn on_message(&self, msg: &Message, ctx: &mut PluginContext) -> PluginResult<()> {
        // Process incoming messages
        if msg.content.contains("/greet") {
            ctx.reply("Hello there! 👋");
        }
        Ok(())
    }

    fn on_connect(&self, user_id: &str, ctx: &mut PluginContext) -> PluginResult<()> {
        ctx.log(format!("User {} connected", user_id));
        Ok(())
    }
}
```

### Plugin Hooks

| Hook | Description |
|------|-------------|
| `on_message` | Intercept every message before delivery |
| `on_connect` | User connected to gateway |
| `on_disconnect` | User disconnected |
| `on_channel_join` | User joined a channel |
| `on_channel_leave` | User left a channel |
| `on_schedule` | Periodic tasks (cron-like) |

### Building a Plugin

```bash
cargo build --target wasm32-wasip1 --release -p my-plugin
# Output: target/wasm32-wasip1/release/my_plugin.wasm
```

### Loading a Plugin

```yaml
# nexis.toml
[plugins]
enabled = ["my-plugin"]
[plugins.sources.my-plugin]
path = "./plugins/my_plugin.wasm"
```

## Getting Help

- **GitHub Discussions** — [Ask questions here](https://github.com/gbrothersgroup/Nexis/discussions)
- **Discord** — [Join our community](https://discord.gg/clawd)
- **Issues** — [Bug reports & feature requests](https://github.com/gbrothersgroup/Nexis/issues)

Thank you for contributing to Nexis! 🎉
