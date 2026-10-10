<template>
    <section class="border-t-4 border-black bg-pink-200">
        <div class="max-w-6xl mx-auto px-6 py-20 text-center">
            <h2 class="text-3xl font-bold">Ready to launch your store?</h2>
            <p class="mt-4">Create your account and start building in minutes.</p>
            <NuxtLink to="/register"
                class="inline-block mt-8 border-2 border-black bg-yellow-300 px-6 py-3 font-bold shadow-[4px_4px_0_0_#000] hover:translate-x-0.5 hover:translate-y-0.5 hover:shadow-[2px_2px_0_0_#000] transition-all">
                Create account &rarr;
            </NuxtLink>
        </div>
    </section>

    <footer class="border-t-4 border-black bg-black text-yellow-300">
        <div class="max-w-6xl mx-auto px-6 py-6 flex flex-col md:flex-row items-center justify-between gap-2 text-sm">
            <span>&copy; {{ year }} webedit99. All rights reserved.</span>
            <div class="flex gap-4">
                <NuxtLink to="/login" class="hover:underline">Login</NuxtLink>
                <NuxtLink to="/register" class="hover:underline">Register</NuxtLink>
            </div>
        </div>
    </footer>
</template>

<script setup>
const year = new Date().getFullYear()
</script>