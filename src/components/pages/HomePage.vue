<template>
  <!-- 整体布局容器 -->
  <div class="home-layout">
    <!-- 上部组件 -->
    <div class="up-content">
      <WelcomeCard class="welcome"></WelcomeCard>
      <UpdateCard 
        class="update"
        :is-loading="loading"
        :update="update"
        :update-security="updateSecurity"
      ></UpdateCard>
    </div>

    <!-- 下部组件 -->
    <div class="down-content">
      <div class="app-content">
        <div class="app-title">{{ $t("home.appTitle") }}</div>
        <div class="apps">
          <AppCard
            v-for="(app, index) in recommendList?.packages"
            :key="index"
            :name="app.name"
            :intro="app.intro"
            :image="resolveAssetPath(app.icon, app.name)"
            @click="showDetail(app.name)"
          ></AppCard>
        </div>
      </div>
      <div class="tip-content">
        <div class="tip-title">{{ $t("home.tipTitle") }}</div>
        <div class="tips">
          <TipCard
            v-for="(tip, index) in tipList"
            :key="index"
            :month="tip.month"
            :day="tip.day"
            :title="tip.title"
            :intro="tip.intro"
          ></TipCard>
        </div>
      </div>
    </div>
  </div>
</template>


<script setup lang='ts'>
import { ref, onBeforeMount } from 'vue';

import WelcomeCard from '../share/WelcomeCard.vue';
import UpdateCard from '../share/UpdateCard.vue';
import AppCard from '../share/AppCard.vue';
import TipCard from '../share/TipCard.vue';
import router from '../../router';
import { fetchRecommend, fetchTumUpdate, fetchUpdateCount } from '../../utils/wrapper';
import { RecommendIndex } from '../../types/home';
import { resolveAssetPath } from '../../utils/url';

// 总升级与安全升级数
const loading = ref(true);
const update = ref(0);
const updateSecurity = ref(0);

onBeforeMount(async () => {
  const updateCount = await fetchUpdateCount();
  const tumUpdate = await fetchTumUpdate();
  const securityUpdateCount = tumUpdate.filter((v) => {return v.is_security;}).length;
  updateSecurity.value = securityUpdateCount;
  update.value = updateCount - securityUpdateCount;
  recommendList.value = await fetchRecommend();
  loading.value = false;
});


// 技巧指南列表
const tipList = [
  {
    month: "四月",
    day: 23,
    title: "龙架构如何运行 x86 程序？",
    intro: "LATX 安装及使用指南"
  },
  {
    month: "三月",
    day: 17,
    title: "开机时间为何漫长？",
    intro: "手把手教您分析和调整系统服务配置"
  },
  {
    month: "二月",
    day: 22,
    title: "Windows 时间错乱？",
    intro: "调整时间配置，轻松同步双系统配置"
  },
  {
    month: "二月",
    day: 7,
    title: "应用无法安装？",
    intro: "安同维护者来帮您！"
  },
];

// 获取推荐列表
const recommendList = ref<RecommendIndex | null>(null);

// const recommendList = ref<AppInfo[]>([])
// const fetchRecommendList = async () => {
//   try {
//     const result = await invoke<AppInfo[]>('fetch_recommend');
//     recommendList.value = result;
//   } catch (error) {
//     console.error('Error fetching list:', error);
//   }
// }

// 跳转到应用详情
const showDetail = (name: string) => {
  router.push(`/app/${name}`);
};

// 组件挂载时自动执行
// onBeforeMount(() => {
//   fetchRecommendList()
// })
</script>

<style scoped>
.home-layout {
  display: grid;
  grid-template-columns: 2fr 1fr;
  gap: 20px;
  margin: 60px 20px 0 20px;
}

/* 上部区域 */
.up-content {
  display: contents; /* 让子元素直接继承 grid 列 */
}

.welcome {
  height: 200px;
}

.update {
  height: 200px;
}

/* 下部区域 */
.down-content {
  display: contents; /* 同样继承 grid 列 */
}

.app-content {
  margin-bottom: 20px;
}

.app-title {
  font-size: 26px;
  margin-bottom: 5px;
  font-weight: 500;
}

.apps {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 16px;
}

.tip-content {
  height: 300px;
  margin-bottom: 20px;
  flex-shrink: 0;
}

.tip-title {
  font-size: 26px;
  margin-bottom: 5px;
  font-weight: 500;
}

.tips {
  display: flex;
  flex-direction: column;
  gap: 12px;
}
</style>
