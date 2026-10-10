<template>
    <input :value="props.modelValue" @input="emit('update:modelValue', $event.target.value)" autocomplete="off"
        :class="[$attrs.class, 'border-2 border-black bg-white px-4 py-3 text-sm focus:outline-none focus:bg-yellow-100 shadow-[3px_3px_0_0_#000]']"
        v-bind="{ ...$attrs, class: undefined }">
</template>

<script setup>
const props = defineProps(['modelValue'])
const emit = defineEmits(['update:modelValue'])
</script>