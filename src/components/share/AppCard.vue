<template>
  <div class="card">
    <img :src="image" />
    <div>
      <div class="name">{{ name }}</div>
      <div class="intro">{{ intro }}</div>
    </div>
  </div>
</template>

<script setup lang='ts'>
// 接收参数
defineProps<{
  name: string,
  intro: string,
  image: string
}>();
</script>

<style scoped>
.card {
  display: flex;
  flex-direction: row;
  align-items: center;
  padding: 10px;
  border-radius: 5px;
  height: 72px;
  background-color: #f7f7f7;
  transition: background-color 0.2s;
  box-sizing: border-box;
  cursor: pointer;
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
  font-size: 20px;
  line-height: 20px;
  font-weight: 500;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.intro {
  font-size: 14px;
  color: #666;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
</style>
