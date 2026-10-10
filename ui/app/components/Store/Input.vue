<template>
    <input :value="props.modelValue" @input="emit('update:modelValue', $event.target.value)" autocomplete="off"
        class="w-full rounded-lg border border-gray-400 bg-white px-3 py-2 text-sm text-gray-900 placeholder-gray-500 shadow-inner transition-colors hover:bg-gray-50 focus:border-blue-600 focus:bg-white focus:outline-none focus:ring-2 focus:ring-blue-600 disabled:cursor-not-allowed disabled:bg-gray-100 disabled:text-gray-500">
</template>

<script setup>
const props = defineProps(['modelValue'])
const emit = defineEmits(['update:modelValue'])
</script>