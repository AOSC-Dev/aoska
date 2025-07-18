<template>
  <div class="header" :style="{ backgroundImage: `url(${appBannerImg}), linear-gradient(to right, #e7e7e7, #f3f3f3)` }">
    <div class="app-banner" >
      <img :src="appIcon" class="app-img" alt="App Icon">
      <div class="app-intro">
        <div class="name">{{ appName }}</div>
        <div class="intro">{{ appIntro }}</div>
      </div>
    </div>
    <Button class="install">
      安装
    </Button>
  </div>
</template>

<script setup lang="ts">
import Button from '../share/Button.vue';

const props = defineProps({
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
  height: calc( 192px - 20px * 2 );
  border-radius: 5px;
  margin: 65px 20px 10px 20px;
  padding: 20px;
  background-size: cover;
  background-position: center;
}

.header .app-img {
  width: 192px;
  height: 192px;
  margin: 20px;
}

.app-banner {
  width:100%;
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

.install {
  width: 120px;
  height: 36px;
  font-size: 24px;
  line-height: 36px;
  background-color: #ceffd6;
}

</style>