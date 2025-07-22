<template>
  <div class="working">
    <div class="ranking">
      <span>{{ $t("pages.ranking") }}</span>
      <span>{{ $t("pages.name") }}</span>
      <span>{{ $t("pages.download") }}</span>
      <span>{{ $t("pages.update") }}</span>
    </div>
    <AppBanner
      v-for="(app, index) in categoryIndex?.packages"
      :key="index"
      :name="app.name"
      :intro="app.intro"
      :icon="resolveAssetPath(app.icon, app.name)"
      @click="showDetail(app.name)"
    ></AppBanner>
  </div>
</template>

<script setup lang='ts'>
import { ref, onMounted } from 'vue';

import AppBanner from '../share/AppBanner.vue';
import router from '../../router';
import { onBeforeRouteUpdate, useRoute } from 'vue-router';
import { CategoryIndex } from '../../types/home';
import { fetchByCategory } from '../../utils/wrapper';
import { resolveAssetPath } from '../../utils/url';

const route = useRoute();
const categoryIndex = ref<CategoryIndex | null>(null);

const fetchCategory = async (categoryName: string) => {
  categoryIndex.value = await fetchByCategory(categoryName as string);
}

onMounted(async () => {
  await fetchCategory(route.params.categoryName as string);
});

onBeforeRouteUpdate(async (to, _, next) => {
  await fetchCategory(to.params.categoryName as string);
  next();
});

// 定义AppInfo类型
interface AppInfo {
  name: string,
  intro: string,
  version: string,
  size: string
}

// 获取应用列表
const appList = [
  {
    name: "WPS 办公套件",
    intro: "这是应用程序 WPS，这是应用程序 WPS 办公套件",
    version: "123.4.5",
    size: "233 Mib"
  },
  {
    name: "WPS 办公套件",
    intro: "这是应用程序 WPS，这是应用程序 WPS 办公套件",
    version: "123.4.5",
    size: "233 Mib"
  }
]

// const appList = ref<AppInfo[]>([])
// const fetchAppList = async (category: string) => {
//   try {
//     const result = await invoke<AppInfo[]>('fetch_by_category', { 
//       category: category 
//     });
//     appList.value = result;
//   } catch (error) {
//     console.error('Error fetching list:', error);
//   }
// }

// 跳转到应用详情
const showDetail = (name: string) => {
  router.push(`/app/${name}`);
}

// 组件挂载时自动执行
// const category = "working"
// onBeforeMount(() => {
//   fetchAppList(category)
// })
</script>

<style scoped>
.working {
  margin: 60px 20px 20px 20px;
}

.ranking {
  height: 40px;
  font-size: 18px;
  line-height: 40px;
  background-color: rgb(206, 233, 255);
  border-radius: 5px 5px 0 0;
}

.ranking span {
  margin-left: 10px;
}
</style>