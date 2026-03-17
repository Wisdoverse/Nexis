---
layout: home

hero:
  name: Nexis
  text: Real-time Collaboration
  tagline: 企业级实时协作平台 · 支持 10 万+ 并发连接 · 云原生架构
  image:
    src: /images/hero-illustration.svg
    alt: Nexis Architecture Diagram
  actions:
    - theme: brand
      text: 🚀 快速开始
      link: /en/getting-started/quick-start
    - theme: alt
      text: 📖 查看文档
      link: /en/architecture/
    - theme: alt
      text: 💻 GitHub
      link: https://github.com/gbrothersgroup/Nexis

features:
  - icon: 
      src: /images/icon-rocket.svg
    title: 极致性能
    details: <strong>10 万+</strong>并发 WebSocket 连接<br/>O(1) 时间复杂度的连接操作<br/>基于 Tokio 异步运行时
    link: /en/performance/benchmark-report
    linkText: 查看性能报告
  - icon:
      src: /images/icon-security.svg
    title: 企业级安全
    details: JWT 身份认证 · 速率限制<br/><strong>OWASP Top 10</strong> 合规<br/>零漏洞依赖
    link: /en/security/audit-report
    linkText: 安全审计报告
  - icon:
      src: /images/icon-ai.svg
    title: AI 智能增强
    details: 上下文智能摘要 · 情绪分析<br/>AI 消息路由<br/>多模型提供商支持
    link: /en/roadmap
    linkText: 路线图
  - icon:
      src: /images/icon-platform.svg
    title: 插件系统
    details: <strong>Wasm</strong> 沙箱隔离<br/>丰富的生命周期钩子<br/>插件市场 (即将推出)
    link: /en/development/contributing
    linkText: 贡献指南
  - icon:
      src: /images/icon-cloud.svg
    title: 云原生 + 多租户
    details: <strong>12-Factor App</strong> 完全合规<br/>Kubernetes Helm Charts<br/>租户隔离 & 数据分区
    link: /en/operations/deployment
    linkText: 部署指南
  - icon:
      src: /images/icon-observability.svg
    title: 全链路可观测
    details: <strong>Prometheus</strong> 指标监控<br/>Grafana 可视化面板<br/>OpenTelemetry 分布式追踪
    link: /en/observability/tracing
    linkText: 可观测性指南
---

<script setup>
import { onMounted } from 'vue'
import { useRouter } from 'vitepress'

onMounted(() => {
  const hero = document.querySelector('.VPHero')
  if (hero) {
    hero.classList.add('animate-fade-in')
  }
})
</script>

<template>
  <div class="home-content">

    <!-- Trusted By -->
    <div class="trusted-by">
      <p class="trusted-label">Trusted by teams at</p>
      <div class="logo-grid">
        <div class="logo-placeholder">🏢 TechCorp</div>
        <div class="logo-placeholder">🏦 FinGroup</div>
        <div class="logo-placeholder">🏥 HealthTech</div>
        <div class="logo-placeholder">🚀 StartupX</div>
        <div class="logo-placeholder">🎓 EduPlatform</div>
      </div>
    </div>

    <div class="stats-grid">
      <div class="stat-card">
        <div class="stat-number">100K+</div>
        <div class="stat-label">并发连接</div>
      </div>
      <div class="stat-card">
        <div class="stat-number">270+</div>
        <div class="stat-label">测试用例</div>
      </div>
      <div class="stat-card">
        <div class="stat-number">0</div>
        <div class="stat-label">安全漏洞</div>
      </div>
      <div class="stat-card">
        <div class="stat-number">100%</div>
        <div class="stat-label">云原生合规</div>
      </div>
    </div>

    <div class="quick-start-section">
      <h2>⚡ 快速开始</h2>
      <div class="code-group">
        <div class="code-block">
          <div class="code-header">
            <span class="code-title">克隆项目</span>
            <button class="copy-btn" onclick="copyCode(this)">复制</button>
          </div>
          <pre><code class="language-bash">git clone https://github.com/gbrothersgroup/Nexis.git
