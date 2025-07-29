<template>
  <!-- 导航区 -->
  <NavBar></NavBar>
  <!-- 展示区 -->
  <RouterView></RouterView>
</template>

<script setup lang='ts'>
import { RouterView } from 'vue-router';
import NavBar from './components/share/NavBar.vue';
</script>

<style>
* {
  margin: 0;
  padding: 0;
}

</style>