# 测试指南

本文档描述 Nexis 平台的测试策略，涵盖单元测试、集成测试、端到端测试和覆盖率目标。

## 概述

Nexis 采用多层测试方法确保 Rust 后端、TypeScript SDK 和 React Web 客户端的可靠性。

```
┌─────────────────────────────────────────┐
│         E2E 测试 (Playwright)            │  ← 完整用户流程
├─────────────────────────────────────────┤
│            集成测试                       │  ← API + 数据库 + WebSocket
├─────────────────────────────────────────┤
│            单元测试                       │  ← 独立函数与模块
├─────────────────────────────────────────┤
│         属性测试 (Property-based)        │  ← 模糊/生成式测试
└─────────────────────────────────────────┘
```

## 单元测试

### 策略

单元测试在隔离环境中验证单个函数和模块。它们应该是**快速的**（每个 < 10ms）、**确定性的**，且**不依赖**外部服务。

### Rust 后端

单元测试使用 Rust 内置的 `#[cfg(test)]` 模块，与源码放在同一文件中：

```rust
// crates/nexis-gateway/src/crypto/tests.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let encryption = DataEncryption::from_env();
        let plaintext = b"sensitive message data";

        let encrypted = encryption.encrypt(plaintext).expect("encryption should succeed");
        let decrypted = encryption.decrypt(&encrypted).expect("decryption should succeed");

        assert_eq!(plaintext.to_vec(), decrypted);
    }

    #[test]
    fn test_decrypt_invalid_data_returns_error() {
        let encryption = DataEncryption::from_env();
        let result = encryption.decrypt(b"invalid encrypted data");

        assert!(result.is_err());
    }
}
```

### 测试组织结构

```
crates/nexis-gateway/
├── src/
│   ├── crypto/
│   │   ├── mod.rs
│   │   └── tests.rs          ← 加密模块单元测试
│   ├── router/
│   │   ├── mod.rs
│   │   └── tests.rs          ← 路由逻辑单元测试
│   └── auth/
│       ├── mod.rs
│       └── tests.rs          ← 认证逻辑单元测试
└── tests/
    ├── api_integration.rs    ← 集成测试（见下文）
    ├── load_test.rs          ← 负载/压力测试
    └── boundary_conditions.rs ← 边界条件测试
```

### 运行单元测试

```bash
# 运行 crate 中的所有单元测试
cargo test -p nexis-gateway

# 运行特定测试
cargo test -p nexis-gateway test_encrypt_decrypt_roundtrip

# 显示输出运行测试
cargo test -p nexis-gateway -- --nocapture

# 运行匹配模式的测试
cargo test -p nexis-gateway crypto -- --nocapture
```

### 属性测试

对于数据密集型逻辑，使用 `proptest` 进行生成式测试：

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_room_name_always_valid(name in "[a-zA-Z0-9_-]{1,100}") {
        let room = Room::new(&name, "test");
        assert!(!room.name.is_empty());
        assert!(room.name.len() <= 100);
    }

    #[test]
    fn test_message_id_unique_after_serialization(
        content in "[a-z]{1,500}",
        room_id in "[a-z0-9]{1,36}"
    ) {
        let msg = StoredMessage::new(&content, &room_id);
        let serialized = serde_json::to_string(&msg).unwrap();
        let deserialized: StoredMessage = serde_json::from_str(&serialized).unwrap();
        assert_eq!(msg.id, deserialized.id);
    }
}
```

## 集成测试

### 策略

集成测试验证多个组件协同工作是否正确。它们测试完整的 HTTP 处理栈，包括路由、中间件（认证）和状态管理。

### API 集成测试

集成测试位于 `tests/` 目录，测试完整的 HTTP 栈：

```rust
// crates/nexis-gateway/tests/api_integration.rs

use axum::body::Body;
use axum::http::{Request, StatusCode};
use nexis_gateway::build_routes;
use serde_json::Value;
use tower::ServiceExt;

