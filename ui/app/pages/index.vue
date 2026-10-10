<template>
    <div class="min-h-screen bg-yellow-50 text-black font-mono">
        <section class="max-w-6xl mx-auto px-6 py-24">
            <div class="border-4 border-black bg-white shadow-[8px_8px_0_0_#000]">
                <div class="flex items-center gap-2 border-b-4 border-black bg-sky-300 px-4 py-2">
                    <span class="w-3 h-3 border-2 border-black bg-red-400 rounded-full"></span>
                    <span class="w-3 h-3 border-2 border-black bg-yellow-300 rounded-full"></span>
                    <span class="w-3 h-3 border-2 border-black bg-green-400 rounded-full"></span>
                    <span class="ml-3 text-sm font-bold">welcome.exe</span>
                </div>
                <div class="px-6 py-16 text-center">
                    <h1 class="text-4xl md:text-6xl font-bold leading-tight">
                        Build and run your online store with
                        <span class="bg-yellow-300 px-2">webedit99</span>
                    </h1>
                    <p class="mt-6 text-lg max-w-2xl mx-auto">
                        Products, blogs, customer registration, a visual editor, code editing and a template engine.
                        Everything you need in one place.
                    </p>
                    <div class="mt-10 flex flex-wrap justify-center gap-4">
                        <NuxtLink to="/register"
                            class="border-2 border-black bg-pink-300 px-6 py-3 font-bold shadow-[4px_4px_0_0_#000] hover:translate-x-0.5 hover:translate-y-0.5 hover:shadow-[2px_2px_0_0_#000] transition-all">
                            Start for free &rarr;
                        </NuxtLink>
                        <NuxtLink to="/#features"
                            class="border-2 border-black bg-white px-6 py-3 font-bold shadow-[4px_4px_0_0_#000] hover:translate-x-0.5 hover:translate-y-0.5 hover:shadow-[2px_2px_0_0_#000] transition-all">
                            Learn more
                        </NuxtLink>
                    </div>
                </div>
            </div>
        </section>

        <section id="features" class="bg-sky-200 border-y-4 border-black">
            <div class="max-w-6xl mx-auto px-6 py-20">
                <h2 class="text-3xl font-bold text-center">
                    <span class="bg-white border-2 border-black px-3 py-1 shadow-[4px_4px_0_0_#000]">Everything in one
                        platform</span>
                </h2>
                <div class="mt-14 grid gap-8 md:grid-cols-2 lg:grid-cols-3">
                    <div v-for="feature in features" :key="feature.title"
                        class="border-4 border-black bg-white shadow-[6px_6px_0_0_#000]">
                        <div class="flex items-center justify-between border-b-4 border-black px-4 py-2"
                            :class="feature.color">
                            <h3 class="font-bold">{{ feature.title }}</h3>
                            <span class="text-xs font-bold">[ {{ feature.icon }} ]</span>
                        </div>
                        <p class="p-5 text-sm">{{ feature.description }}</p>
                    </div>
                </div>
            </div>
        </section>

        <section id="how" class="max-w-6xl mx-auto px-6 py-20">
            <h2 class="text-3xl font-bold text-center">
                <span class="bg-yellow-300 border-2 border-black px-3 py-1 shadow-[4px_4px_0_0_#000]">How it
                    works</span>
            </h2>
            <div class="mt-14 grid gap-8 md:grid-cols-3">
                <div v-for="(step, index) in steps" :key="step.title"
                    class="relative border-4 border-black bg-white p-6 pt-10 shadow-[6px_6px_0_0_#000]">
                    <div
                        class="absolute -top-5 left-4 w-10 h-10 border-4 border-black bg-pink-300 flex items-center justify-center text-xl font-bold">
                        {{ index + 1 }}
                    </div>
                    <h3 class="font-bold text-lg">{{ step.title }}</h3>
                    <p class="mt-2 text-sm">{{ step.description }}</p>
                </div>
            </div>
        </section>
    </div>
</template>

<script setup>
const features = [
    {
        title: 'Products',
        icon: 'SHOP',
        color: 'bg-yellow-300',
        description: 'Add, organize and manage your products with prices, images, variants and inventory.'
    },
    {
        title: 'Blogs',
        icon: 'POST',
        color: 'bg-pink-300',
        description: 'Publish posts and articles to grow your audience and keep customers engaged.'
    },
    {
        title: 'Customer Register',
        icon: 'USER',
        color: 'bg-green-300',
        description: 'Let customers sign up, log in and manage their accounts on your store.'
    },
    {
        title: 'Editor',
        icon: 'EDIT',
        color: 'bg-sky-300',
        description: 'Edit your pages and content visually without writing any code.'
    },
    {
        title: 'Simple Code Editing',
        icon: 'CODE',
        color: 'bg-orange-300',
        description: 'Open the code editor to adjust HTML, CSS and scripts whenever you need full control.'
    },
    {
        title: 'Template Engine',
        icon: 'TPL',
        color: 'bg-purple-300',
        description: 'Build reusable templates and layouts that render your store dynamically.'
    }
]

const steps = [
    {
        title: 'Create your account',
        description: 'Register and set up your store in a few clicks.'
    },
    {
        title: 'Add content',
        description: 'Add products, write blog posts and customize pages with the editor.'
    },
    {
        title: 'Launch',
        description: 'Publish your store and start accepting customers.'
    }
]

useHead({
    title: 'webedit99',
    meta: [
        {
            name: 'description',
            content: 'webedit99 - products, blogs, customer registration, editor, code editing and template engine.'
        }
    ]
})
</script>