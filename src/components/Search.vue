<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { BANGS } from "../config/bangs";
import LoadingSpinner from '../components/LoadingSpinner.vue'

const results = defineModel();
const search = ref<string>("");
const loading = ref<boolean>(false)

type ValidatedQuery = {
  site: string;
  search: string;
}

function validateQuery(query: string): ValidatedQuery | null {
  if (!query) return null;

  const matches = query.match(/\:(\w+) (.*)/);

  if (!matches?.length || !matches[1] || !matches[2]) return null;

  const bang = matches[1].trim();
  const search = matches[2].trim();

  const selectedBang = BANGS.find(b => b.bang === bang);

  if (!selectedBang || !search) return null;

  return { site: selectedBang.site, search };
}

async function executeSearch() {
  const validQuery = validateQuery(search.value);

  if (!validQuery) return;

  loading.value = true;
  results.value = await invoke("search", { site: validQuery.site, search: validQuery.search });
  loading.value = false;
}

// TODO: Add bang autocomplete to input
</script>

<template>
  <div class="flex items-center py-3">
    <img class="rounded-lg mr-3" src="../assets/logo.svg" width="50" />
    <form class="flex justify-between w-full" @submit.prevent="executeSearch">
      <input autoFocus class="w-full text-white placeholder-zinc-400" v-model="search" placeholder=":bang Search..." />
      <button :disabled="loading || !search"
        class="flex items-center bg-zinc-900 text-sm rounded-full ml-2 px-3 py-1 border border-zinc-800" type="submit">
        <div v-if="loading">
          <LoadingSpinner />
        </div>
        <span v-else>Enter</span>
      </button>
    </form>
  </div>
</template>
