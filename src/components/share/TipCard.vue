<template>
  <div class="card">
    <div class="date">
      <div class="day">{{ day }}</div>
      <div class="month">{{ month }}</div>
    </div>
    <div class="content">
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
  position: relative;
  width: 100%;
  height: 72px;
  border-radius: 5px;
  display: flex;
  overflow: hidden;
  cursor: pointer;
}

.card::before {
  content: "";
  position: absolute;
  top: 0; left: 0; bottom: 0;
  width: 72px;
  background-color: #f7f7f7;
  border-radius: 5px;
  transition: width 0.3s ease;
  z-index: 0;
}

.card:hover::before {
  width: 100%;
  border-radius: 5px;
}

.card > * {
  position: relative;
  z-index: 1;
}

.date {
  width: 72px;
  height: 72px;
  margin-right: 8px;
  background-color: #f7f7f7;
  border-radius: 5px;
  display: flex;
  flex-direction: column;
  justify-content: center;
  gap: 5px
}

.day {
  font-size: 30px;
  line-height: 28px;
  text-align: center;
}

.month {
  font-size: 16px;
  line-height: 16px;
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
