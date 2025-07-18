<template>
  <button class="button">
    <slot></slot>
  </button>
</template>

<style scoped>
.button {
  margin: auto 15px 15px auto;
  border: 1px solid black;
  border-radius: 5px;
  font-size: 18px;
  text-align: center;
}
</style>