<template>
    <div class="flex flex-col gap-4">
        <div class="flex items-center justify-between">
            <h1 class="text-xl font-semibold text-gray-900">Sub collections</h1>
            <NuxtLink :to="`/stores/${uuid}/sub-collections/create`"
                class="rounded-lg bg-gray-900 px-4 py-2 text-sm font-medium text-white hover:bg-gray-800">
                Create sub collection
            </NuxtLink>
        </div>

        <div class="flex flex-wrap items-center gap-2">
            <div class="w-64">
                <StoreInput id="search" v-model="search" type="text" placeholder="Search sub collections" />
            </div>
            <select v-model="collectionId"
                class="rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-400">
                <option value="">All collections</option>
                <option v-for="c in collections" :key="c.id" :value="c.id">{{ c.title }}</option>
            </select>
            <select v-model="status"
                class="rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-400">
                <option value="">All statuses</option>
                <option value="true">Active</option>
                <option value="false">Inactive</option>
            </select>
        </div>

        <div v-if="error">
            <CardsError :text="error" />
            <button type="button" @click="fetchSubCollections"
                class="mt-3 rounded-lg border border-gray-300 bg-white px-3 py-1.5 text-sm font-medium hover:bg-gray-50">
                Try again
            </button>
        </div>

        <div class="overflow-x-auto rounded-xl border border-gray-300 bg-white shadow-sm">
            <table class="w-full text-left text-sm">
                <thead class="border-b border-gray-300 bg-gray-100 text-xs uppercase tracking-wide text-gray-500">
                    <tr>
                        <th class="px-4 py-2 font-semibold">Title</th>
                        <th class="px-4 py-2 font-semibold">Collection</th>
                        <th class="px-4 py-2 font-semibold">Slug</th>
                        <th class="px-4 py-2 font-semibold">Order</th>
                        <th class="px-4 py-2 font-semibold">Status</th>
                        <th class="px-4 py-2 font-semibold">Created</th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-if="loading">
                        <td colspan="6" class="px-4 py-10 text-center text-gray-500">Loading sub collections...</td>
                    </tr>
                    <tr v-else-if="items.length === 0">
                        <td colspan="6" class="px-4 py-10 text-center text-gray-500">No sub collections found.</td>
                    </tr>
                    <template v-else>
                        <tr v-for="s in items" :key="s.id"
                            class="border-b border-gray-200 last:border-0 hover:bg-gray-50">
                            <td class="px-4 py-2 font-medium text-gray-900">
                                {{ s.title }}
                                <span v-if="s.is_featured"
                                    class="ml-2 rounded-full bg-yellow-100 px-2 py-0.5 text-xs font-medium text-yellow-800">
                                    Featured
                                </span>
                            </td>
                            <td class="px-4 py-2 text-gray-600">{{ s.collection_title }}</td>
                            <td class="px-4 py-2 text-gray-600">{{ s.slug }}</td>
                            <td class="px-4 py-2 text-gray-600">{{ s.sort_order }}</td>
                            <td class="px-4 py-2">
                                <span class="rounded-full px-2 py-0.5 text-xs font-medium"
                                    :class="s.is_active ? 'bg-green-100 text-green-700' : 'bg-red-100 text-red-700'">
                                    {{ s.is_active ? 'Active' : 'Inactive' }}
                                </span>
                            </td>
                            <td class="px-4 py-2 text-gray-600">{{ formatDate(s.created_at) }}</td>
                        </tr>
                    </template>
                </tbody>
            </table>
        </div>

        <div class="flex items-center justify-between text-sm text-gray-600">
            <span>{{ total }} sub collection{{ total === 1 ? '' : 's' }}</span>
            <div class="flex items-center gap-2">
                <button type="button" :disabled="page <= 1" @click="page--"
                    class="rounded-lg border border-gray-300 bg-white px-3 py-1.5 hover:bg-gray-50 disabled:opacity-50">
                    Previous
                </button>
                <span>Page {{ page }} of {{ totalPages }}</span>
                <button type="button" :disabled="page >= totalPages" @click="page++"
                    class="rounded-lg border border-gray-300 bg-white px-3 py-1.5 hover:bg-gray-50 disabled:opacity-50">
                    Next
                </button>
            </div>
        </div>
    </div>
</template>

<script setup>
definePageMeta({
    middleware: 'auth',
    layout: 'store'
});

const route = useRoute();
const { authFetch } = useAuthFetch();

const uuid = computed(() => route.params.uuid);

const items = ref([]);
const collections = ref([]);
const total = ref(0);
const loading = ref(true);
const error = ref('');

const page = ref(1);
const perPage = 20;
const search = ref('');
const status = ref('');
const collectionId = ref('');

const totalPages = computed(() => Math.max(1, Math.ceil(total.value / perPage)));

async function fetchCollections() {
    try {
        const data = await authFetch(`/system/stores/${uuid.value}/collections?per_page=100`, {
            method: 'GET'
        });
        collections.value = data?.data || [];
    } catch (e) {
        collections.value = [];
    }
}

async function fetchSubCollections() {
    loading.value = true;
    error.value = '';
    try {
        const params = new URLSearchParams({
            page: String(page.value),
            per_page: String(perPage)
        });
        if (search.value.trim()) params.set('search', search.value.trim());
        if (status.value) params.set('is_active', status.value);
        if (collectionId.value) params.set('collection_id', collectionId.value);

        const data = await authFetch(`/system/stores/${uuid.value}/sub-collections?${params}`, {
            method: 'GET'
        });

        items.value = data?.data || [];
        total.value = data?.total || 0;
    } catch (e) {
        error.value = e.statusMessage || 'Failed to load sub collections.';
    } finally {
        loading.value = false;
    }
}

function formatDate(value) {
    return new Date(value).toLocaleDateString();
}

let timer;
watch(search, () => {
    clearTimeout(timer);
    timer = setTimeout(() => {
        if (page.value === 1) fetchSubCollections();
        else page.value = 1;
    }, 300);
});

watch([status, collectionId], () => {
    if (page.value === 1) fetchSubCollections();
    else page.value = 1;
});

watch(page, fetchSubCollections);

onMounted(() => {
    fetchCollections();
    fetchSubCollections();
});
onBeforeUnmount(() => clearTimeout(timer));

useHead({
    title: 'Sub collections - webedit99'
});
</script>