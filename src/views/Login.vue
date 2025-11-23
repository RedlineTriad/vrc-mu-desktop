<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import TextInput from "../components/TextInput.vue";
import TwoFactorInput from "../components/TwoFactorInput.vue";
import Button from "../components/Button.vue";
import { invoke } from "@tauri-apps/api/core";

const emit = defineEmits<{
  (e: "login"): void;
}>();

const username = ref("");
const password = ref("");

const twofaRequired = ref(false);
const twofa = ref("");

const loginPending = ref(false);
const loginError = ref(null as string | null);

onMounted(async () => {
  // Potentially check if already logged in and set currentView accordingly
  const loggedIn = await invoke("am_logged_in");
  if (loggedIn) {
    emit("login");
  }
});

const handleLogin = async () => {
  // Handle login logic here
  console.log(
    `Username: ${username.value}, Password: ${password.value}, 2FA: ${twofa.value}`,
  );
  try {
    loginError.value = null;
    loginPending.value = true;
    const resp = await invoke("login", {
      username: username.value,
      password: password.value,
      twoFaCode: twofa.value || null,
    });
    console.log(resp);
    emit("login");
  } catch (e) {
    console.error(e);
    loginError.value = e as string;
    if (e === "2FA code required") {
      twofaRequired.value = true;
    }
  } finally {
    loginPending.value = false;
  }
};

const isFormValid = computed(() => {
  return (
    username.value.trim() !== "" &&
    password.value.trim() !== "" &&
    (!twofaRequired.value || twofa.value.length === 6)
  );
});
</script>

<template>
  <div class="centered">
    <form @submit.prevent="handleLogin">
      <h1>Log Into Your <span class="vrchat-logo">VRChat</span> Account</h1>
      <TextInput
        id="username"
        label="Username"
        type="text"
        v-model="username"
      />
      <TextInput
        id="password"
        label="Password"
        type="password"
        v-model="password"
      />
      <TwoFactorInput
        id="twofa"
        :auto-focus="true"
        v-model="twofa"
        v-if="twofaRequired"
      />
      <Button :disabled="!isFormValid || loginPending" type="submit"
        >Login</Button
      >
      <output v-if="loginError">
        {{ loginError }}
      </output>
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

output {
  background-color: var(--error-background-color);
  border: 1px solid var(--error-color);
  padding: calc(var(--padding) / 2);
  border-radius: var(--border-radius);
  text-align: center;
  animation: pulse 2s infinite;
}
</style>
