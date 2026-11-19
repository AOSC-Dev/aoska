<template>
  <ViewAllPage :category="category" />
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useRoute } from 'vue-router';
import ViewAllPage from './ViewAllPage.vue';

const route = useRoute();
const category = computed(() => route.params.categoryName === 'working' ? 'office' : String(route.params.categoryName));
</script>
