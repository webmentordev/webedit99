<template>
    <div class="border-4 border-black bg-white shadow-[6px_6px_0_0_#000] font-mono">
        <div class="flex items-center justify-between border-b-4 border-black bg-orange-300 px-4 py-2">
            <h3 class="font-bold">{{ title }}</h3>
            <span class="text-xs font-bold">[ FAIL ]</span>
        </div>
        <p class="p-5 text-sm">{{ text }}</p>
    </div>
</template>

<script setup>
defineProps({
    title: {
        type: String,
        default: 'Failed'
    },
    text: {
        type: String,
        required: true
    }
})
</script>