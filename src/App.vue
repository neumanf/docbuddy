<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import Search from "./components/Search.vue";
import ResultsList from "./components/ResultsList.vue";
import WebsitePreviewModal from "./components/WebsitePreviewModal.vue";
import { setWindowsSizeAsCompact, setWindowsSizeAsExpanded } from "./utils/window";
import { SearchResponse } from "./types/search-response";

const searchResults = ref<SearchResponse[]>([]);

onMounted(async () => {
  await setWindowsSizeAsCompact();
})

watch(searchResults, async results => {
  if (!results.length) {
    await setWindowsSizeAsCompact();
  } else {
    await setWindowsSizeAsExpanded();
  }
})

const isModalVisible = ref<boolean>(false)
const selectedItem = ref<SearchResponse>()

function showModal(item: SearchResponse) {
  selectedItem.value = item;
  isModalVisible.value = true;
}

function closeModal() {
  isModalVisible.value = false;
}
</script>

<template>
  <div class="py-4 px-6 gap-2" :class="{
    'divide-y divide-zinc-500': searchResults.length
  }">
    <Search v-model="searchResults" />

    <ResultsList :search-results="searchResults" @showModal="showModal" />
  </div>

  <WebsitePreviewModal v-show="isModalVisible" :item="selectedItem" @close="closeModal" />
</template>

<style scoped></style>
