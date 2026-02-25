# Phase 4 P2 Mobile (T12-T14) Design

Date: 2026-02-25

## Scope
- T12: Mobile app initialization and base architecture
- T13: Core mobile features (rooms, message stream, search)
- T14: Release readiness (EAS build, icon, basic tests)

## Architecture
- Runtime: Expo + React Native + TypeScript
- Navigation: React Navigation (Native Stack + Bottom Tabs)
- State: Zustand stores by feature (`auth`, `rooms`, `messages`, `search`)
- Networking: Axios HTTP client with interceptor-based token injection and refresh lock
- Auth persistence: `expo-secure-store` via a token storage service
- Biometric prep: `expo-local-authentication` capability + authenticate helper

## Reuse Strategy From Web
- Keep Web API client behavior parity:
  - Same endpoint shape (`/rooms`, `/messages`, `/search`, `/auth/*`)
  - Same refresh lock semantics (single in-flight refresh)
  - Same 401 recoverable handling (`code=token_expired`)
- Keep API modules organized as Web (`shared/api/httpClient`, `shared/api/endpoints/*`)

## Feature Plan
- Rooms screen: list rooms, loading/error states, tap to open room messages
- Messages screen: load room message stream and send message
- Search screen: query + result list

## Release Readiness
- EAS config (`eas.json`) for dev preview + production profile
- App icon in `assets/icon.png`
- Basic tests for token storage and auth store behavior

## Error Handling
- Uniform store-level error fields for API failures
- Refresh failure logs out and clears secure token state

## Testing
- Unit tests (Vitest):
  - `tokenStorage`: set/get/clear
  - `authStore`: login/logout persistence hooks
- Type check gate: `npm run typecheck`
