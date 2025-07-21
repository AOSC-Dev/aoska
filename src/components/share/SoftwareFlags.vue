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

const props = defineProps({
  type: {
    type: String,
    validator: (value) => ['verified', 'non_native', 'service_limited', 'unofficial', 'windows_app', 'telemetry'].includes(value)
  }
})

const icons = {
  "verified": VerifiedIcon,
  "non_native": NonNativeIcon,
  "service_limited": ServiceLimitedIcon,
  "unofficial": UnofficialIcon,
  "windows_app": WindowsIcon,
  "telemetry": TelemetyIcon
}

</script>

<style scoped>
.software-flags {
  display: flex;
  width: 520px;
  height: 100px;
  border-radius: 5px;
  background-color: #f3f3f3;
}

.title {
  margin-top: 20px;
  font-size: 24px;
  line-height: 30px;
  font-weight: 500;
}

.description {
  margin-top: 8px;
  line-height: 16px;
}

.img {
  width: 60px;
  height: 60px;
  margin: 20px;
}
</style>