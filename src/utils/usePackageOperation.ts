import { computed, reactive, toRefs } from 'vue';
import { onBeforeRouteLeave, onBeforeRouteUpdate } from 'vue-router';
import { getPackageManager } from './pmClient';
import { createPackageOperationFlow, type PackageOperationFlowState } from './pmOperations';

export function usePackageOperation(onFinished: () => Promise<void>) {
  const state = reactive<PackageOperationFlowState>({ plan: null, activeOperation: null, busy: false, submitting: false, error: '' });
  const locked = computed(() => state.busy || state.submitting);
  onBeforeRouteLeave(() => !locked.value);
  onBeforeRouteUpdate(() => !locked.value);
  return { ...toRefs(state), locked, ...createPackageOperationFlow(getPackageManager(), state, onFinished) };
}
