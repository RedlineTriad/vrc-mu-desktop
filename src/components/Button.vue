<script setup lang="ts"></script>

<template>
  <button type="submit"><slot /></button>
</template>

<style scoped>
button {
  border: none;
  border-radius: var(--border-radius);
  background-color: var(--primary-color);
  color: var(--dark-text-color);
  font-size: 1.5em;
  padding: calc(var(--padding) / 2);
  font-weight: 600;
  cursor: pointer;
  transition: background-color 0.3s ease;
}
button:hover {
  background-color: var(--primary-color-dark);
}
button:disabled {
  background-color: var(--background-color-3);
  cursor: not-allowed;
}
</style>
