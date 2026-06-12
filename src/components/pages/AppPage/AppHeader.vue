<template>
  <div class="header" :style="{ backgroundImage: `linear-gradient(90deg, rgba(32,35,38,0.98), rgba(32,35,38,0.72), rgba(32,35,38,0.3)), url(${appBannerImg})` }">
    <div class="app-banner" >
      <img :src="appIcon" class="app-img" alt="App Icon">
      <div class="app-intro">
        <div class="name">{{ appName }}</div>
        <div class="intro">{{ appIntro }}</div>
      </div>
    </div>
    <CustomButton
      size="medium"
      color="green"
      width="129"
      height="37"
    >
      安装
    </CustomButton>
  </div>
</template>

<script setup lang="ts">
import CustomButton from '../../share/CustomButton.vue';

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
  overflow: hidden;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-card);
  margin: 62px 20px 10px;
  background-size: cover;
  background-position: center;
}

.header .app-img {
  width: 128px;
  height: 128px;
  margin: 40px;
  filter: drop-shadow(5px 5px 10px rgba(0, 0, 0, 0.5));
}

.app-banner {
  height: 100%;
  display: flex;
  align-items: center;
}

.name {
  color: var(--color-text);
  font-size: 40px;
  text-shadow: 0 2px 8px rgba(0, 0, 0, 0.8);
}

.intro {
  color: var(--color-text-muted);
  font-size: 18px;
}
</style>
