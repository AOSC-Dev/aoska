<template>
  <div class="up-content">
    <div class="update-center">
      <div class="title">{{ $t("updates.title") }}</div>
      <div class="intro">{{ $t("updates.intro") }}</div>
      <div class="status">
        <span class="number">
          <template v-if="loadingSystem">
            <span class="skeleton skeleton-number"></span>
          </template>
          <template v-else>
            {{ updateSystem }}
          </template>
        </span>
        {{ $t("updates.status1") }}
        <span class="number">
          <template v-if="loadingApp">
            <span class="skeleton skeleton-number"></span>
          </template>
          <template v-else>
            {{ updateApp }}
          </template>
        </span>
        {{ $t("updates.status2") }}
      </div>
    </div>
    <div class="download"></div>
  </div>
  <UpdateItem
    v-for="(tumUpdate, index) in tumUpdates"
    :key="index"
    :tum-update="tumUpdate"
    :update-detail="updateDetail"
  ></UpdateItem>
</template>

<script setup lang='ts'>
import { onMounted, ref } from 'vue';
import { fetchTumUpdate, fetchUpdateCount, fetchUpdateDetail } from '../../utils/wrapper';
import { OmaOperation, TumUpdateInfo } from '../../types/oma';
import UpdateItem from './Updates/UpdateItem.vue';

onMounted(async () => {
  const updateCount = await fetchUpdateCount();
  updateSystem.value = updateCount;
  tumUpdates.value = await fetchTumUpdate();
  updateDetail.value = await fetchUpdateDetail();
  loadingSystem.value = false;
  loadingApp.value = false;
});

// 系统升级与应用升级数
const updateSystem = ref(0);
const updateApp = ref(0);
const tumUpdates = ref<TumUpdateInfo[] | null>(null);
const updateDetail = ref<OmaOperation | null>(null);
const loadingSystem = ref(true);
const loadingApp = ref(true);
</script>

<style scoped>
.up-content {
  display: flex;
  margin-top: 40px;
}

.update-center {
  flex-grow: 1;
  height: 200px;
  margin: 20px;
  border-radius: 5px;
  background-color: rgb(231, 231, 231);
}

.title {
  margin-top: 30px;
  margin-left: 50px;
  font-size: 36px;
  font-weight: 700;
}

.intro {
  margin-left: 50px;
  font-size: 20px;
}

.status {
  margin-top: 5px;
  margin-left: 50px;
  font-size: 20px;
}

.number {
  font-size: 30px;
  font-weight: 500;
  display: inline-block;
}

.download {
  width: 300px;
  height: 200px;
  margin: 20px 20px 20px 0;
  border-radius: 5px;
  background-color: rgb(206, 233, 255);
}

.skeleton {
  display: inline-block;
  background: linear-gradient(90deg, #ececec 25%, #f3f3f3 37%, #ececec 63%);
  background-size: 400% 100%;
  animation: skeleton-loading 1.2s ease-in-out infinite;
  border-radius: 4px;
}

.skeleton-number {
  width: 40px;
  height: 34px;
  vertical-align: middle;
}

@keyframes skeleton-loading {
  0% {
    background-position: 100% 0;
  }
  100% {
    background-position: 0 0;
  }
}

</style>
