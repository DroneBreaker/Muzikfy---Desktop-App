<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <main class="container">
    <h1>Welcome to Muzikfy.</h1>

    <center>
      <form class="row" @submit.prevent="greet">
        <input id="greet-input" v-model="name" placeholder="Enter a name..." />
        <button style="margin-left: 20px;" type="submit">Greet</button>
      </form>
      <p>{{ greetMsg }}</p>
    </center>
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
