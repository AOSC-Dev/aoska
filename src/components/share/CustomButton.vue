<template>
  <button
    :class="['button', size, color]"
    :style="{
      width: width ? (isNaN(width) ? width : width + 'px') : null,
      height: height ? (isNaN(height) ? height : height + 'px') : null,
      lineHeight: height ? (isNaN(height) ? height : height + 'px') : null
    }"
  >
    <slot></slot>
  </button>
</template>

<script setup>
defineProps({
  size: {
    type: String,
    default: '',
    validator: (value) => ['small', 'medium', 'large', ''].includes(value)
  },
  color: {
    type: String,
    default: '',
    validator: (value) => ['green', 'grey', 'yellow', 'red', 'blue', ''].includes(value)
  },
  width: {
    type: [String, Number],
    default: null
  },
  height: {
    type: [String, Number],
    default: null
  }
});
</script>

<style scoped>
.button {
  margin: auto 15px 15px auto;
  min-width: 128px;
  min-height: 35px;
  padding: 0 18px;
  border: 1px solid var(--color-border-strong);
  border-radius: var(--radius-control);
  background-color: var(--color-button-grey);
  color: var(--color-text);
  font-size: 18px;
  text-align: center;
  cursor: pointer;
  transition: background-color 0.2s ease, border-color 0.2s ease, transform 0.2s ease;
}

.button:hover {
  border-color: var(--color-text);
  background-color: var(--color-surface-hover);
  transform: translateY(-1px);
}

.button:disabled {
  cursor: not-allowed;
  opacity: 0.48;
  transform: none;
}

.small {
  min-width: 96px;
  font-size: 14px;
}

.medium {
  font-size: 18px;
}

.large {
  min-height: 42px;
  font-size: 20px;
}

.green {
  background-color: var(--color-button-green);
}

.grey {
  background-color: var(--color-button-grey);
}

.yellow {
  background-color: var(--color-button-yellow);
}

.red {
  background-color: var(--color-button-red);
}

.blue {
  background-color: var(--color-button-blue);
}
</style>