fn auth_header() -> String {
    let now = chrono::Utc::now().timestamp() as usize;
    let claims = nexis_gateway::auth::Claims {
        sub: "integration-user".to_string(),
        exp: now + 3600,
        iat: now,
        iss: "nexis".to_string(),
        aud: "nexis".to_string(),
        member_type: "human".to_string(),
    };
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret("default_secret".as_bytes()),
    )
    .expect("encode test token");
    format!("Bearer {token}")
}

#[tokio::test]
async fn api_create_room_and_send_message_roundtrip() {
    let app = build_routes();
    let auth = auth_header();

    // 步骤 1：创建房间
    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/rooms")
                .header("content-type", "application/json")
                .header("authorization", auth.clone())
                .body(Body::from(
                    serde_json::json!({
                        "name": "integration-room",
                        "topic": "api",
                    })
                    .to_string(),
                ))
                .expect("create room request should build"),
        )
        .await
        .expect("create room response");

    assert_eq!(create_response.status(), StatusCode::CREATED);

    // 步骤 2：验证房间已创建
    let room_payload: Value = serde_json::from_slice(
        &axum::body::to_bytes(create_response.into_body(), usize::MAX)
            .await
            .expect("create room body"),
    )
    .expect("create room payload should parse");
    let room_id = room_payload["id"].as_str().expect("room id should exist");

    // 步骤 3：向房间发送消息
    let msg_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/messages")
                .header("content-type", "application/json")
                .header("authorization", auth.clone())
                .body(Body::from(
                    serde_json::json!({
                        "room_id": room_id,
                        "content": "Hello, integration test!",
                    })
                    .to_string(),
                ))
                .expect("send message request should build"),
        )
        .await
        .expect("send message response");

    assert_eq!(msg_response.status(), StatusCode::OK);
}
```

### 运行集成测试

```bash
# 运行所有集成测试
cargo test -p nexis-gateway --test api_integration

# 运行特定集成测试
cargo test -p nexis-gateway --test api_integration api_create_room_and_send_message_roundtrip

# 运行 tests/ 目录下所有测试
cargo test -p nexis-gateway --tests
```

### 边界条件测试

```rust
// crates/nexis-gateway/tests/boundary_conditions.rs

