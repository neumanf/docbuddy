<script setup lang="ts">
import Search from "./components/Search.vue";
</script>

<template>
  <div class="py-3 px-6 gap-2 divide-y divide-zinc-500">
    <div class="flex items-center py-3">
      <img class="rounded-lg mr-3" src="./assets/logo.svg" width="50" />
      <Search />
    </div>
    <div class="py-3"></div>
  </div>
</template>

<style scoped></style>
