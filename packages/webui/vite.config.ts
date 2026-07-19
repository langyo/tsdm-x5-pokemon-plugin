import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueJsx from '@vitejs/plugin-vue-jsx'
import UnoCSS from 'unocss/vite'
import { resolve } from 'path'

export default defineConfig({
  plugins: [vue(), vueJsx(), UnoCSS()],
  resolve: {
    alias: {
      '@': resolve(__dirname, 'src'),
    },
  },
  base: '/source/plugin/pokemon/dist/',
  build: {
    outDir: '../dist',
    target: 'es2020',
    rollupOptions: {
      output: {
        manualChunks: {
          vue: ['vue', 'vue-router', 'pinia'],
        },
      },
    },
  },
  server: {
    proxy: {
      '/plugin.php': 'https://localhost:8443',
      '/source': 'https://localhost:8443',
      '/data': 'https://localhost:8443',
    },
  },
  test: {
    environment: 'happy-dom',
    include: ['src/**/*.test.ts'],
  },
})
