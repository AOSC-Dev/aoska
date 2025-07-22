import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import i18n from "./locales";

import './assets/styles/variables.css'
import { createPinia } from "pinia";
import { useConfigStore } from "./stores/cofig";

async function bootstrap() {
    const pinia = createPinia()
    const app = createApp(App)

    app.use(pinia)
    app.use(router)
    app.use(i18n)

    const configStore = useConfigStore();
    await configStore.loadStaticEndpoint();

    app.mount("#app");
}

bootstrap();
