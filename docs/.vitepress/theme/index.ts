import { h } from 'vue'
import type { Theme } from 'vitepress'
import DefaultTheme from 'vitepress/theme'
import Breadcrumb from '../components/Breadcrumb.vue'
import './custom.css'

export default {
  extends: DefaultTheme,
  Layout: () => {
    return h(DefaultTheme.Layout, null, {
      'doc-before': () => h(Breadcrumb)
    })
  },
  enhanceApp({ app, router, siteData }) {
    // Register global components if needed
  }
} satisfies Theme
