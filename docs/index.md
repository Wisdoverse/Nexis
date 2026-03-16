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
      link: /en/getting-started/development-guide
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
      src: /images/icon-observability.svg
    title: 全链路可观测
    details: <strong>Prometheus</strong> 指标监控<br/>Grafana 可视化面板<br/>OpenTelemetry 分布式追踪
    link: /en/observability/tracing
    linkText: 可观测性指南
  - icon:
      src: /images/icon-cloud.svg
    title: 云原生就绪
    details: <strong>12-Factor App</strong> 完全合规<br/>Kubernetes Helm Charts<br/>HPA 自动扩缩容 (3-100 副本)
    link: /en/operations/deployment
    linkText: 部署指南
  - icon:
      src: /images/icon-ai.svg
    title: AI 智能增强
    details: 上下文智能摘要<br/>AI 消息路由<br/>多模型提供商支持
    link: /en/architecture/components
    linkText: 了解更多
  - icon:
      src: /images/icon-platform.svg
    title: 跨平台支持
    details: <strong>Web</strong> (React + Vite)<br/><strong>Mobile</strong> (React Native)<br/><strong>Desktop</strong> (Tauri) · CLI
    link: /en/getting-started/development-guide
    linkText: 开发指南
---

<script setup>
import { onMounted } from 'vue'
import { useRouter } from 'vitepress'

onMounted(() => {
  // Add animation classes
  const hero = document.querySelector('.VPHero')
  if (hero) {
    hero.classList.add('animate-fade-in')
  }
})
</script>

<template>
  <div class="home-content">
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
      <a href="/en/getting-started/development-guide" class="cta-button primary">
        🚀 开始使用
      </a>
      <a href="https://github.com/gbrothersgroup/Nexis" class="cta-button secondary">
        ⭐ GitHub
      </a>
    </div>
  </div>
</template>

<style>
/* ===== 自定义首页样式 ===== */

/* 动画 */
@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translateY(30px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.7;
  }
}

@keyframes gradient {
  0% {
    background-position: 0% 50%;
  }
  50% {
    background-position: 100% 50%;
  }
  100% {
    background-position: 0% 50%;
  }
}

/* Hero 区域增强 */
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

/* 统计数据网格 */
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
  top: 0;
  left: 0;
  right: 0;
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

/* 快速开始区域 */
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

/* 徽章区域 */
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

.badge-icon {
  font-size: 1.5rem;
}

.badge-text {
  font-weight: 600;
  color: var(--vp-c-text-1);
}

.badge-status {
  font-size: 0.85rem;
  color: var(--vp-c-brand-1);
  font-weight: 700;
  background: var(--vp-c-brand-soft);
  padding: 0.25rem 0.75rem;
  border-radius: 9999px;
}

/* 架构预览 */
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

/* CTA 按钮 */
.cta-section {
  margin: 5rem auto;
  max-width: 600px;
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

/* Feature 卡片增强 */
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

.VPFeature .title {
  font-size: 1.3rem !important;
  font-weight: 700 !important;
}

.VPFeature .details {
  color: var(--vp-c-text-2);
  line-height: 1.7;
}

/* 响应式 */
@media (max-width: 768px) {
  .VPHero .name {
    font-size: 3rem !important;
  }
  
  .VPHero .text {
    font-size: 1.5rem !important;
  }
  
  .VPHero .tagline {
    font-size: 1rem !important;
  }
  
  .stat-number {
    font-size: 2rem;
  }
  
  .quick-start-section h2,
  .badges-section h3,
  .architecture-preview h3 {
    font-size: 1.8rem;
  }
}

/* 暗黑模式优化 */
.dark {
  .stat-card::before {
    background: linear-gradient(90deg, #818cf8, #c084fc);
  }
  
  .VPFeature:hover {
    box-shadow: 0 20px 60px rgba(129, 140, 248, 0.2) !important;
  }
}

/* 平滑滚动 */
html {
  scroll-behavior: smooth;
}

/* 全局字体优化 */
body {
  font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

/* 代码字体 */
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
    setTimeout(() => {
      button.textContent = '复制';
    }, 2000);
  });
}
</script>