cd Nexis</code></pre>
        </div>
        
        <div class="code-block">
          <div class="code-header">
            <span class="code-title">使用 Docker 运行</span>
            <button class="copy-btn" onclick="copyCode(this)">复制</button>
          </div>
          <pre><code class="language-bash">docker-compose up -d
# 访问 http://localhost:8080</code></pre>
        </div>
        
        <div class="code-block">
          <div class="code-header">
            <span class="code-title">从源码构建</span>
            <button class="copy-btn" onclick="copyCode(this)">复制</button>
          </div>
          <pre><code class="language-bash">cargo build --release
cargo run -p nexis-gateway</code></pre>
        </div>
      </div>
    </div>

    <!-- Comparison Table -->
    <div class="comparison-section">
      <h2>⚖️ Why Nexis?</h2>
      <div class="comparison-table-wrapper">
        <table class="comparison-table">
          <thead>
            <tr>
              <th>Feature</th>
              <th class="highlight-col">Nexis</th>
              <th>Slack</th>
              <th>Discord</th>
              <th>Mattermost</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td>Max Concurrent Connections</td>
              <td class="highlight-col"><strong>100K+</strong></td>
              <td>~10K</td>
              <td>~25K</td>
              <td>~5K</td>
            </tr>
            <tr>
              <td>Self-Hosted</td>
              <td class="highlight-col">✅ Full</td>
              <td>❌ Cloud only</td>
              <td>⚠️ Limited</td>
              <td>✅ Full</td>
            </tr>
            <tr>
              <td>AI Summaries</td>
              <td class="highlight-col">✅ Built-in</td>
              <td>⚠️ Extra cost</td>
              <td>❌</td>
              <td>⚠️ Plugin</td>
            </tr>
            <tr>
              <td>Plugin System (Wasm)</td>
              <td class="highlight-col">✅ Sandboxed</td>
              <td>⚠️ Node.js</td>
              <td>⚠️ Bot API</td>
              <td>⚠️ Go/React</td>
            </tr>
            <tr>
              <td>Multi-Tenancy</td>
              <td class="highlight-col">✅ Native</td>
              <td>Enterprise</td>
              <td>❌</td>
              <td>⚠️ Workspaces</td>
            </tr>
            <tr>
              <td>Open Source</td>
              <td class="highlight-col">✅ MIT</td>
              <td>❌ Proprietary</td>
              <td>❌ Proprietary</td>
              <td>✅ MIT/Proprietary</td>
            </tr>
            <tr>
              <td>Language</td>
              <td class="highlight-col">Rust</td>
              <td>Scala/C++</td>
              <td>Elixir/JS</td>
              <td>Go/React</td>
            </tr>
            <tr>
              <td>SSO / SAML</td>
              <td class="highlight-col">✅ v0.3</td>
              <td>✅ Enterprise</td>
              <td>⚠️ Limited</td>
              <td>✅ Enterprise</td>
            </tr>
            <tr>
              <td>Sub-ms Message Latency</td>
              <td class="highlight-col">✅ <1ms P99</td>
              <td>~100ms</td>
              <td>~50ms</td>
              <td>~200ms</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <div class="badges-section">
      <h3>🏆 企业级认证</h3>
      <div class="badge-grid">
        <div class="badge-item">
          <span class="badge-icon">✅</span>
          <span class="badge-text">12-Factor App</span>
          <span class="badge-status">100%</span>
        </div>
        <div class="badge-item">
          <span class="badge-icon">✅</span>
          <span class="badge-text">CNCF 云原生</span>
          <span class="badge-status">生产就绪</span>
        </div>
        <div class="badge-item">
          <span class="badge-icon">✅</span>
          <span class="badge-text">OWASP 安全</span>
          <span class="badge-status">Top 10</span>
        </div>
        <div class="badge-item">
          <span class="badge-icon">✅</span>
          <span class="badge-text">SOC 2</span>
          <span class="badge-status">审计就绪</span>
        </div>
        <div class="badge-item">
          <span class="badge-icon">✅</span>
          <span class="badge-text">GDPR</span>
          <span class="badge-status">合规</span>
        </div>
        <div class="badge-item">
          <span class="badge-icon">✅</span>
          <span class="badge-text">CI/CD</span>
          <span class="badge-status">全自动化</span>
        </div>
      </div>
    </div>

    <div class="architecture-preview">
      <h3>🏗️ 架构预览</h3>
      <img src="/images/architecture-diagram.svg" alt="Nexis Architecture" class="arch-img" />
    </div>

    <div class="cta-section">
      <a href="/en/getting-started/quick-start" class="cta-button primary">
        🚀 开始使用
      </a>
      <a href="https://github.com/gbrothersgroup/Nexis" class="cta-button secondary">
        ⭐ GitHub
      </a>
      <a href="/en/roadmap" class="cta-button secondary">
        🗺️ 路线图
      </a>
    </div>
  </div>
