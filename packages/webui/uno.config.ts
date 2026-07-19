import { defineConfig, presetWind, presetIcons } from 'unocss'

export default defineConfig({
  presets: [presetWind(), presetIcons()],
  shortcuts: {
    'btn': 'px-4 py-2 rounded-lg cursor-pointer transition-colors',
    'btn-primary': 'btn bg-primary text-white hover:bg-primary/80',
    'card': 'bg-surface rounded-xl shadow-sm border border-border',
    'input': 'px-3 py-2 rounded-lg border border-border bg-input focus:outline-none focus:ring-2 focus:ring-primary/50',
  },
  theme: {
    colors: {
      primary: 'rgb(211, 55, 116)',
      surface: 'rgb(252, 252, 252)',
      border: 'rgb(250, 224, 232)',
      input: 'rgb(252, 252, 252)',
    },
  },
})
