<template>
    <div class="flex flex-col gap-4">
        <div class="flex items-center justify-between">
            <h1 class="text-xl font-semibold text-gray-900">Sub variants</h1>
            <NuxtLink :to="`/stores/${uuid}/product-sub-variants/create`"
                class="rounded-lg bg-gray-900 px-4 py-2 text-sm font-medium text-white hover:bg-gray-800">
                Create sub variant
            </NuxtLink>
        </div>

        <div class="flex flex-wrap items-center gap-2">
            <div class="w-64">
                <StoreInput id="search" v-model="search" type="text" placeholder="Search title or SKU" />
            </div>
            <select v-model="variantId"
                class="rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-400">
                <option value="">All variants</option>
                <option v-for="v in variants" :key="v.id" :value="v.id">{{ v.product_title }} - {{ v.title }}</option>
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
            <button type="button" @click="fetchItems"
                class="mt-3 rounded-lg border border-gray-300 bg-white px-3 py-1.5 text-sm font-medium hover:bg-gray-50">
                Try again
            </button>
        </div>

        <div class="overflow-x-auto rounded-xl border border-gray-300 bg-white shadow-sm">
            <table class="w-full text-left text-sm">
                <thead class="border-b border-gray-300 bg-gray-100 text-xs uppercase tracking-wide text-gray-500">
                    <tr>
                        <th class="px-4 py-2 font-semibold">Title</th>
                        <th class="px-4 py-2 font-semibold">Variant</th>
                        <th class="px-4 py-2 font-semibold">SKU</th>
                        <th class="px-4 py-2 font-semibold">Price</th>
                        <th class="px-4 py-2 font-semibold">Qty</th>
                        <th class="px-4 py-2 font-semibold">Status</th>
                        <th class="px-4 py-2 font-semibold">Created</th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-if="loading">
                        <td colspan="7" class="px-4 py-10 text-center text-gray-500">Loading sub variants...</td>
                    </tr>
                    <tr v-else-if="items.length === 0">
                        <td colspan="7" class="px-4 py-10 text-center text-gray-500">No sub variants found.</td>
                    </tr>
                    <template v-else>
                        <tr v-for="s in items" :key="s.id"
                            class="border-b border-gray-200 last:border-0 hover:bg-gray-50">
                            <td class="px-4 py-2 font-medium text-gray-900">{{ s.title }}</td>
                            <td class="px-4 py-2 text-gray-600">{{ s.variant_title }}</td>
                            <td class="px-4 py-2 text-gray-600">{{ s.sku }}</td>
                            <td class="px-4 py-2 text-gray-600">{{ s.price }}</td>
                            <td class="px-4 py-2">
                                <span v-if="s.quantity === null" class="text-gray-400">-</span>
                                <span v-else :class="isLow(s) ? 'font-medium text-red-700' : 'text-gray-600'">
                                    {{ s.quantity }}
                                </span>
                            </td>
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
            <span>{{ total }} sub variant{{ total === 1 ? '' : 's' }}</span>
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
const variants = ref([]);
const total = ref(0);
const loading = ref(true);
const error = ref('');

const page = ref(1);
const perPage = 20;
const search = ref('');
const status = ref('');
const variantId = ref('');

const totalPages = computed(() => Math.max(1, Math.ceil(total.value / perPage)));

function isLow(s) {
    return s.low_stock_threshold !== null && s.quantity !== null && s.quantity <= s.low_stock_threshold;
}

async function fetchVariants() {
    try {
        const data = await authFetch(`/system/stores/${uuid.value}/product-sub-variants/options`, {
            method: 'GET'
        });
        variants.value = Array.isArray(data) ? data : [];
    } catch (e) {
        variants.value = [];
    }
}

async function fetchItems() {
    loading.value = true;
    error.value = '';
    try {
        const params = new URLSearchParams({
            page: String(page.value),
            per_page: String(perPage)
        });
        if (search.value.trim()) params.set('search', search.value.trim());
        if (status.value) params.set('is_active', status.value);
        if (variantId.value) params.set('product_variant_id', variantId.value);

        const data = await authFetch(`/system/stores/${uuid.value}/product-sub-variants?${params}`, {
            method: 'GET'
        });

        items.value = data?.data || [];
        total.value = data?.total || 0;
    } catch (e) {
        error.value = e.statusMessage || 'Failed to load sub variants.';
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
        if (page.value === 1) fetchItems();
        else page.value = 1;
    }, 300);
});

watch([status, variantId], () => {
    if (page.value === 1) fetchItems();
    else page.value = 1;
});

watch(page, fetchItems);

onMounted(() => {
    fetchVariants();
    fetchItems();
});
onBeforeUnmount(() => clearTimeout(timer));

useHead({
    title: 'Sub variants - webedit99'
});
</script>