<template>
  <div class="welcome">
    <div class="sub-title">{{ $t("welcome.sub-title") }}</div>
    <div class="title">{{ $t("welcome.title") }}</div>
  </div>
</template>

<script setup lang='ts'></script>

<style scoped>
.welcome {
  border-radius: 5px;
  background-color: rgb(0, 32, 74);
}

.sub-title {
  padding: 70px 0 0 30px;
  font-size: 30px;
  color: white;
  font-weight: 600;
  -webkit-text-stroke: 1px black;
}

.title {
  padding-left: 30px;
  font-size: 38px;
  color: white;
  font-weight: 600;
  -webkit-text-stroke: 1px black;
}
</style>
