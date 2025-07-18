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
    validator: (value) => ['green', 'grey', 'yellow', 'red', ''].includes(value)
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
  border: 1px solid black;
  border-radius: 5px;
  font-size: 18px;
  text-align: center;
  cursor: pointer;
  transition: all 0.3s ease;
}

.small {
  font-size: 14px;
}

.medium {
  font-size: 18px;
}

.green {
  background-color: #ceffd6;
}

.grey {
  background-color: #f3f3f3;
}

.yellow {
  background-color: #fad8b9;
}

.red {
  background-color: #ffd9ce;
}
</style>