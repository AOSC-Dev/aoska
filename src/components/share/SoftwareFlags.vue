<template>
  <span class="software-flag">
    <component v-if="icon" :is="icon" class="icon" aria-hidden="true" />
    {{ $t(`detail.fields.${field}`) }} · {{ $t(`detail.values.${value}`) }}
  </span>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import VerifiedIcon from '../../assets/icons/verified.svg';
import NonNativeIcon from '../../assets/icons/non-native.svg';
import ServiceLimitedIcon from '../../assets/icons/service-limited.svg';
import WindowsIcon from '../../assets/icons/windows.svg';
import TelemetryIcon from '../../assets/icons/telemetry.svg';

const props = defineProps<{ field: string; value: string }>();
const icons = {
  verified: VerifiedIcon,
  'compatibility-layer': NonNativeIcon,
  windows: WindowsIcon,
  present: TelemetryIcon,
  limited: ServiceLimitedIcon,
};
const icon = computed(() => icons[props.value as keyof typeof icons]);
</script>

<style scoped>
.software-flag { display: inline-flex; align-items: center; gap: 6px; padding: 5px 10px; background: var(--color-surface-raised); border: 1px solid var(--color-border); border-radius: var(--radius-card); }
.icon { width: 18px; height: 18px; }
</style>
