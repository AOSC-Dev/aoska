<template>
  <div class="card">
    <img src="../../assets/icons/aosc_logo.png" alt="安同社区的logo">
    <div>
      <div class="name">{{ name }}</div>
      <div class="intro">{{ intro }}</div>
    </div>
  </div>
</template>

<script setup lang='ts'>
// 接收参数
const props = defineProps<{
  name: string,
  intro: string,
}>()
</script>

<style scoped>
.card {
  display: flex;
  width: 350px;
  height: 72px;
  border-radius: 5px;
}

.card:hover {
  background-color: rgb(231, 231, 231);
}

img {
  width: 52px;
  height: 52px;
  margin: 10px;
}

.name {
  margin-top: 14px;
  font-size: 20px;
  line-height: 20px;
}

.intro {
  margin-top: 4px;
  font-size: 14px;
}
</style>