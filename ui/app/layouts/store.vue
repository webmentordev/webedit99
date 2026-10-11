<template>
    <main class="flex h-screen overflow-hidden bg-gray-50">
        <StoreNavbar />
        <nuxt-loading-indicator />
        <section class="flex-1 h-screen overflow-y-auto">
            <slot />
        </section>
    </main>
</template>