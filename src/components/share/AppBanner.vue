<template>
  <div class="banner">
    <img src="../../assets/icons/aosc_logo.png" alt="安同社区的logo">
    <div>
      <div class="name">{{ name }}</div>
      <div class="info">{{ size }} | {{ version }}</div>
      <div class="intro">{{ intro }}</div>
      </div>
      <div class="button">{{ $t("appBanner.install") }}</div>
  </div>
</template>

<script setup lang='ts'>
// 接收参数
const props = defineProps<{
  name: string,
  intro: string,
  version: string,
  size: string,
}>()
</script>

<style scoped>
.banner {
  display: flex;
  width: 100%;
  height: 80px;
  background-color: white;
  border-bottom: 1px solid black;
}

.banner:hover {
  background-color: rgb(231, 231, 231);
}

img {
  width: 60px;
  height: 60px;
  margin: 10px;
}

.name {
  margin-top: 8px;
  margin-left: 5px;
  font-size: 24px;
  line-height: 24px;
}

.info {
  margin-top: 3px;
  margin-left: 5px;
  font-size: 16px;
  line-height: 16px;
  color: rgb(84, 84, 84);
}

.intro {
  margin-top: 4px;
  margin-top: 3px;
  margin-left: 5px;
  font-size: 16px;
  line-height: 16px;
}

.button {
  margin: auto 15px 15px auto;
  width: 120px;
  height: 35px;
  border: 1px solid black;
  border-radius: 5px;
  font-size: 20px;
  text-align: center;
  line-height: 35px;
}

.button:hover {
  background-color: rgb(206, 255, 214);
}
</style>