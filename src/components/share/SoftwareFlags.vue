<template>
  <div class="software-flags">
    <component :is="icons[type]" class="img"/>
    <div class="info">
      <div class="title">{{ $t(`tags.${type}.title`) }}</div>
      <div class="description">{{ $t(`tags.${type}.description`) }}</div>
    </div>
  </div>
</template>

<script setup>
import VerifiedIcon from '../../assets/icons/verified.svg';
import NonNativeIcon from '../../assets/icons/non-native.svg';
import ServiceLimitedIcon from '../../assets/icons/service-limited.svg';
import UnofficialIcon from '../../assets/icons/unofficial.svg';
import WindowsIcon from '../../assets/icons/windows.svg';
import TelemetyIcon from '../../assets/icons/telemetry.svg';

defineProps({
  type: {
    type: String,
    validator: (value) => ['verified', 'non_native', 'service_limited', 'unofficial', 'windows_app', 'telemetry'].includes(value)
  }
});

const icons = {
  "verified": VerifiedIcon,
  "non_native": NonNativeIcon,
  "service_limited": ServiceLimitedIcon,
  "unofficial": UnofficialIcon,
  "windows_app": WindowsIcon,
  "telemetry": TelemetyIcon
};
</script>

<style scoped>
.software-flags {
  display: flex;
  width: 100%;
  min-height: 92px;
  border: 1px solid transparent;
  border-radius: var(--radius-card);
  background-color: var(--color-surface-raised);
}

.software-flags:hover {
  border-color: var(--color-border);
}

.info {
  min-width: 0;
  padding: 16px 16px 16px 0;
}

.title {
  color: var(--color-text);
  font-size: 23px;
  line-height: 30px;
  font-weight: 500;
}

.description {
  margin-top: 6px;
  color: var(--color-text-muted);
  line-height: 18px;
}

.img {
  width: 58px;
  height: 58px;
  margin: 17px 20px;
  filter: drop-shadow(0 3px 4px rgba(0, 0, 0, 0.45));
}
</style>
