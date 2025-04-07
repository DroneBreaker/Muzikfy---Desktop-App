<script setup>
import { RouterView } from "vue-router";
</script>

<template>
  <main class="bg-black h-[100vh] text-white">
    <RouterView/>
  </main>
</template>

<style scoped>
h1 {
  text-align: center;
  padding-top: 15rem;
}
</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}



@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: black;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}
</style>
