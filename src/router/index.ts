import { createRouter, createWebHistory } from "vue-router";

// 引入路由组件
import Home from "../components/pages/Home.vue";
import Observing from "../components/pages/Observing.vue";
import Updates from "../components/pages/Updates.vue";
import AppPage from "../components/pages/AppPage.vue";
import Category from "../components/pages/Category.vue";

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
            path: "/category/:categoryName",
            component: Category
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