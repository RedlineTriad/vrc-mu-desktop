<script setup lang="ts">
import { ref } from "vue";
import Login from "./views/Login.vue";
import Upload from "./views/Upload.vue";
import ApiKey from "./views/ApiKey.vue";

const currentView = ref("login");

const handleLogin = () => {
  currentView.value = "api-key";
};

const handleSubmitApiKey = () => {
  console.log("API Key submitted, moving to upload view");
  currentView.value = "upload";
};
</script>

<template>
  <main>
    <Login v-if="currentView === 'login'" @login="handleLogin" />
    <ApiKey
      v-else-if="currentView === 'api-key'"
      @submit-api-key="handleSubmitApiKey"
    />
    <Upload v-else-if="currentView === 'upload'" />
  </main>
</template>

<style>
:root {
  --primary-color-luminance: 55.06%;
  --primary-color-chroma: 0.23;
  --primary-color-hue: 295.1;
  --primary-color: oklch(
    var(--primary-color-luminance) var(--primary-color-chroma)
      var(--primary-color-hue)
  );
  --primary-color-dark: oklch(
    calc(var(--primary-color-luminance) - 30%) var(--primary-color-chroma)
      var(--primary-color-hue)
  );
  --primary-color-very-dark: oklch(
    calc(var(--primary-color-luminance) - 50%) var(--primary-color-chroma)
      var(--primary-color-hue)
  );
  --primary-color-very-very-dark: oklch(
    calc(var(--primary-color-luminance) - 70%) var(--primary-color-chroma)
      var(--primary-color-hue)
  );

  --secondary-color-luminance: 72.92%;
  --secondary-color-chroma: 0.1751;
  --secondary-color-hue: 42.83;
  --secondary-color: oklch(
    var(--secondary-color-luminance) var(--secondary-color-chroma)
      var(--secondary-color-hue)
  );
  --secondary-color-dark: oklch(
    calc(var(--secondary-color-luminance) - 30%) var(--secondary-color-chroma)
      var(--secondary-color-hue)
  );

  --background-color-luminance: 10%;
  --background-color-chroma: 0;
  --background-color-hue: 0;
  --background-color: oklch(
    var(--background-color-luminance) var(--background-color-chroma)
      var(--background-color-hue)
  );
  --background-color-1: oklch(
    calc(var(--background-color-luminance) + 10%) var(--background-color-chroma)
      var(--background-color-hue)
  );
  --background-color-2: oklch(
    calc(var(--background-color-luminance) + 20%) var(--background-color-chroma)
      var(--background-color-hue)
  );
  --background-color-3: oklch(
    calc(var(--background-color-luminance) + 30%) var(--background-color-chroma)
      var(--background-color-hue)
  );
  --background-color-4: oklch(
    calc(var(--background-color-luminance) + 40%) var(--background-color-chroma)
      var(--background-color-hue)
  );

  --light-text-color: oklch(80% 0 0);
  --dark-text-color: oklch(20% 0 0);

  --error-color: oklch(60% 0.25 25);
  --error-background-color: oklch(20% 0.1 25);

  --border-radius: 8px;
  --padding: 16px;

  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-weight: 400;

  background-color: var(--background-color);
  color: var(--light-text-color);
}

body {
  background-image:
    repeating-radial-gradient(
      circle at 0 0,
      transparent 0,
      var(--background-color) 75px
    ),
    repeating-linear-gradient(
      var(--primary-color-very-dark),
      var(--primary-color-very-very-dark)
    );
}

html,
body,
main,
#app {
  height: 100%;
  width: 100%;
}

/* 1. Use a more-intuitive box-sizing model */
*,
*::before,
*::after {
  box-sizing: border-box;
}

/* 2. Remove default margin */
* {
  margin: 0;
}

/* 3. Enable keyword animations */
@media (prefers-reduced-motion: no-preference) {
  html {
    interpolate-size: allow-keywords;
  }
}

body {
  /* 4. Add accessible line-height */
  line-height: 1.5;
  /* 5. Improve text rendering */
  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

/* 6. Improve media defaults */
img,
picture,
video,
canvas,
svg {
  display: block;
  max-width: 100%;
}

/* 7. Inherit fonts for form controls */
input,
button,
textarea,
select {
  font: inherit;
}

/* 8. Avoid text overflows */
p,
h1,
h2,
h3,
h4,
h5,
h6 {
  overflow-wrap: break-word;
}

/* 9. Improve line wrapping */
p {
  text-wrap: pretty;
}

h1,
h2,
h3,
h4,
h5,
h6 {
  text-wrap: balance;
}

/*
  10. Create a root stacking context
*/
#root,
#__next {
  isolation: isolate;
}
</style>
