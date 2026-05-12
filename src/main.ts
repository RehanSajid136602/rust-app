import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { createRouter, createWebHistory } from 'vue-router'
import App from './App.vue'
import './styles/main.css'

// Import pages
import Dashboard from './pages/Dashboard.vue'
import Products from './pages/Products.vue'
import Clients from './pages/Clients.vue'
import Invoices from './pages/Invoices.vue'
import Quotations from './pages/Quotations.vue'
import Settings from './pages/Settings.vue'

const routes = [
  { path: '/', name: 'Dashboard', component: Dashboard },
  { path: '/products', name: 'Products', component: Products },
  { path: '/clients', name: 'Clients', component: Clients },
  { path: '/invoices', name: 'Invoices', component: Invoices },
  { path: '/quotations', name: 'Quotations', component: Quotations },
  { path: '/settings', name: 'Settings', component: Settings },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

const pinia = createPinia()
const app = createApp(App)

app.use(pinia)
app.use(router)
app.mount('#app')
