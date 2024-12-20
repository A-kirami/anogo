import { createNotivue } from 'notivue'
import { createPinia } from 'pinia'
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'
import { createApp } from 'vue'

import App from './App.vue'

import '@unocss/reset/tailwind.css'
import 'virtual:uno.css'

import '~/styles/main.css'

const app = createApp(App)

const pinia = createPinia()
pinia.use(piniaPluginPersistedstate)
app.use(pinia)

const notivue = createNotivue({
  limit: 3,
  notifications: {
    global: {
      duration: 3000,
    },
  },
})
app.use(notivue)

app.mount('#app')
