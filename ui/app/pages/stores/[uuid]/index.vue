<template>
    <h1>Welcome to your store</h1>
</template>
<script setup>
definePageMeta({
    middleware: 'auth',
    layout: 'store'
});

const route = useRoute().params.uuid;
</script>