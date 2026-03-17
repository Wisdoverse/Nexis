# Testing Guide

This document describes the testing strategy for the Nexis platform, covering unit tests, integration tests, end-to-end tests, and coverage targets.

## Overview

Nexis uses a multi-layer testing approach to ensure reliability across its Rust backend, TypeScript SDK, and React web client.

```
┌─────────────────────────────────────────┐
│            E2E Tests (Playwright)        │  ← Full user flows
├─────────────────────────────────────────┤
│         Integration Tests                │  ← API + DB + WebSocket
├─────────────────────────────────────────┤
│           Unit Tests                     │  ← Individual functions & modules
├─────────────────────────────────────────┤
│         Property-Based Tests             │  ← Fuzz / generative testing
└─────────────────────────────────────────┘
```

## Unit Tests

### Strategy

Unit tests verify individual functions and modules in isolation. They should be **fast** (< 10ms each), **deterministic**, and **independent** of external services.

### Rust Backend

Unit tests live alongside source code using Rust's built-in `#[cfg(test)]` module:

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

### Test Organization

```
crates/nexis-gateway/
├── src/
│   ├── crypto/
│   │   ├── mod.rs
│   │   └── tests.rs          ← Unit tests for crypto module
│   ├── router/
│   │   ├── mod.rs
│   │   └── tests.rs          ← Unit tests for routing logic
│   └── auth/
│       ├── mod.rs
│       └── tests.rs          ← Unit tests for auth logic
└── tests/
    ├── api_integration.rs    ← Integration tests (see below)
    ├── load_test.rs          ← Load/stress tests
    └── boundary_conditions.rs ← Edge case tests
```

### Running Unit Tests

```bash
# Run all unit tests in a crate
cargo test -p nexis-gateway

# Run a specific test
cargo test -p nexis-gateway test_encrypt_decrypt_roundtrip

# Run tests with output
cargo test -p nexis-gateway -- --nocapture

# Run tests matching a pattern
cargo test -p nexis-gateway crypto -- --nocapture
```

### Property-Based Testing

For data-intensive logic, use `proptest` for generative testing:

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

## Integration Tests

### Strategy

Integration tests verify that multiple components work together correctly. They test HTTP handlers with real routing, middleware (auth), and state.

### API Integration Tests

Integration tests live in the `tests/` directory and test the full HTTP stack:

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

    // Step 1: Create a room
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

    // Step 2: Verify room was created
    let room_payload: Value = serde_json::from_slice(
        &axum::body::to_bytes(create_response.into_body(), usize::MAX)
            .await
            .expect("create room body"),
    )
    .expect("create room payload should parse");
    let room_id = room_payload["id"].as_str().expect("room id should exist");

    // Step 3: Send a message to the room
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

### Running Integration Tests

```bash
# Run all integration tests
cargo test -p nexis-gateway --test api_integration

# Run specific integration test
cargo test -p nexis-gateway --test api_integration api_create_room_and_send_message_roundtrip

# Run all tests in the tests/ directory
cargo test -p nexis-gateway --tests
```

### Boundary Condition Tests

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

## End-to-End Tests

### Strategy

E2E tests verify complete user workflows through the web client, exercising both the frontend and backend together. These tests use Playwright.

### Running E2E Tests

```bash
# Install dependencies
cd apps/web && npm ci

# Run E2E tests
npx playwright test

# Run with UI mode
npx playwright test --ui

# Run specific test file
npx playwright test rooms.spec.ts
```

### E2E Test Example

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

## Load & Stress Tests

Performance tests ensure the system handles expected load:

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

    // All requests should succeed
    for status in &statuses {
        assert_eq!(*status, StatusCode::CREATED);
    }
}
```

Benchmarks use [Criterion.rs](https://github.com/brianhicks/criterion.rs) and run automatically via the `benchmark.yml` workflow:

```bash
# Run benchmarks locally
cargo bench --workspace

# Run specific benchmark
cargo bench -p nexis-gateway -- "room_creation"
```

## Coverage

### Targets

| Metric | Target | Configured In |
|--------|--------|---------------|
| **Project coverage** | ≥ 70% | `codecov.yml` |
| **Patch coverage** | ≥ 80% | `codecov.yml` |
| **Coverage threshold** | ±5% drift allowed | `codecov.yml` |

### Configuration

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

### Measuring Coverage

```bash
# Install coverage tool
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --workspace --out Html --out Lcov

# View HTML report
open tarpaulin-report.html
```

Coverage reports are generated in CI and uploaded to Codecov as artifacts.

## Test Categories Summary

| Category | Location | Runtime | When to Write |
|----------|----------|---------|---------------|
| **Unit** | `src/**/tests.rs` | ~1ms per test | Every function with non-trivial logic |
| **Property-based** | `src/**/tests.rs` | ~10ms per test | Serialization, parsing, validation |
| **Integration** | `tests/*.rs` | ~50ms per test | API endpoints, cross-module flows |
| **E2E** | `apps/web/e2e/` | ~1-5s per test | Critical user journeys |
| **Load** | `tests/load_test.rs` | ~10s+ per test | Performance-sensitive paths |
| **Benchmark** | `benches/` | ~30s+ per test | Hot paths, optimized code |

## Best Practices

1. **Test behavior, not implementation** — Tests should verify what the code does, not how it does it.
2. **Use meaningful test names** — `test_decrypt_invalid_data_returns_error` > `test_decrypt_2`.
3. **Keep tests independent** — No shared mutable state between tests; each test sets up its own data.
4. **Mock external dependencies** — Use `mockall` for trait mocks; `httpmock` for external HTTP services.
5. **Test error paths** — Not just happy paths; test 400, 401, 404, 429, 500 scenarios.
6. **Use `#[tokio::test]`** for async tests — Never use `std::thread::sleep` in tests; use `tokio::time::advance` instead.

## See Also

- [Development Guide](/en/getting-started/development-guide) — Setting up the dev environment
- [Release Process](/en/guides/release-process) — CI pipeline and quality gates
- [Performance Benchmarks](/en/performance/benchmark-report) — Benchmark results
