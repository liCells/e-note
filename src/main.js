import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import store from './store'
import conf from './conf'

const app = createApp(App)

app.config.globalProperties.globalConfig = conf.config

app.use(store).use(router).mount('#app')