#[tokio::test]
async fn test_unauthenticated_request_returns_401() {
    let app = build_routes();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/v1/rooms")
                .body(Body::empty())
                .expect("request should build"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_create_room_with_empty_name_returns_400() {
    let app = build_routes();
    let auth = auth_header();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/rooms")
                .header("content-type", "application/json")
                .header("authorization", auth)
                .body(Body::from(
                    serde_json::json!({ "name": "", "topic": "" }).to_string(),
                ))
                .expect("request should build"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
```

## 端到端测试

### 策略

E2E 测试通过 Web 客户端验证完整的用户工作流，同时测试前端和后端。使用 Playwright 运行。

### 运行 E2E 测试

```bash
# 安装依赖
cd apps/web && npm ci

# 运行 E2E 测试
npx playwright test

# UI 模式运行
npx playwright test --ui

# 运行特定测试文件
npx playwright test rooms.spec.ts
```

### E2E 测试示例

```typescript
// apps/web/e2e/rooms.spec.ts
import { test, expect } from '@playwright/test';

test.describe('Rooms', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/login');
    await page.fill('[data-testid="email"]', 'test@nexis.ai');
    await page.fill('[data-testid="password"]', 'test-password');
    await page.click('[data-testid="login-btn"]');
    await expect(page).toHaveURL(/\/rooms/);
  });

  test('should create a new room', async ({ page }) => {
    await page.click('[data-testid="create-room-btn"]');
    await page.fill('[data-testid="room-name-input"]', 'E2E Test Room');
    await page.fill('[data-testid="room-topic-input"]', 'Testing');
    await page.click('[data-testid="save-room-btn"]');

    await expect(page.locator('text=E2E Test Room')).toBeVisible();
  });

  test('should send and display a message', async ({ page }) => {
    await page.click('text=general');
    await page.fill('[data-testid="message-input"]', 'Hello from E2E!');
    await page.press('[data-testid="message-input"]', 'Enter');

    await expect(page.locator('text=Hello from E2E!')).toBeVisible();
  });
});
```

## 负载与压力测试

性能测试确保系统能够处理预期负载：

```rust
// crates/nexis-gateway/tests/load_test.rs

#[tokio::test]
async fn test_concurrent_room_creation() {
    let app = build_routes();
    let auth = auth_header();

    let handles: Vec<_> = (0..100)
        .map(|i| {
            let app = app.clone();
            let auth = auth.clone();
            tokio::spawn(async move {
                let response = app
                    .oneshot(
                        Request::builder()
                            .method("POST")
                            .uri("/v1/rooms")
                            .header("content-type", "application/json")
                            .header("authorization", auth)
                            .body(Body::from(
                                serde_json::json!({
                                    "name": format!("load-test-room-{}", i),
                                    "topic": "load",
                                })
                                .to_string(),
                            ))
                            .expect("request"),
                    )
                    .await
                    .expect("response");
                response.status()
            })
        })
        .collect();

    let statuses: Vec<_> = futures::future::join_all(handles)
        .await
        .into_iter()
        .map(|r| r.unwrap())
        .collect();

    // 所有请求都应该成功
    for status in &statuses {
        assert_eq!(*status, StatusCode::CREATED);
    }
}
```

基准测试使用 [Criterion.rs](https://github.com/brianhicks/criterion.rs)，通过 `benchmark.yml` 工作流自动运行：

```bash
# 本地运行基准测试
cargo bench --workspace

# 运行特定基准测试
cargo bench -p nexis-gateway -- "room_creation"
```

## 覆盖率

### 目标

| 指标 | 目标 | 配置位置 |
|------|------|----------|
| **项目覆盖率** | ≥ 70% | `codecov.yml` |
| **增量覆盖率** | ≥ 80% | `codecov.yml` |
| **覆盖率阈值** | 允许 ±5% 偏差 | `codecov.yml` |

### 配置

```yaml
# codecov.yml
coverage:
  status:
    project:
      default:
        target: 70%
        threshold: 5%
    patch:
      default:
        target: 80%
        threshold: 10%
```

### 测量覆盖率

```bash
# 安装覆盖率工具
cargo install cargo-tarpaulin

# 生成覆盖率报告
cargo tarpaulin --workspace --out Html --out Lcov

# 查看 HTML 报告
open tarpaulin-report.html
```

覆盖率报告在 CI 中自动生成并上传到 Codecov。

## 测试类别总结

| 类别 | 位置 | 运行时间 | 何时编写 |
|------|------|----------|----------|
| **单元测试** | `src/**/tests.rs` | ~1ms/测试 | 每个有非平凡逻辑的函数 |
| **属性测试** | `src/**/tests.rs` | ~10ms/测试 | 序列化、解析、校验逻辑 |
| **集成测试** | `tests/*.rs` | ~50ms/测试 | API 端点、跨模块流程 |
| **E2E 测试** | `apps/web/e2e/` | ~1-5s/测试 | 关键用户旅程 |
| **负载测试** | `tests/load_test.rs` | ~10s+/测试 | 性能敏感路径 |
| **基准测试** | `benches/` | ~30s+/测试 | 热路径、优化代码 |

## 最佳实践

1. **测试行为而非实现** — 测试应验证代码做什么，而非怎么做
2. **使用有意义的测试名称** — `test_decrypt_invalid_data_returns_error` > `test_decrypt_2`
3. **保持测试独立** — 测试间不共享可变状态；每个测试设置自己的数据
4. **模拟外部依赖** — 使用 `mockall` 模拟 trait；使用 `httpmock` 模拟外部 HTTP 服务
5. **测试错误路径** — 不仅测试成功路径；还要测试 400、401、404、429、500 场景
6. **异步测试使用 `#[tokio::test]`** — 测试中永远不要使用 `std::thread::sleep`；使用 `tokio::time::advance` 代替

## 另见

- [开发指南](/zh-CN/getting-started/development-guide) — 开发环境搭建
- [发布流程](/zh-CN/guides/release-process) — CI 流水线与质量门禁
- [性能基准](/zh-CN/performance/benchmark-report) — 基准测试结果
