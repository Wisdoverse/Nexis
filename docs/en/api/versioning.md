# API Versioning Strategy

## Current Version
- **v1** - Current stable API

## Versioning Scheme

### URL Path (Primary)
```
/v1/rooms
/v2/rooms  (future)
```

### Header (Secondary)
```
X-API-Version: 1
```

## Deprecation Policy

| Phase | Timeline | Action |
|-------|----------|--------|
| Announcement | Day 0 | Mark as deprecated in docs |
| Warning Header | Day 30 | Add `Deprecated: true` header |
| Sunset Notice | Day 90 | Add `Sunset: <date>` header |
| Removal | Day 180 | Remove endpoint |

## Breaking vs Non-Breaking Changes

### Breaking Changes (Require Version Bump)
- Removing endpoints
- Changing request/response structure
- Changing authentication mechanism
- Changing error format
- Renaming fields

### Non-Breaking Changes (No Version Bump)
- Adding optional fields
- Adding new endpoints
- Adding new response fields
- Changing error messages
- Performance improvements

## Migration Guide

### Upgrading from v1 to v2
TBD - Document when v2 is released

## Best Practices

1. **Always check response headers** for deprecation notices
2. **Subscribe to API changelog** for updates
3. **Test against canary environment** before production
4. **Use versioned SDKs** when available

## Changelog
- 2026-03-15: Initial versioning policy documented
