import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'Nexis',
  description: 'Enterprise-grade Real-time Collaboration Platform — 100K+ concurrent connections, cloud-native, AI-powered',
  lang: 'en-US',
  
  ignoreDeadLinks: true,

  head: [
    // Favicon and theme
    ['link', { rel: 'icon', type: 'image/svg+xml', href: '/images/logo.svg' }],
    ['meta', { name: 'theme-color', content: '#6366f1' }],

    // SEO meta tags
    ['meta', { name: 'keywords', content: 'Nexis, real-time collaboration, WebSocket, Rust, cloud-native, AI, enterprise messaging' }],
    ['meta', { name: 'author', content: 'G-Brothers Group' }],
    ['meta', { name: 'description', content: 'Nexis is an enterprise-grade real-time collaboration platform built with Rust and Tokio. Supports 100K+ concurrent WebSocket connections with AI-powered summaries, plugin system, and multi-tenancy.' }],

    // Open Graph
    ['meta', { property: 'og:type', content: 'website' }],
    ['meta', { property: 'og:title', content: 'Nexis - Enterprise Real-time Collaboration Platform' }],
    ['meta', { property: 'og:description', content: '100K+ concurrent connections · AI-powered summaries · Plugin system · Multi-tenancy' }],
    ['meta', { property: 'og:image', content: 'https://gbrothersgroup.github.io/Nexis/images/og-image.png' }],
    ['meta', { property: 'og:url', content: 'https://gbrothersgroup.github.io/Nexis/' }],
    ['meta', { property: 'og:site_name', content: 'Nexis' }],

    // Twitter Card
    ['meta', { name: 'twitter:card', content: 'summary_large_image' }],
    ['meta', { name: 'twitter:title', content: 'Nexis - Enterprise Real-time Collaboration Platform' }],
    ['meta', { name: 'twitter:description', content: '100K+ concurrent connections · AI-powered summaries · Plugin system · Multi-tenancy' }],
    ['meta', { name: 'twitter:image', content: 'https://gbrothersgroup.github.io/Nexis/images/og-image.png' }],

    // Canonical URL
    ['link', { rel: 'canonical', href: 'https://gbrothersgroup.github.io/Nexis/' }],

    // Google Analytics (replace G-XXXXXXXXXX with your actual ID)
    ['script', { async: '', src: 'https://www.googletagmanager.com/gtag/js?id=G-XXXXXXXXXX' }],
    ['script', {}, `window.dataLayer = window.dataLayer || [];
function gtag(){dataLayer.push(arguments);}
gtag('js', new Date());
gtag('config', 'G-XXXXXXXXXX');`],
  ],
  
  themeConfig: {
    logo: '/images/logo.svg',
    siteTitle: 'Nexis',
    
    icon: {
      type: 'svg'
    },
    
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Quick Start', link: '/en/getting-started/quick-start' },
      { text: 'Docs', link: '/en/architecture/' },
      {
        text: 'Resources',
        items: [
          { text: 'Roadmap', link: '/en/roadmap' },
          { text: 'Contributing', link: '/en/development/contributing' },
          { text: 'Security', link: '/en/security/audit-report' },
          { text: 'Performance', link: '/en/performance/benchmark-report' },
        ]
      }
    ],
    
    sidebar: {
      '/en/': [
        {
          text: '📚 Introduction',
          items: [
            { text: 'Overview', link: '/en/' }
          ]
        },
        {
          text: '🚀 Getting Started',
          collapsed: false,
          items: [
            { text: 'Quick Start (5 min)', link: '/en/getting-started/quick-start' },
            { text: 'Development Guide', link: '/en/getting-started/development-guide' }
          ]
        },
        {
          text: '🏗️ Architecture',
          collapsed: false,
          items: [
            { text: 'Overview', link: '/en/architecture/' },
            { text: 'Data Flow', link: '/en/architecture/data-flow' },
            { text: 'Components', link: '/en/architecture/components' }
          ]
        },
        {
          text: '📡 API Reference',
          collapsed: true,
          items: [
            { text: 'Metrics API', link: '/en/api/metrics' },
            { text: 'Versioning', link: '/en/api/versioning' }
          ]
        },
        {
          text: '🔍 Observability',
          collapsed: true,
          items: [
            { text: 'Tracing', link: '/en/observability/tracing' }
          ]
        },
        {
          text: '⚡ Performance',
          collapsed: true,
          items: [
            { text: 'Benchmark Report', link: '/en/performance/benchmark-report' }
          ]
        },
        {
          text: '🛡️ Security',
          collapsed: true,
          items: [
            { text: 'Audit Report', link: '/en/security/audit-report' }
          ]
        },
        {
          text: '🤝 Community',
          collapsed: true,
          items: [
            { text: 'Contributing Guide', link: '/en/development/contributing' },
            { text: 'Roadmap', link: '/en/roadmap' }
          ]
        }
      ]
    },
    
    socialLinks: [
      { icon: 'github', link: 'https://github.com/gbrothersgroup/Nexis' },
      { icon: 'discord', link: 'https://discord.gg/clawd' }
    ],
    
    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright © 2026-present G-Brothers Group'
    },
    
    search: {
      provider: 'local',
      options: {
        translations: {
          button: {
            buttonText: 'Search Documentation',
            buttonAriaLabel: 'Search'
          }
        }
      }
    },
    
    editLink: {
      pattern: 'https://github.com/gbrothersgroup/Nexis/edit/main/docs/:path',
      text: 'Edit this page on GitHub'
    },
    
    outline: {
      level: [2, 3],
      label: 'On this page'
    },
    
    lastUpdated: {
      text: 'Last updated',
      formatOptions: {
        dateStyle: 'medium',
        timeStyle: 'short'
      }
    },
    
    docFooter: {
      prev: 'Previous Page',
      next: 'Next Page'
    }
  }
})
