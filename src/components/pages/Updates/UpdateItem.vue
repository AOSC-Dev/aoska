<template>
  <div 
    class="tum-update"
    :style="{ backgroundColor: tumUpdate.is_security ? 'var(--color-bg-light-coral)' : 'rgb(231,231,231)' }"
  >
    <div class="title">{{ getLangText(tumUpdate.name, lang) }}</div>
    <div class="caution" v-if="tumUpdate.caution">{{ getLangText(tumUpdate.caution, lang) }}</div>
    <br>
    <ul class="update-packages">
      <li
        v-for="(pkg, index) in packageVersions"
        :key="index"
      >{{ pkg.name }}<span v-if="pkg.version"> ({{ pkg.version }})</span></li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import { OmaOperation, TumUpdateInfo } from '../../../types/oma';

const props = defineProps<{ tumUpdate: TumUpdateInfo, updateDetail: OmaOperation | null}>();
const lang = ref(navigator.language.replace("-", "_"));

function getLangText(obj: Record<string, string> | undefined, lang: string, fallback = "default") {
  if (!obj) return "";
  return obj[lang] || obj[fallback] || Object.values(obj)[0] || "";
}

const packageVersions = computed(() => {
  return props.tumUpdate.package_names.map(name => {
    const found = props.updateDetail?.install.find((v) => v.name_without_arch === name);
    return {
      name,
      version: found?.new_version
    };
  });
});

</script>

<style scoped>
.tum-update {
  flex-grow: 1;
  max-height: 220px;
  margin: auto 20px 20px 20px;
  border-radius: 5px;
  background-color: rgb(231, 231, 231);
  padding-top: 30px;
  padding-left: 50px;
  padding-bottom: 50px;
}


.title {
  font-size: 29px;
  font-weight: 700;
}

.caution {
  font-size: 16px;
}

.update-packages {
  list-style: none;
  padding-left: 0;
}

.update-packages li {
  font-size: 16px;
  font-weight: 700;
  letter-spacing: 0.5px;
}

</style>