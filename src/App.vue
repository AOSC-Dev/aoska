<template>
  <!-- 导航区 -->
  <div class="navigator">
    <RouterLink to="/home">{{ $t("app.home") }}</RouterLink>
    <RouterLink to="/category/working">{{ $t("app.working") }}</RouterLink>
    <RouterLink to="/category/games">{{ $t("app.games") }}</RouterLink>
    <RouterLink to="/category/video">{{ $t("app.video") }}</RouterLink>
    <RouterLink to="/category/creating">{{ $t("app.creating") }}</RouterLink>
    <RouterLink to="/observing">{{ $t("app.observing") }}</RouterLink>
    <RouterLink to="/updates">{{ $t("app.updates") }}</RouterLink>
  </div>
  <!-- 展示区 -->
  <RouterView></RouterView>
</template>

<script setup lang='ts'>
import { RouterLink, RouterView } from 'vue-router';
</script>

<style>
* {
  margin: 0;
  padding: 0;
}

.navigator {
  display: flex;
  width: 100%;
  height: 40px;
  background-color: rgb(0, 53, 5);
  position: fixed;
  top: 0;
  left: 0;
  z-index: 9999;
}

.navigator a {
  display: inline-block;
  text-decoration: none;
  color: white;
  font-size: 18px;
  line-height: 30px;
  margin: 0 5px;
  padding: 0 8px;
  border-top: 5px solid transparent;
}

.navigator a:hover {
  border-bottom: 5px solid rgb(29, 148, 178);
}
</style>