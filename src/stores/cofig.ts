import { defineStore } from 'pinia';
import { getEndpointBaseUrl } from '../utils/wrapper';

export const useConfigStore = defineStore('config', {
  state: () => {
    return { endpoint: "" };
  },
  actions: {
    async loadStaticEndpoint() {
      this.endpoint = await getEndpointBaseUrl();
    }
  }
});
