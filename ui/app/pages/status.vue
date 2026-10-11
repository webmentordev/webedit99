<template>
    <div class="w-full min-h-[80vh] bg-yellow-50 font-mono text-black px-4 py-12">
        <div class="max-w-2xl mx-auto">
            <div class="border-4 border-black bg-white shadow-[8px_8px_0_0_#000]">
                <div class="flex items-center gap-2 border-b-4 border-black bg-sky-300 px-4 py-2">
                    <span class="w-3 h-3 border-2 border-black bg-red-400 rounded-full"></span>
                    <span class="w-3 h-3 border-2 border-black bg-yellow-300 rounded-full"></span>
                    <span class="w-3 h-3 border-2 border-black bg-green-400 rounded-full"></span>
                    <span class="ml-3 text-sm font-bold">status.exe</span>
                </div>

                <div class="p-6">
                    <h1 class="text-2xl md:text-3xl font-bold">System Status</h1>

                    <div class="mt-6 border-4 border-black shadow-[6px_6px_0_0_#000]">
                        <div class="flex items-center justify-between border-b-4 border-black px-4 py-2"
                            :class="config.color">
                            <h2 class="font-bold">{{ config.title }}</h2>
                            <span class="text-xs font-bold">[ {{ config.tag }} ]</span>
                        </div>
                        <p class="p-5 text-sm">{{ config.text }}</p>
                    </div>

                    <div class="mt-8">
                        <h3 class="font-bold">Services</h3>
                        <div class="mt-3 border-2 border-black divide-y-2 divide-black">
                            <div class="flex items-center justify-between px-4 py-3 text-sm">
                                <span class="font-bold">API</span>
                                <span class="border-2 border-black px-2 py-0.5 text-xs font-bold" :class="config.color">
                                    {{ config.tag }}
                                </span>
                            </div>
                            <div v-for="item in details" :key="item.key"
                                class="flex items-center justify-between px-4 py-3 text-sm">
                                <span class="font-bold">{{ item.key }}</span>
                                <span class="border-2 border-black bg-yellow-100 px-2 py-0.5 text-xs font-bold">
                                    {{ item.value }}
                                </span>
                            </div>
                        </div>
                    </div>

                    <div class="mt-8 flex flex-wrap items-center justify-between gap-4 text-sm">
                        <p>
                            Last checked:
                            <span class="font-bold">{{ lastChecked || '--' }}</span>
                        </p>
                        <button type="button" :disabled="loading" @click="check"
                            class="border-2 border-black bg-pink-300 px-5 py-2 font-bold shadow-[4px_4px_0_0_#000] hover:translate-x-[2px] hover:translate-y-[2px] hover:shadow-[2px_2px_0_0_#000] transition-all disabled:opacity-60 disabled:cursor-not-allowed">
                            {{ loading ? 'Checking...' : 'Refresh' }}
                        </button>
                    </div>

                    <p class="mt-4 text-xs">Auto refreshes every 30 seconds.</p>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup>

const state = ref('checking');
const loading = ref(false);
const lastChecked = ref('');
const response = ref(null);
let timer = null;

const { publicFetch } = usePublicFetch();

const configs = {
    checking: {
        title: 'Checking status...',
        tag: 'WAIT',
        color: 'bg-yellow-300',
        text: 'Contacting the server. Please wait.'
    },
    up: {
        title: 'All systems operational',
        tag: 'OK',
        color: 'bg-green-300',
        text: 'Everything is running normally.'
    },
    down: {
        title: 'System is down',
        tag: 'DOWN',
        color: 'bg-red-400',
        text: 'We are unable to reach the system right now. Please try again shortly.'
    }
};

const config = computed(() => configs[state.value]);

const details = computed(() => {
    if (!response.value || typeof response.value !== 'object') return [];
    return Object.entries(response.value).map(([key, value]) => ({
        key,
        value: typeof value === 'object' ? JSON.stringify(value) : String(value)
    }));
});

async function check() {
    loading.value = true;
    try {
        const data = await publicFetch('/health');
        response.value = data.message;
        state.value = 'up';
    } catch (e) {
        response.value = null;
        state.value = 'down';
    } finally {
        lastChecked.value = new Date().toLocaleString();
        loading.value = false;
    }
}

onMounted(() => {
    check();
    timer = setInterval(check, 30000);
});

onBeforeUnmount(() => {
    clearInterval(timer);
});

useHead({
    title: 'Status - webedit99'
});
</script>