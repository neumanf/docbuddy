<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const result = ref("");
const search = ref("");

async function executeSearch() {
  result.value = await invoke("search", { search: search.value });
}
</script>

<template>
  <div class="flex items-center py-3">
    <img class="rounded-lg mr-3" src="../assets/logo.svg" width="50" />
    <form class="flex justify-between w-full" @submit.prevent="executeSearch">
      <input autoFocus class="w-full text-white placeholder-zinc-400" v-model="search" placeholder=":bang Search..." />
      <button class="bg-zinc-900 text-sm rounded-full ml-2 px-3 py-1 border border-zinc-800"
        type="submit">Enter</button>
    </form>
  </div>
</template>
