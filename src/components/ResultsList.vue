<script setup lang="ts">
import { SearchResponse } from '../types/search-response';

const { searchResults } = defineProps<{ searchResults: SearchResponse[] }>();
</script>

<template>
    <div v-if="searchResults.length" class="flex flex-col pt-5">
        <div v-for="item of searchResults" class="mb-2 p-2 hover:bg-zinc-800 rounded-lg"
            @click="$emit('showModal', item)">
            <span class="line-clamp-1">{{ item.title }}</span>
            <p class="text-zinc-500 line-clamp-2">{{ item.description }}</p>
        </div>
    </div>
</template>