import { createApp } from 'vue'
import { createPinia } from 'pinia'
import router from './router'
import App from './App'
import 'virtual:uno.css'
import '@unocss/reset/tailwind.css'

const app = createApp(App)

app.use(createPinia())
app.use(router)

app.mount('#app')
