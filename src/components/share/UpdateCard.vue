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
            {{ $t(`${prefix}.greeting1`) }}<span class="number small-number">{{ update }}</span>{{ $t(`${prefix}.greeting2`) }}
          </template>
          <template v-else>
            {{ $t(`${prefix}.greeting`) }}
          </template>
        </div>
        <button class="button" type="button">{{ $t(`${prefix}.button`) }}</button>
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
  securityClassificationAvailable: {
    type: Boolean,
    default: true
  },
  isLoading: {
    type: Boolean,
    default: false
  }
});

const hasUpdates = computed(() => props.update > 0);
const hasSecurityUpdate = computed(() => props.updateSecurity > 0);
const hasUnknownSecurityClassification = computed(() => hasUpdates.value && !props.securityClassificationAvailable);

const updateClass = computed(() => {
  if (props.isLoading) return 'checking-update';
  if (props.updateSecurity > 0) return 'security-update';
  if (props.update > 0 && !props.securityClassificationAvailable) return 'unknown-security-update';
  if (props.update > 0) return 'system-update';
  return 'no-update';
});

const prefix = computed(() => {
  if (props.isLoading) return 'checkingUpdate';
  if (props.updateSecurity > 0) return 'securityUpdate';
  if (hasUnknownSecurityClassification.value) return 'unknownSecurityUpdate';
  if (props.update > 0) return 'systemUpdate';
  return 'noUpdate';
});
</script>

<style scoped>
.update-card {
  display: flex;
  flex-direction: column;
  overflow: hidden;
  border: 1px solid transparent;
  border-radius: var(--radius-card);
  box-shadow: var(--shadow-card);
}

.card-content {
  display: flex;
  flex-direction: column;
  min-height: 100%;
  flex: 1;
  padding: 25px;
}

.checking-update,
.system-update,
.unknown-security-update {
  border-color: var(--color-info-border);
  background-color: var(--color-info);
}

.no-update {
  border-color: var(--color-success-border);
  background-color: var(--color-success);
}

.security-update {
  border-color: var(--color-security-border);
  background-color: var(--color-security);
}

.title {
  color: var(--color-text);
  font-size: 32px;
  line-height: 38px;
  font-weight: 700;
}

.status {
  margin-top: 14px;
  color: var(--color-text);
  font-size: 16px;
}

.number {
  margin: 0 5px;
  font-size: 32px;
  line-height: 20px;
  font-weight: 600;
}

.small-number {
  font-size: 22px;
}

.greeting {
  margin-top: 8px;
  color: var(--color-text-muted);
  font-size: 16px;
  line-height: 24px;
}

.button {
  align-self: flex-end;
  min-width: 96px;
  height: 35px;
  margin-top: auto;
  border: 1px solid var(--color-border-strong);
  border-radius: var(--radius-control);
  background: rgba(255, 255, 255, 0.03);
  color: var(--color-text);
  font-size: 16px;
  cursor: pointer;
}

.button:hover {
  border-color: var(--color-text);
}

.skeleton {
  background: linear-gradient(90deg, rgba(255,255,255,0.09) 25%, rgba(255,255,255,0.16) 37%, rgba(255,255,255,0.09) 63%);
  background-size: 400% 100%;
  animation: skeleton-loading 1.2s ease-in-out infinite;
  border-radius: 4px;
}

.skeleton-title {
  width: 220px;
  height: 38px;
  margin-bottom: 12px;
}

.skeleton-status {
  width: 180px;
  height: 22px;
  margin-bottom: 10px;
}

.skeleton-greeting {
  width: 240px;
  height: 22px;
  margin-bottom: 12px;
}

.skeleton-button {
  width: 96px;
  height: 35px;
}

@keyframes skeleton-loading {
  0% { background-position: 100% 0; }
  100% { background-position: 0 0; }
}

.fade-enter-active, .fade-leave-active {
  transition: opacity 0.25s cubic-bezier(.4,0,.2,1);
}
.fade-enter-from, .fade-leave-to { opacity: 0; }
.fade-enter-to, .fade-leave-from { opacity: 1; }
</style>
