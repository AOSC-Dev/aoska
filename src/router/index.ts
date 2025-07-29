import { createRouter, createWebHistory } from "vue-router";

// 引入路由组件
import Home from "../components/pages/HomePage.vue";
import ViewAllPage from "../components/pages/ViewAllPage.vue";
import UpdatePage from "../components/pages/UpdatePage.vue";
import AppPage from "../components/pages/AppPage.vue";
import CategoryPage from "../components/pages/CategoryPage.vue";

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
      component: CategoryPage
    },
    {
      path: "/view-all",
      component: ViewAllPage
    },
    {
      path: "/updates",
      component: UpdatePage
    },
    {
      path: "/app/:pkgName",
      component: AppPage
    }
  ]
});

export default router;
