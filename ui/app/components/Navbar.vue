<template>
    <header class="border-b-4 border-black bg-white">
        <div class="max-w-6xl mx-auto px-6 py-4 flex items-center justify-between">
            <NuxtLink to="/" class="text-xl font-bold bg-black text-yellow-300 px-3 py-1">webedit99_</NuxtLink>
            <nav class="flex items-center gap-6 text-sm font-bold">
                <NuxtLink to="/#features" class="hover:underline">Features</NuxtLink>
                <NuxtLink to="/#how" class="hover:underline">How it works</NuxtLink>
                <NuxtLink to="/login" class="hover:underline">Login</NuxtLink>
                <NuxtLink to="/register"
                    class="border-2 border-black bg-pink-300 px-4 py-2 shadow-[4px_4px_0_0_#000] hover:translate-x-0.5 hover:translate-y-0.5 hover:shadow-[2px_2px_0_0_#000] transition-all">
                    Get started
                </NuxtLink>
            </nav>
        </div>
    </header>
</template>