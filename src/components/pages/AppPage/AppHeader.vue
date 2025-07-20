<template>
  <div class="header" :style="{ backgroundImage: `url(${appBannerImg}), linear-gradient(to right, #e7e7e7, #f3f3f3)` }">
    <div class="app-banner" >
      <img :src="appIcon" class="app-img" alt="App Icon">
      <div class="app-intro">
        <div class="name">{{ appName }}</div>
        <div class="intro">{{ appIntro }}</div>
      </div>
    </div>
    <Button
      size="medium"
      color="green"
      width="129"
      height="37"
    >
      安装
    </Button>
  </div>
</template>

<script setup lang="ts">
import Button from '../../share/Button.vue';

defineProps({
  appIcon: {
    type: String,
  },
  appName: {
    type: String,
  },
  appIntro: {
    type: String,
  },
  appBannerImg: {
    type: String,
  }
});
</script>

<style lang="css" scoped>
.header {
  display: flex;
  height: 192px;
  border-radius: 5px;
  margin: 60px 20px 10px 20px;
  background-size: cover;
  background-position: center;
}

.header .app-img {
  width: 192px;
  height: 192px;
  margin: 20px;
}

.app-banner {
  height: 100%;
  display: flex;
  align-items: center;
}

.name {
  font-size: 40px;
}

.intro {
  font-size: 18px;
}
</style>