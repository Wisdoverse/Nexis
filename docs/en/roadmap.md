# Product Roadmap

Nexis is actively developed. Here's our public roadmap — priorities may shift based on community feedback.

---

## v0.1 — Core Platform ✅

> **Status:** Shipped

**Real-time collaboration foundation**

- ✅ WebSocket gateway with 100K+ concurrent connections
- ✅ JWT authentication & role-based access control
- ✅ Channel-based messaging (public, private, DM)
- ✅ Rate limiting & abuse protection
- ✅ Prometheus metrics & Grafana dashboards
- ✅ Docker & Kubernetes deployment
- ✅ OWASP Top 10 security compliance
- ✅ 270+ test cases, zero known vulnerabilities

---

## v0.2 — AI Enhancement 🔨

> **Status:** In Development · Q2 2026

**Intelligent collaboration features**

- 🔨 Context-aware message summaries (channel & thread level)
- 🔨 Sentiment analysis & toxicity detection
- 🔨 Smart notification routing (reduce noise, surface what matters)
- 🔨 AI-powered search with semantic understanding
- 🔨 Auto-transcription for voice messages
- 🔨 Multi-model provider support (OpenAI, Anthropic, local LLMs)

::: details Technical Approach
AI features are implemented as a pluggable module (`nexis-ai` crate). All AI processing happens server-side with configurable providers. Users can opt in/out per workspace.
:::

---

## v0.3 — Enterprise Features 📋

> **Status:** Planned · Q3 2026

**Enterprise-ready platform**

- 📋 SSO integration (SAML 2.0, OpenID Connect)
- 📋 SCIM user provisioning (Azure AD, Okta)
- 📋 Audit logging & compliance reporting
- 📋 Data retention policies & eDiscovery export
- 📋 IP allowlisting & network policies
- 📋 Custom branding & white-labeling
- 📋 SLA guarantees & premium support tiers

---

## v1.0 — General Availability 🎯

> **Status:** Planned · Q4 2026

**Production-ready at scale**

- 🎯 Formal stability guarantees (API versioning, deprecation policy)
- 🎯 Horizontal scaling playbook (10M+ connections)
- 🎯 Multi-region deployment support
- 🎯 Plugin marketplace (official & community plugins)
- 🎯 Managed cloud offering
- 🎯 Full SDK coverage (TypeScript, Python, Go, Java)
- 🎯 Comprehensive operator documentation & runbooks

---

## Contribute to the Roadmap

Have a feature request? We'd love to hear from you:

- **[GitHub Discussions](https://github.com/gbrothersgroup/Nexis/discussions)** — feature proposals & voting
- **[GitHub Issues](https://github.com/gbrothersgroup/Nexis/issues)** — bug reports & detailed requests
- **[Discord](https://discord.gg/clawd)** — real-time community chat

---

*Last updated: March 2026 · Roadmap is indicative and subject to change.*
