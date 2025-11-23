<script lang="ts" setup>
const { id, label, type } = defineProps<{
  id: string;
  label: string;
  type: string;
}>();

const model = defineModel<string>();
</script>

<template>
  <div class="text-input">
    <label :for="id">{{ label }}</label>
    <input :id="id" :type="type" v-model="model" />
  </div>
</template>

<style scoped>
.text-input {
  display: flex;
  flex-direction: column;
}

label {
  color: var(--primary-color);
}

input {
  padding: calc(var(--padding) / 2);
  border: none;
  border-bottom: 2px solid var(--primary-color-dark);
  background: var(--background-color-2);
  color: var(--light-text-color);
}

input:focus {
  outline: none;
  border-bottom: 2px solid var(--primary-color);
  box-shadow: 0 4px 6px var(--primary-color-dark);
  transition:
    border-bottom 0.2s ease-out,
    box-shadow 0.2s ease-out;
}
</style>
