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
import ServiceRestricedIcon from '../../assets/icons/service-restricted.svg';
import UnofficialIcon from '../../assets/icons/unofficial.svg';
import WindowsIcon from '../../assets/icons/windows.svg';
import TelemetyIcon from '../../assets/icons/telemetry.svg';

const props = defineProps({
  type: {
    type: String,
    validator: (value) => ['verified', 'nonNative', 'serviceRestricted', 'unofficial', 'windows', 'telemetry'].includes(value)
  }
})

const icons = {
  "verified": VerifiedIcon,
  "nonNative": NonNativeIcon,
  "serviceRestricted": ServiceRestricedIcon,
  "unofficial": UnofficialIcon,
  "windows": WindowsIcon,
  "telemetry": TelemetyIcon
}

</script>

<style scoped>
.software-flags {
  display: flex;
  width: 500px;
  height: 100px;
  border-radius: 5px;
  background-color: lightgray;
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