<template>
  <div class="update-card" :class="updateClass">
    <transition name="fade" mode="out-in">
      <div v-if="isLoading" key="skeleton" class="card-content">
        <div class="title"><div class="skeleton skeleton-title"></div></div>
        <div class="status"><div class="skeleton skeleton-status"></div></div>
        <div class="greeting"><div class="skeleton skeleton-greeting"></div></div>
        <div class="button"><div class="skeleton skeleton-button"></div></div>
      </div>
      <div v-else key="content" class="card-content">
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
    </transition>
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
  },
  isLoading: {
    type: Boolean,
    default: false
  }
})

const hasUpdates = computed(() => props.update > 0)
const hasSecurityUpdate = computed(() => props.updateSecurity > 0)

const updateClass = computed(() => {
  if (props.isLoading) return 'loading'
  if (props.updateSecurity > 0) return 'security-update'
  if (props.update > 0) return 'system-update'
  return 'no-update'
})

const prefix = computed(() => {
  if (props.updateSecurity > 0) return 'securityUpdate'
  if (props.update > 0) return 'systemUpdate'
  return 'noUpdate'
})
</script>

<style scoped>
.update-card {
  display: flex;
  flex-direction: column;
  border-radius: 5px;
}

.card-content {
  display: flex;
  flex-direction: column;
  flex: 1;
}

.loading {
  background-color: #f7f7f7;
}

.no-update {
  background-color: var(--color-bg-light-green);
}

.system-update {
  background-color: var(--color-bg-light-blue);
}

.security-update {
  background-color: var(--color-bg-light-coral);
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

.skeleton {
  background: linear-gradient(90deg, #ececec 25%, #f3f3f3 37%, #ececec 63%);
  background-size: 400% 100%;
  animation: skeleton-loading 1.2s ease-in-out infinite;
  border-radius: 4px;
}

.skeleton-title {
  width: 180px;
  height: 32px;
  margin-bottom: 6px;
}

.skeleton-status {
  width: 120px;
  height: 22px;
  margin-bottom: 6px;
}

.skeleton-greeting {
  width: 140px;
  height: 22px;
  margin-bottom: 6px;
}

.skeleton-button {
  width: 120px;
  height: 35px;
}

@keyframes skeleton-loading {
  0% {
    background-position: 100% 0;
  }
  100% {
    background-position: 0 0;
  }
}

.fade-enter-active, .fade-leave-active {
  transition: opacity 0.4s cubic-bezier(.4,0,.2,1);
}
.fade-enter-from, .fade-leave-to {
  opacity: 0;
}
.fade-enter-to, .fade-leave-from {
  opacity: 1;
}
</style>