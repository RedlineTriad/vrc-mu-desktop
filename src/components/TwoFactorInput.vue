<script lang="ts" setup>
import { onMounted, useTemplateRef } from "vue";
const { id, autoFocus = false } = defineProps<{
  id: string;
  autoFocus?: boolean;
}>();

const model = defineModel<string>();

const isNumber = (event: KeyboardEvent) => {
  if (event.key < "0" || event.key > "9") {
    event.preventDefault();
  }
};

const inputRef = useTemplateRef("input");

onMounted(() => {
  if (autoFocus) {
    inputRef?.value?.focus();
  }
});
</script>

<template>
  <div class="twofactor-input">
    <input
      :id="id"
      autocomplete="one-time-code"
      type="text"
      v-model="model"
      inputmode="numeric"
      maxlength="6"
      pattern="\d{6}"
      @keypress="isNumber($event)"
      :style="{ '--_otp-digit': inputRef?.selectionStart ?? 0 }"
      ref="input"
    />
  </div>
</template>

<style scoped>
.twofactor-input {
  display: relative;
  width: fit-content;
  margin: auto;
  background-color: var(--background-color-2);
  padding: calc(var(--padding) / 2);
  border-radius: var(--border-radius);
  border-bottom: 2px solid var(--primary-color-dark);
}
.twofactor-input:focus-within {
  border-bottom: 2px solid var(--primary-color);
  box-shadow: 0 4px 6px var(--primary-color-dark);
  transition:
    border-bottom 0.2s ease-out,
    box-shadow 0.2s ease-out;
}

label {
  text-align: center;
  color: var(--primary-color);
}

input {
  all: unset;

  margin-right: -1ch; /* I don't quite understand why this is necessary but it is */

  --otp-bg: var(--background-color-1);
  --otp-cc: var(--light-text-color);
  --otp-fz: 1.75em;
  --otp-pb: 0.5ch;

  /* stolen from https://dev.to/madsstoumann/using-a-single-input-for-one-time-code-352l */
  --otp-digits: 6;
  --otp-ls: 2ch;
  --otp-gap: 1.25;

  /* private consts */
  --_otp-bgsz: calc(var(--otp-ls) + 1ch);
  --_otp-digit: 0;

  background:
    linear-gradient(
      90deg,
      var(--otp-bg, #bbb) calc(var(--otp-gap) * var(--otp-ls)),
      transparent 0
    ),
    linear-gradient(
      90deg,
      var(--otp-bg, #eee) calc(var(--otp-gap) * var(--otp-ls)),
      transparent 0
    );
  background-position:
    calc(var(--_otp-digit) * var(--_otp-bgsz)) 0,
    0 0;
  background-repeat: no-repeat, repeat-x;
  background-size: var(--_otp-bgsz) 100%;
  caret-color: var(--otp-cc, #222);
  caret-shape: block;
  clip-path: inset(0% calc(var(--otp-ls) / 2) 0% 0%);
  font-family: ui-monospace, monospace;
  font-size: var(--otp-fz, 2.5em);
  inline-size: calc(
    var(--otp-digits) * var(--_otp-bgsz) + 1px
  ); /* to prevent layout shift I had to add a +1*/
  letter-spacing: var(--otp-ls);
  padding-block: var(--otp-pb, 1ch);
  padding-inline-start: calc(((var(--otp-ls) - 1ch) / 2) * var(--otp-gap));
}

input::-webkit-outer-spin-button,
input::-webkit-inner-spin-button {
  display: none;
  -webkit-appearance: none;
}
</style>
