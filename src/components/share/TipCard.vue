<template>
  <div class="card">
    <div class="date">
      <div class="day">{{ day }}</div>
      <div class="month">{{ month }}</div>
    </div>
    <div>
      <div class="title">{{ title }}</div>
      <div class="intro">{{ intro }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
// 接收参数
defineProps<{
  month: string,
  day: number,
  title: string,
  intro: string,
}>();
</script>

<style scoped>
.card {
  width: 100%;
  height: 72px;
  border-radius: 5px;
  display: flex;
}

.card:hover {
  background-color: rgb(231, 231, 231);
}

.date {
  width: 32px;
  height: 52px;
  margin: 8px;
  color: rgb(84, 84, 84);
}

.day {
  font-size: 30px;
  line-height: 28px;
  text-align: center;
  margin-bottom: 10px;
}

.month {
  font-size: 16px;
  line-height: 16px;
  margin-top: 8px;
  text-align: center;
}

.title {
  font-size: 20px;
  line-height: 20px;
  margin: 15px 0 10px 5px;
  font-weight: 500;
}

.intro {
  font-size: 14px;
  line-height: 14px;
  margin-left: 5px;
}
</style>
