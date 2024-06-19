<script setup lang="ts">
import { shell } from '@tauri-apps/api';
import { SearchResponse } from '../types/search-response';

const { item } = defineProps<{
    item?: SearchResponse,
}>();

async function openInBrowser() {
    if (!item?.href) return;

    await shell.open(item.href)
}
</script>

<template>
    <div v-if="item"
        class="overflow-y-auto overflow-x-hidden fixed top-0 right-0 left-0 z-50 justify-center items-center w-full md:inset-0 h-full">
        <div class="relative p-4 w-full h-full">
            <div class="relative bg-zinc-900 rounded-lg shadow h-full">
                <div class="flex items-center justify-between p-4 md:p-5 rounded-t">
                    <span>{{ item.title }}</span>
                    <button type="button"
                        class="end-2.5 text-gray-400 bg-transparent hover:bg-zinc-700 rounded-lg text-sm w-8 h-8 ms-auto inline-flex justify-center items-center"
                        @click="$emit('close')">
                        <svg class="w-3 h-3" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none"
                            viewBox="0 0 14 14">
                            <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                d="m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6" />
                        </svg>
                    </button>
                </div>

                <div class="p-4 md:p-5 h-5/6">
                    <iframe class="w-full h-full" v-if="item.href" :src="item.href" frameborder="0"></iframe>
                </div>

                <div class="flex justify-end px-4">
                    <div class="flex items-center gap-1" @click="openInBrowser">
                        <i class="ti ti-external-link"></i>
                        <span>Open in browser</span>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<style></style>
