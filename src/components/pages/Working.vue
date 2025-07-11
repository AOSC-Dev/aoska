<template>
  <div class="working">
    <AppBanner
      v-for="(app, index) in appList"
      :key="index"
      :name="app.name"
      :intro="app.intro"
      :version="app.version"
      :size="app.size"
      @click="showDetail"
    ></AppBanner>
  </div>
</template>

<script setup lang='ts'>
import { ref, onBeforeMount } from 'vue';
import { invoke } from '@tauri-apps/api/core';

import AppBanner from '../share/AppBanner.vue';
import router from '../../router';

// 定义AppInfo类型
interface AppInfo {
  name: string,
  intro: string,
  version: string,
  size: string
}

// 获取应用列表
// const appList = [
//   {
//     name: "WPS 办公套件",
//     intro: "这是应用程序 WPS，这是应用程序 WPS 办公套件",
//     version: "123.4.5",
//     size: "233 Mib"
//   },
//   {
//     name: "WPS 办公套件",
//     intro: "这是应用程序 WPS，这是应用程序 WPS 办公套件",
//     version: "123.4.5",
//     size: "233 Mib"
//   }
// ]

const appList = ref<AppInfo[]>([])
const fetchAppList = async (category: string) => {
  try {
    const result = await invoke<AppInfo[]>('fetch_by_category', { 
      category: category 
    });
    appList.value = result;
  } catch (error) {
    console.error('Error fetching list:', error);
  }
}

// 跳转到应用详情
const showDetail = () => {
  router.push("/app")
}

// 组件挂载时自动执行
const category = "working"
onBeforeMount(() => {
  fetchAppList(category)
})
</script>

<style scoped>
.working {
  margin: 60px 20px 20px 20px;
}
</style>