</template>

<style>
/* ===== 自定义首页样式 ===== */

@keyframes fadeInUp {
  from { opacity: 0; transform: translateY(30px); }
  to { opacity: 1; transform: translateY(0); }
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.7; }
}

@keyframes gradient {
  0% { background-position: 0% 50%; }
  50% { background-position: 100% 50%; }
  100% { background-position: 0% 50%; }
}

.VPHero {
  animation: fadeInUp 0.8s ease-out;
}

.VPHero .name {
  font-size: 5rem !important;
  font-weight: 900 !important;
  background: linear-gradient(135deg, #6366f1 0%, #a855f7 50%, #ec4899 100%);
  background-size: 200% 200%;
  animation: gradient 5s ease infinite;
  -webkit-background-clip: text;
  background-clip: text;
  color: transparent;
  text-shadow: 0 0 80px rgba(99, 102, 241, 0.3);
}

.VPHero .text {
  font-size: 2.5rem !important;
  font-weight: 700;
  margin-top: 0.5rem;
  color: var(--vp-c-text-1);
}

.VPHero .tagline {
  font-size: 1.3rem !important;
  color: var(--vp-c-text-2);
  margin-top: 1rem;
  font-weight: 400;
}

/* Trusted By */
.trusted-by {
  margin: 3rem auto;
  max-width: 900px;
  text-align: center;
}

.trusted-label {
  font-size: 0.9rem;
  text-transform: uppercase;
  letter-spacing: 0.1em;
  color: var(--vp-c-text-3);
  margin-bottom: 1.5rem;
}

.logo-grid {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 2rem;
  flex-wrap: wrap;
}

.logo-placeholder {
  font-size: 1.1rem;
  color: var(--vp-c-text-3);
  padding: 0.75rem 1.5rem;
  border: 1px solid var(--vp-c-divider);
  border-radius: 8px;
  background: var(--vp-c-bg-soft);
  opacity: 0.7;
  transition: opacity 0.3s ease;
}

.logo-placeholder:hover {
  opacity: 1;
}

/* Stats */
.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 2rem;
  margin: 4rem auto;
  max-width: 1000px;
  padding: 0 2rem;
}

.stat-card {
  background: linear-gradient(135deg, var(--vp-c-bg-soft) 0%, var(--vp-c-bg-alt) 100%);
  border: 1px solid var(--vp-c-divider);
  border-radius: 16px;
  padding: 2rem;
  text-align: center;
  transition: all 0.3s ease;
  position: relative;
  overflow: hidden;
}

