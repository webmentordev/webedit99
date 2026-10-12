<template>
    <div class="min-h-screen bg-yellow-50 font-mono text-black p-6">
        <div class="max-w-6xl mx-auto">
            <div class="flex items-center justify-between gap-4">
                <h1 class="text-2xl font-bold bg-black text-yellow-300 px-3 py-1 inline-block">stores_</h1>
                <button type="button" @click="fetchStores" :disabled="loading"
                    class="border-2 border-black bg-white px-4 py-2 text-sm font-bold shadow-[3px_3px_0_0_#000] hover:translate-x-px hover:translate-y-px hover:shadow-[2px_2px_0_0_#000] transition-all disabled:opacity-50">
                    {{ loading ? 'Loading...' : 'Refresh' }}
                </button>
            </div>

            <div class="mt-6 border-4 border-black bg-white shadow-[6px_6px_0_0_#000]">
                <div class="border-b-4 border-black bg-yellow-100 p-3">
                    <input v-model="search" type="text" placeholder="Search stores..."
                        class="w-full border-2 border-black bg-white px-3 py-2 text-sm font-bold placeholder:text-gray-500 focus:outline-none focus:bg-yellow-50" />
                </div>

                <div v-if="loading" class="p-10 text-center text-sm font-bold">Loading stores...</div>

                <div v-else-if="error" class="p-6 text-center">
                    <p class="inline-block border-2 border-black bg-red-300 px-3 py-2 text-sm font-bold">{{ error }}</p>
                    <div>
                        <button type="button" @click="fetchStores"
                            class="mt-4 border-2 border-black bg-white px-4 py-2 text-sm font-bold hover:bg-yellow-100">
                            Try again
                        </button>
                    </div>
                </div>

                <div v-else-if="stores.length === 0" class="p-10 text-center">
                    <p class="font-bold">No stores yet</p>
                </div>

                <div v-else-if="filtered.length === 0" class="p-10 text-center">
                    <p class="font-bold">No results found</p>
                    <p class="mt-1 text-sm">Try a different search term.</p>
                </div>

                <div v-else class="overflow-x-auto">
                    <table class="w-full text-sm">
                        <thead>
                            <tr class="border-b-4 border-black bg-yellow-300 text-left">
                                <th class="px-4 py-2 font-bold">Store</th>
                                <th class="px-4 py-2 font-bold border-l-2 border-black">Domain</th>
                                <th class="px-4 py-2 font-bold border-l-2 border-black">Status</th>
                                <th class="px-4 py-2 font-bold border-l-2 border-black">Contact</th>
                                <th class="px-4 py-2 font-bold border-l-2 border-black">Locale</th>
                                <th class="px-4 py-2 font-bold border-l-2 border-black text-right">Members</th>
                                <th class="px-4 py-2 font-bold border-l-2 border-black text-right">Customers</th>
                                <th class="px-4 py-2 font-bold border-l-2 border-black">Created</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr v-for="store in filtered" :key="store.id"
                                class="border-b-2 border-black last:border-b-0 hover:bg-yellow-100">
                                <td class="px-4 py-2">
                                    <p class="font-bold">{{ store.title }}</p>
                                    <span
                                        class="mt-1 inline-block border-2 border-black bg-black px-2 py-0.5 text-xs font-bold text-yellow-300">
                                        {{ store.slug }}
                                    </span>
                                </td>
                                <td class="px-4 py-2 border-l-2 border-black">
                                    <template v-if="store.custom_domain">
                                        <p class="font-bold">{{ store.custom_domain }}</p>
                                        <span
                                            class="mt-1 inline-block border-2 border-black px-2 py-0.5 text-xs font-bold"
                                            :class="store.domain_verified_at ? 'bg-green-300' : 'bg-orange-300'">
                                            {{ store.domain_verified_at ? 'Verified' : 'Unverified' }}
                                        </span>
                                    </template>
                                    <span v-else>-</span>
                                </td>
                                <td class="px-4 py-2 border-l-2 border-black">
                                    <div class="flex flex-wrap gap-1">
                                        <span class="border-2 border-black px-2 py-0.5 text-xs font-bold"
                                            :class="store.is_active ? 'bg-green-300' : 'bg-gray-300'">
                                            {{ store.is_active ? 'Active' : 'Inactive' }}
                                        </span>
                                        <span v-if="store.is_banned"
                                            class="border-2 border-black bg-red-300 px-2 py-0.5 text-xs font-bold">
                                            Banned
                                        </span>
                                    </div>
                                </td>
                                <td class="px-4 py-2 border-l-2 border-black">
                                    <p>{{ store.contact_email || '-' }}</p>
                                    <p v-if="store.contact_phone_number" class="text-xs">{{ store.contact_phone_number
                                        }}</p>
                                    <p v-if="store.whatsapp" class="text-xs">WA: {{ store.whatsapp }}</p>
                                </td>
                                <td class="px-4 py-2 border-l-2 border-black whitespace-nowrap">
                                    <p>{{ store.default_locale }}</p>
                                    <p class="text-xs">{{ store.timezone }}</p>
                                </td>
                                <td class="px-4 py-2 border-l-2 border-black text-right font-bold">{{
                                    store.members_count }}</td>
                                <td class="px-4 py-2 border-l-2 border-black text-right font-bold">{{
                                    store.customers_count }}</td>
                                <td class="px-4 py-2 border-l-2 border-black whitespace-nowrap">
                                    {{ formatDate(store.created_at) }}
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>

                <div v-if="!loading && !error && stores.length > 0"
                    class="border-t-4 border-black bg-yellow-50 px-4 py-2 text-xs font-bold">
                    Showing {{ filtered.length }} of {{ stores.length }} stores
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

const stores = ref([]);
const search = ref('');
const loading = ref(true);
const error = ref('');

const filtered = computed(() => {
    const q = search.value.trim().toLowerCase();
    if (!q) return stores.value;
    return stores.value.filter((store) =>
        [store.title, store.slug, store.custom_domain, store.contact_email, store.contact_phone_number, store.whatsapp]
            .filter(Boolean)
            .some((value) => String(value).toLowerCase().includes(q))
    );
});

function formatDate(value) {
    if (!value) return '-';
    return new Date(value).toLocaleString();
}

async function fetchStores() {
    loading.value = true;
    error.value = '';
    try {
        const data = await authFetch('/admin/stores', {
            method: 'GET'
        });
        if (Array.isArray(data)) {
            stores.value = data;
        } else {
            stores.value = data?.stores || data?.data || [];
        }
    } catch (e) {
        error.value = e.statusMessage || 'Something went wrong!';
    } finally {
        loading.value = false;
    }
}

onMounted(fetchStores);

useHead({
    title: 'Stores - webedit99'
});
</script>