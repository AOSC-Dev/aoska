<template>
  <!-- 导航区 -->
  <Navbar></Navbar>
  <!-- 展示区 -->
  <RouterView></RouterView>
</template>

<script setup lang='ts'>
import { RouterView } from 'vue-router';
import Navbar from './components/share/Navbar.vue';
</script>

<style>
* {
  margin: 0;
  padding: 0;
}

</style>