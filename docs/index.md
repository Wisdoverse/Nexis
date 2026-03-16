---
layout: home

hero:
  name: "Nexis"
  text: "Real-time Collaboration Platform"
  tagline: Enterprise-grade • Cloud-native • 100K+ concurrent connections
  image:
    src: /images/hero-diagram.svg
    alt: Nexis Architecture
  actions:
    - theme: brand
      text: 🚀 Get Started
      link: /en/getting-started/development-guide
    - theme: alt
      text: 📖 Documentation
      link: /en/architecture/
    - theme: alt
      text: ⭐ GitHub
      link: https://github.com/gbrothersgroup/Nexis

features:
  - icon: 🚀
    title: Ultra High Performance
    details: Sharded connection pool supporting <strong>100K+ concurrent WebSocket connections</strong> with O(1) operations. Built on Tokio async runtime.
  - icon: 🔐
    title: Enterprise Security
    details: JWT authentication, rate limiting, OWASP Top 10 compliant. Comprehensive audit trail with <strong>0 vulnerabilities</strong>.
  - icon: 📊
    title: Full Observability
    details: <strong>Prometheus metrics</strong>, Grafana dashboards, OpenTelemetry distributed tracing. Real-time monitoring out of the box.
  - icon: ☁️
    title: Cloud Native
    details: <strong>12-Factor App compliant</strong>, Kubernetes Helm charts with HPA auto-scaling (3-100 replicas). Production-ready deployment.
  - icon: 🤖
    title: AI-Powered Context
    details: Intelligent context summarization, AI-powered message routing. Multi-provider abstraction (OpenAI, Anthropic, local LLMs).
  - icon: 🌐
    title: Multi-Platform
    details: Web (React + Vite), Mobile (React Native + Expo), Desktop (Tauri), CLI. <strong>One codebase, all platforms</strong>.
---

<style>
:root {
  --vp-home-hero-name-color: transparent;
  --vp-home-hero-name-background: linear-gradient(120deg, #6366f1 30%, #a855f7 70%);
}

.VPHome .VPHero .image-bg {
  transition: all 0.5s ease;
}

.VPHome .features {
  padding: 2rem 0;
}

.VPFeature {
  transition: all 0.3s ease;
}

.VPFeature:hover {
  transform: translateY(-4px);
  box-shadow: 0 12px 40px rgba(0, 0, 0, 0.08);
}
</style>

<div style="text-align: center; margin: 3rem 0;">
  <h2>⚡ Quick Start</h2>
  
```bash
# Clone the repository
git clone https://github.com/gbrothersgroup/Nexis.git
cd Nexis

# Run with Docker
docker-compose up -d

# Or build from source
cargo build --release
```

</div>

<div style="text-align: center; margin: 3rem 0; padding: 2rem; background: var(--vp-c-bg-soft); border-radius: 12px;">
  <h3>📊 Project Stats</h3>
  
| Metric | Status |
|--------|--------|
| Test Coverage | 70%+ ✅ |
| Security Issues | 0 ✅ |
| Dependencies | All up-to-date ✅ |
| CI/CD | Full automation ✅ |
| Documentation | Complete ✅ |
  
</div>

<div style="margin: 3rem 0;">
  <h3>🏆 Enterprise Ready</h3>
  
  ✅ **12-Factor App**: 100% compliant  
  ✅ **CNCF Cloud-Native**: Production-grade  
  ✅ **OWASP Security**: Top 10 verified  
  ✅ **SOC 2**: Audit ready  
  ✅ **GDPR**: Compliant  
  
</div>
