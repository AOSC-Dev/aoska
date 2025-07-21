import { createRouter, createWebHistory } from "vue-router";

// 引入路由组件
import Home from "../components/pages/Home.vue";
import Working from "../components/pages/Working.vue";
import Games from "../components/pages/Games.vue";
import Video from "../components/pages/Video.vue";
import Creating from "../components/pages/Creating.vue";
import Observing from "../components/pages/Observing.vue";
import Updates from "../components/pages/Updates.vue";
import AppPage from "../components/pages/AppPage.vue";

// 创建路由器
const router = createRouter({
    history: createWebHistory(),
    routes: [
        {
            // 重新定向
            path: "/",
            redirect: "/home"
        },
        {
            path: "/home",
            component: Home
        },
        {
            path: "/working",
            component: Working
        },
        {
            path: "/games",
            component: Games
        },
        {
            path: "/video",
            component: Video
        },
        {
            path: "/creating",
            component: Creating
        },
        {
            path: "/observing",
            component: Observing
        },
        {
            path: "/updates",
            component: Updates
        },
        {
            path: "/app/:pkgName",
            component: AppPage
        }
    ]
})

export default router