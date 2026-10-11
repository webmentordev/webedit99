<template>
    <div class="min-h-screen bg-yellow-50 font-mono text-black p-6">
        <div class="max-w-5xl mx-auto">
            <div class="flex items-center justify-between gap-4">
                <h1 class="text-2xl font-bold bg-black text-yellow-300 px-3 py-1 inline-block">currencies_</h1>
                <div class="flex items-center gap-2">
                    <button type="button" @click="fetchCurrencies" :disabled="loading"
                        class="border-2 border-black bg-white px-4 py-2 text-sm font-bold shadow-[3px_3px_0_0_#000] hover:translate-x-px hover:translate-y-px hover:shadow-[2px_2px_0_0_#000] transition-all disabled:opacity-50">
                        {{ loading ? 'Loading...' : 'Refresh' }}
                    </button>
                    <NuxtLink to="/dashboard/currencies/create"
                        class="border-2 border-black bg-yellow-300 px-4 py-2 text-sm font-bold shadow-[3px_3px_0_0_#000] hover:translate-x-px hover:translate-y-px hover:shadow-[2px_2px_0_0_#000] transition-all">
                        + Create currency
                    </NuxtLink>
                </div>
            </div>

            <div class="mt-6 border-4 border-black bg-white shadow-[6px_6px_0_0_#000]">
                <div class="border-b-4 border-black bg-yellow-100 p-3">
                    <input v-model="search" type="text" placeholder="Search currencies..."
                        class="w-full border-2 border-black bg-white px-3 py-2 text-sm font-bold placeholder:text-gray-500 focus:outline-none focus:bg-yellow-50" />
                </div>

                <div v-if="loading" class="p-10 text-center text-sm font-bold">Loading currencies...</div>

                <div v-else-if="error" class="p-6 text-center">
                    <p class="inline-block border-2 border-black bg-red-300 px-3 py-2 text-sm font-bold">{{ error }}</p>
                    <div>
                        <button type="button" @click="fetchCurrencies"
                            class="mt-4 border-2 border-black bg-white px-4 py-2 text-sm font-bold hover:bg-yellow-100">
                            Try again
                        </button>
                    </div>
                </div>

                <div v-else-if="currencies.length === 0" class="p-10 text-center">
                    <p class="font-bold">No currencies yet</p>
                    <NuxtLink to="/dashboard/currencies/create"
                        class="inline-block mt-4 border-2 border-black bg-yellow-300 px-4 py-2 text-sm font-bold shadow-[3px_3px_0_0_#000] hover:translate-x-px hover:translate-y-px hover:shadow-[2px_2px_0_0_#000] transition-all">
                        + Create currency
                    </NuxtLink>
                </div>

                <div v-else-if="filtered.length === 0" class="p-10 text-center">
                    <p class="font-bold">No results found</p>
                    <p class="mt-1 text-sm">Try a different search term.</p>
                </div>

                <div v-else class="overflow-x-auto">
                    <table class="w-full text-sm">
                        <thead>
                            <tr class="border-b-4 border-black bg-yellow-300 text-left">
                                <th class="px-4 py-2 font-bold">Code</th>
                                <th class="px-4 py-2 font-bold border-l-2 border-black">Symbol</th>
                                <th class="px-4 py-2 font-bold border-l-2 border-black text-right">Decimals</th>
                                <th class="px-4 py-2 font-bold border-l-2 border-black text-right">Value</th>
                                <th class="px-4 py-2 font-bold border-l-2 border-black">Updated</th>
                                <th class="px-4 py-2 font-bold border-l-2 border-black text-center">Actions</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr v-for="currency in filtered" :key="currency.id"
                                class="border-b-2 border-black last:border-b-0 hover:bg-yellow-100">
                                <td class="px-4 py-2">
                                    <span class="border-2 border-black bg-black px-2 py-0.5 font-bold text-yellow-300">
                                        {{ currency.currency_code }}
                                    </span>
                                </td>
                                <td class="px-4 py-2 border-l-2 border-black font-bold">{{ currency.symbol }}</td>
                                <td class="px-4 py-2 border-l-2 border-black text-right">{{ currency.decimal_places }}
                                </td>
                                <td class="px-4 py-2 border-l-2 border-black text-right font-bold">{{ currency.value }}
                                </td>
                                <td class="px-4 py-2 border-l-2 border-black whitespace-nowrap">
                                    {{ formatDate(currency.updated_at) }}
                                </td>
                                <td class="px-4 py-2 border-l-2 border-black text-center">
                                    <NuxtLink :to="`/dashboard/currencies/${currency.id}`"
                                        class="inline-block border-2 border-black bg-white px-3 py-1 text-xs font-bold shadow-[2px_2px_0_0_#000] hover:bg-yellow-300 hover:translate-x-px hover:translate-y-px hover:shadow-[1px_1px_0_0_#000] transition-all">
                                        Edit
                                    </NuxtLink>
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>

                <div v-if="!loading && !error && currencies.length > 0"
                    class="border-t-4 border-black bg-yellow-50 px-4 py-2 text-xs font-bold">
                    Showing {{ filtered.length }} of {{ currencies.length }} currencies
                </div>
            </div>
        </div>
    </div>
</template>

<script setup>
definePageMeta({
    middleware: 'admin',
    layout: 'admin'
});

const { authFetch } = useAuthFetch();

const currencies = ref([]);
const search = ref('');
const loading = ref(true);
const error = ref('');

const filtered = computed(() => {
    const q = search.value.trim().toLowerCase();
    if (!q) return currencies.value;
    return currencies.value.filter((currency) =>
        [currency.currency_code, currency.symbol, currency.value]
            .filter(Boolean)
            .some((value) => String(value).toLowerCase().includes(q))
    );
});

function formatDate(value) {
    if (!value) return '-';
    return new Date(value).toLocaleString();
}

async function fetchCurrencies() {
    loading.value = true;
    error.value = '';
    try {
        const data = await authFetch('/admin/currencies', {
            method: 'GET'
        });
        if (Array.isArray(data)) {
            currencies.value = data;
        } else {
            currencies.value = data?.currencies || data?.data || [];
        }
    } catch (e) {
        error.value = e.statusMessage || 'Something went wrong!';
    } finally {
        loading.value = false;
    }
}

onMounted(fetchCurrencies);

useHead({
    title: 'Currencies - webedit99'
});
</script>