.stat-card::before {
  content: '';
  position: absolute;
  top: 0; left: 0; right: 0;
  height: 4px;
  background: linear-gradient(90deg, #6366f1, #a855f7);
}

.stat-card:hover {
  transform: translateY(-8px);
  box-shadow: 0 20px 60px rgba(99, 102, 241, 0.15);
  border-color: var(--vp-c-brand-1);
}

.stat-number {
  font-size: 3rem;
  font-weight: 900;
  background: linear-gradient(135deg, #6366f1, #a855f7);
  -webkit-background-clip: text;
  background-clip: text;
  color: transparent;
  margin-bottom: 0.5rem;
}

.stat-label {
  font-size: 1rem;
  color: var(--vp-c-text-2);
  font-weight: 500;
}

/* Quick Start */
.quick-start-section {
  margin: 5rem auto;
  max-width: 1200px;
  padding: 0 2rem;
}

.quick-start-section h2 {
  font-size: 2.5rem;
  font-weight: 800;
  margin-bottom: 2rem;
  text-align: center;
  background: linear-gradient(135deg, #6366f1, #a855f7);
  -webkit-background-clip: text;
  background-clip: text;
  color: transparent;
}

.code-group {
  display: grid;
  gap: 1.5rem;
}

.code-block {
  background: var(--vp-code-block-bg);
  border-radius: 12px;
  overflow: hidden;
  border: 1px solid var(--vp-c-divider);
  transition: all 0.3s ease;
}

.code-block:hover {
  box-shadow: 0 8px 30px rgba(99, 102, 241, 0.1);
}

.code-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 1.5rem;
  background: var(--vp-c-bg-soft);
  border-bottom: 1px solid var(--vp-c-divider);
}

.code-title {
  font-weight: 600;
  color: var(--vp-c-text-1);
}

.copy-btn {
  background: linear-gradient(135deg, #6366f1, #a855f7);
  color: white;
  border: none;
  padding: 0.4rem 1rem;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.85rem;
  font-weight: 500;
  transition: all 0.2s ease;
}

.copy-btn:hover {
  transform: scale(1.05);
  box-shadow: 0 4px 12px rgba(99, 102, 241, 0.3);
}

.code-block pre {
  margin: 0;
  padding: 1.5rem;
  background: transparent;
}

/* Comparison Table */
.comparison-section {
  margin: 5rem auto;
  max-width: 1200px;
  padding: 0 2rem;
}

.comparison-section h2 {
  font-size: 2.5rem;
  font-weight: 800;
  margin-bottom: 2rem;
  text-align: center;
  background: linear-gradient(135deg, #6366f1, #a855f7);
  -webkit-background-clip: text;
  background-clip: text;
  color: transparent;
}

.comparison-table-wrapper {
  overflow-x: auto;
  border-radius: 12px;
  border: 1px solid var(--vp-c-divider);
}

.comparison-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.95rem;
}

.comparison-table th,
.comparison-table td {
  padding: 0.85rem 1.2rem;
  text-align: left;
  border-bottom: 1px solid var(--vp-c-divider);
}

.comparison-table thead th {
  background: var(--vp-c-bg-soft);
  font-weight: 700;
  color: var(--vp-c-text-1);
  font-size: 0.9rem;
  text-transform: uppercase;
  letter-spacing: 0.03em;
}

.comparison-table tbody tr:hover {
  background: var(--vp-c-bg-soft);
}

.comparison-table .highlight-col {
  background: rgba(99, 102, 241, 0.08);
  font-weight: 600;
}

.comparison-table thead .highlight-col {
  background: linear-gradient(135deg, rgba(99, 102, 241, 0.15), rgba(168, 85, 247, 0.15));
  color: var(--vp-c-brand-1);
}

/* Badges */
.badges-section {
  margin: 5rem auto;
  max-width: 1200px;
  padding: 0 2rem;
  text-align: center;
}

.badges-section h3 {
  font-size: 2rem;
  font-weight: 800;
  margin-bottom: 2rem;
  background: linear-gradient(135deg, #6366f1, #a855f7);
  -webkit-background-clip: text;
  background-clip: text;
  color: transparent;
}

.badge-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 1.5rem;
}

.badge-item {
  background: var(--vp-c-bg-soft);
  border: 1px solid var(--vp-c-divider);
  border-radius: 12px;
  padding: 1.5rem;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  transition: all 0.3s ease;
}

.badge-item:hover {
  transform: translateY(-4px);
  border-color: var(--vp-c-brand-1);
  box-shadow: 0 8px 30px rgba(99, 102, 241, 0.1);
}

.badge-icon { font-size: 1.5rem; }
.badge-text { font-weight: 600; color: var(--vp-c-text-1); }
.badge-status {
  font-size: 0.85rem;
  color: var(--vp-c-brand-1);
  font-weight: 700;
  background: var(--vp-c-brand-soft);
  padding: 0.25rem 0.75rem;
  border-radius: 9999px;
}

/* Architecture */
.architecture-preview {
  margin: 5rem auto;
  max-width: 1200px;
  padding: 0 2rem;
  text-align: center;
}

.architecture-preview h3 {
  font-size: 2rem;
  font-weight: 800;
  margin-bottom: 2rem;
  background: linear-gradient(135deg, #6366f1, #a855f7);
  -webkit-background-clip: text;
  background-clip: text;
  color: transparent;
}

.arch-img {
  width: 100%;
  max-width: 900px;
  border-radius: 16px;
  box-shadow: 0 20px 60px rgba(99, 102, 241, 0.15);
  border: 1px solid var(--vp-c-divider);
}

/* CTA */
.cta-section {
  margin: 5rem auto;
  max-width: 700px;
  padding: 0 2rem;
  display: flex;
  gap: 1.5rem;
  justify-content: center;
  flex-wrap: wrap;
}

.cta-button {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  padding: 1rem 2.5rem;
  border-radius: 12px;
  font-size: 1.1rem;
  font-weight: 700;
  text-decoration: none;
  transition: all 0.3s ease;
  border: 2px solid transparent;
}

.cta-button.primary {
  background: linear-gradient(135deg, #6366f1, #a855f7);
  color: white;
  box-shadow: 0 8px 30px rgba(99, 102, 241, 0.3);
}

.cta-button.primary:hover {
  transform: translateY(-4px);
  box-shadow: 0 12px 40px rgba(99, 102, 241, 0.4);
}

.cta-button.secondary {
  background: transparent;
  color: var(--vp-c-text-1);
  border-color: var(--vp-c-divider);
}

.cta-button.secondary:hover {
  border-color: var(--vp-c-brand-1);
  background: var(--vp-c-brand-soft);
}

/* Feature Cards */
.VPFeature {
  border-radius: 16px !important;
  border: 1px solid var(--vp-c-divider);
  transition: all 0.3s ease !important;
  background: var(--vp-c-bg-soft);
  padding: 2rem !important;
}

.VPFeature:hover {
  transform: translateY(-8px) !important;
  box-shadow: 0 20px 60px rgba(99, 102, 241, 0.15) !important;
  border-color: var(--vp-c-brand-1) !important;
}

.VPFeature .title { font-size: 1.3rem !important; font-weight: 700 !important; }
.VPFeature .details { color: var(--vp-c-text-2); line-height: 1.7; }

/* Responsive */
@media (max-width: 768px) {
  .VPHero .name { font-size: 3rem !important; }
  .VPHero .text { font-size: 1.5rem !important; }
  .VPHero .tagline { font-size: 1rem !important; }
  .stat-number { font-size: 2rem; }
  .quick-start-section h2,
  .badges-section h3,
  .architecture-preview h3,
  .comparison-section h2 { font-size: 1.8rem; }
  .comparison-table { font-size: 0.8rem; }
  .comparison-table th, .comparison-table td { padding: 0.6rem 0.8rem; }
  .logo-grid { gap: 1rem; }
}

.dark .stat-card::before {
  background: linear-gradient(90deg, #818cf8, #c084fc);
}
.dark .VPFeature:hover {
  box-shadow: 0 20px 60px rgba(129, 140, 248, 0.2) !important;
}

html { scroll-behavior: smooth; }
body {
  font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}
code, pre {
  font-family: 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
}
</style>

<script>
function copyCode(button) {
  const codeBlock = button.closest('.code-block');
  const code = codeBlock.querySelector('code').textContent;
  navigator.clipboard.writeText(code).then(() => {
    button.textContent = '已复制!';
    setTimeout(() => { button.textContent = '复制'; }, 2000);
  });
}
</script>
