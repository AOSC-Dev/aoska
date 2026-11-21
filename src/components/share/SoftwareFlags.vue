<template>
  <section class="software-flag">
    <component v-if="icon" :is="icon" class="icon" aria-hidden="true" />
    <span v-else class="unknown-icon" aria-hidden="true">ⓘ</span>
    <div>
      <h3>{{ $t(`detail.fields.${field}`) }} · {{ $t(`detail.values.${value}`) }}</h3>
      <p>{{ $t(`detail.disclosureDescriptions.${field}.${value}`) }}</p>
    </div>
  </section>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import VerifiedIcon from '../../assets/icons/verified.svg';
import NonNativeIcon from '../../assets/icons/non-native.svg';
import ServiceLimitedIcon from '../../assets/icons/service-limited.svg';
import WindowsIcon from '../../assets/icons/windows.svg';
import TelemetryIcon from '../../assets/icons/telemetry.svg';
import UnverifiedIcon from '../../assets/icons/unofficial.svg';

const props = defineProps<{ field: string; value: string }>();
const icon = computed(() => {
  if (props.field === 'publisherRelationship') return UnverifiedIcon;
  if (props.field === 'availability') {
    if (props.value === 'verified') return VerifiedIcon;
    if (props.value === 'unverified' || props.value === 'broken') return UnverifiedIcon;
  }
  if (props.field === 'runtime' && props.value === 'compatibility-layer') return NonNativeIcon;
  if (props.field === 'runtime' && props.value === 'windows') return WindowsIcon;
  if (props.field === 'telemetry' && props.value === 'present') return TelemetryIcon;
  if (props.field === 'service' && props.value === 'limited') return ServiceLimitedIcon;
  if (props.field === 'support' && props.value === 'aosc') return VerifiedIcon;
  return undefined;
});
</script>

<style scoped>
.software-flag { display: flex; align-items: center; gap: 14px; padding: 14px 16px; background: var(--color-surface-raised); border: 1px solid var(--color-border); border-radius: var(--radius-card); }
h3 { font-size: 19px; font-weight: 500; line-height: 1.35; }
p { margin-top: 3px; font-size: 13px; line-height: 1.5; color: var(--color-text-muted); }
.icon, .unknown-icon { width: 42px; height: 42px; flex: 0 0 42px; }
.unknown-icon { display: grid; place-items: center; color: var(--color-text-subtle); font-size: 30px; }
</style>
