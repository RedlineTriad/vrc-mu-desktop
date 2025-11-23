<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import Button from "../components/Button.vue";
import TextInput from "../components/TextInput.vue";
import { invoke } from "@tauri-apps/api/core";

const emit = defineEmits<{
  (e: "submitApiKey"): void;
}>();

const apiKey = ref("");

onMounted(async () => {
  const storedApiKey = await invoke("get_api_key");
  if (storedApiKey) {
    apiKey.value = storedApiKey as string;
    emit("submitApiKey");
  }
});

const isFormValid = computed(() => {
  return apiKey.value.length === 64;
});

const handleSubmit = () => {
  console.log(`API Key: ${apiKey.value}`);
  invoke("set_api_key", { apiKey: apiKey.value });
  emit("submitApiKey");
};
</script>

<template>
  <div class="centered">
    <form @submit.prevent="handleSubmit">
      <h1>
        Provide your <span class="vrchat-logo">Mutual Friends</span> API Key
      </h1>
      <TextInput id="api-key" label="API Key" type="text" v-model="apiKey" />
      <Button :disabled="!isFormValid" type="submit">Use this API key</Button>
    </form>
  </div>
</template>

<style scoped>
.centered {
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
}

.vrchat-logo {
  color: var(--secondary-color);
}

form {
  display: flex;
  flex-direction: column;
  gap: var(--padding);
  padding: var(--padding);
  border-radius: var(--border-radius);
  background-color: var(--background-color-1);
  box-shadow: 0 0px 12px var(--background-color-1);
}
</style>
