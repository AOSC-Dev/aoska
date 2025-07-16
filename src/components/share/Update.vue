<template>
  <div class="update-card" :class="updateClass">
    <div class="title">{{ $t(`${prefix}.title`) }}</div>
    <div class="status">
      <template v-if="hasSecurityUpdate">
        {{ $t(`${prefix}.status1`) }}<span class="number">{{ updateSecurity }}</span>{{ $t(`${prefix}.status2`) }}
      </template>
      <template v-else-if="hasUpdates">
        {{ $t(`${prefix}.status1`) }}<span class="number">{{ update }}</span>{{ $t(`${prefix}.status2`) }}
      </template>
      <template v-else>
        {{ $t(`${prefix}.status`) }}
      </template>
    </div>
    <div class="greeting">
      <template v-if="hasSecurityUpdate">
        {{ $t(`${prefix}.greeting1`) }}<span class="number">{{ update }}</span>{{ $t(`${prefix}.greeting2`) }}
      </template>
      <template v-else>
        {{ $t(`${prefix}.greeting`) }}
      </template>
    </div>
    <div class="button">{{ $t(`${prefix}.button`) }}</div>
  </div>
</template>

<script setup lang='ts'>
import { computed } from 'vue';

const props = defineProps({
  update: {
    type: Number,
    default: 0
  },
  updateSecurity: {
    type: Number,
    default: 0
  }
})

const hasUpdates = computed(() => props.update > 0)
const hasSecurityUpdate = computed(() => props.updateSecurity > 0)

const updateClass = computed(() => {
  if (props.updateSecurity > 0) return 'red-update'
  if (props.update > 0) return 'yellow-update'
  return 'green-update'
})

const prefix = computed(() => {
  if (props.updateSecurity > 0) return 'updateRed'
  if (props.update > 0) return 'updateYellow'
  return 'updateGreen'
})
</script>

<style scoped>
.update-card {
  display: flex;
  flex-direction: column;
  border-radius: 5px;
}

.green-update {
  background-color: rgb(206, 255, 214);
}

.yellow-update {
  background-color: rgb(255, 239, 206);
}

.red-update {
  background-color: rgb(255, 217, 206);
}

.title {
  padding: 15px 0 0 15px;
  font-size: 26px;
  font-weight: 700;
}

.status {
  padding: 5px 0 0 15px;
}

.number {
  font-size: 28px;
  margin: 0 3px;
}

.greeting {
  padding: 5px 0 0 15px;
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
</style>