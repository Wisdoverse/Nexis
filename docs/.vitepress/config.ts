import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'Nexis',
  description: 'Enterprise-grade Real-time Collaboration Platform',
  lang: 'en-US',
  
  // Ignore dead links for now
  ignoreDeadLinks: true,
  
  head: [
    ['link', { rel: 'icon', type: 'image/svg+xml', href: '/images/logo.svg' }],
    ['meta', { name: 'theme-color', content: '#6366f1' }],
    ['meta', { name: 'og:type', content: 'website' }],
    ['meta', { name: 'og:title', content: 'Nexis | Real-time Collaboration Platform' }],
    ['meta', { name: 'og:description', content: 'Enterprise-grade, cloud-native, 100K+ concurrent connections' }],
  ],
  
  themeConfig: {
    logo: '/images/logo.svg',
    siteTitle: 'Nexis',
    
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Getting Started', link: '/en/getting-started/development-guide' },
      { text: 'Architecture', link: '/en/architecture/' },
      { text: 'API Reference', link: '/en/api/metrics' },
      {
        text: 'Resources',
        items: [
          { text: 'Security', link: '/en/security/audit-report' },
          { text: 'Performance', link: '/en/performance/benchmark-report' },
          { text: 'Operations', link: '/en/operations/' }
        ]
      }
    ],
    
    sidebar: {
      '/en/': [
        {
          text: '📚 Documentation',
          items: [
            { text: 'Introduction', link: '/en/' }
          ]
        },
        {
          text: '🚀 Getting Started',
          collapsed: false,
          items: [
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
          collapsed: false,
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
