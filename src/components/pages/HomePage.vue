<template>
  <!-- 整体布局容器 -->
  <div class="home-layout">
    <!-- 上部组件 -->
    <div class="up-content">
      <WelcomeCard class="welcome"></WelcomeCard>
      <UpdateCard
        class="update"
        :is-loading="loading"
        :error="updateError"
        @action="router.push('/updates')"
        :update="update"
        :update-security="updateSecurity"
        :security-classification-available="securityClassificationAvailable"
      ></UpdateCard>
    </div>

    <!-- 下部组件 -->
    <div class="down-content">
      <div class="app-content">
        <div class="app-title">{{ $t("home.appTitle") }}</div>
        <p v-if="catalogError" role="alert">{{ catalogError }}</p>
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
import { fetchRecommend } from '../../utils/wrapper';
import { RecommendIndex } from '../../types/home';
import { resolveAssetPath } from '../../utils/url';
import { getPackageManager } from '../../utils/pmClient';

// 总升级与安全升级数
const loading = ref(true);
const update = ref(0);
const updateSecurity = ref(0);
const securityClassificationAvailable = ref(true);

const updateError = ref('');
const catalogError = ref('');
onBeforeMount(() => {
  void fetchRecommend().then(value => { recommendList.value = value; }).catch(cause => { catalogError.value = String(cause); });
  void getPackageManager().updateSummary().then(summary => {
    securityClassificationAvailable.value = summary.securityClassificationAvailable;
    updateSecurity.value = summary.security ?? 0;
    update.value = Math.max(0, summary.total - updateSecurity.value);
  }).catch(cause => { updateError.value = String(cause); }).finally(() => { loading.value = false; });
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
  grid-template-columns: minmax(0, 2fr) minmax(320px, 1fr);
  gap: 20px;
  padding: 62px 20px 20px;
  color: var(--color-text);
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
  color: var(--color-text);
  font-size: 28px;
  margin-bottom: 10px;
  font-weight: 500;
}

.apps {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 15px;
}

.tip-content {
  min-height: 300px;
  margin-bottom: 20px;
  flex-shrink: 0;
}

.tip-title {
  color: var(--color-text);
  font-size: 28px;
  margin-bottom: 10px;
  font-weight: 500;
}

.tips {
  display: flex;
  flex-direction: column;
  gap: 12px;
}
</style>
