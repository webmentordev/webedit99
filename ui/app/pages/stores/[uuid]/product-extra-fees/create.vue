<template>
    <div class="mx-auto flex max-w-xl flex-col gap-4">
        <div class="flex items-center justify-between">
            <h1 class="text-xl font-semibold text-gray-900">Create extra fee</h1>
            <NuxtLink :to="`/stores/${uuid}/product-extra-fees`"
                class="rounded-lg border border-gray-300 bg-white px-3 py-1.5 text-sm font-medium hover:bg-gray-50">
                Back
            </NuxtLink>
        </div>

        <form class="flex flex-col gap-4" @submit.prevent="submit">
            <div class="rounded-xl border border-gray-300 bg-white p-4 shadow-sm">
                <label for="product" class="mb-1 block text-sm font-medium">Product</label>
                <select id="product" v-model="form.product_id"
                    class="w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-400">
                    <option value="" disabled>Select a product</option>
                    <option v-for="p in products" :key="p.id" :value="p.id">{{ p.title }}</option>
                </select>
                <p v-if="!loadingProducts && products.length === 0" class="mt-1 text-xs text-gray-500">
                    No products yet.
                    <NuxtLink :to="`/stores/${uuid}/products/create`" class="underline">Create one first</NuxtLink>.
                </p>

                <label for="title" class="mb-1 mt-3 block text-sm font-medium">Title</label>
                <StoreInput id="title" v-model="form.title" type="text" placeholder="e.g. Gift wrap" />

                <label for="price" class="mb-1 mt-3 block text-sm font-medium">Price</label>
                <input id="price" v-model="form.price" type="number" min="0"
                    class="w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-400" />

                <label class="mt-4 flex items-center gap-2 text-sm">
                    <input v-model="form.is_active" type="checkbox" class="h-4 w-4" />
                    Active
                </label>
            </div>

            <CardsError v-if="error" :text="error" />

            <div class="flex justify-end gap-2">
                <NuxtLink :to="`/stores/${uuid}/product-extra-fees`"
                    class="rounded-lg border border-gray-300 bg-white px-4 py-2 text-sm font-medium hover:bg-gray-50">
                    Cancel
                </NuxtLink>
                <button type="submit"
                    :disabled="saving || !form.product_id || !form.title.trim() || toInt(form.price) === null"
                    class="rounded-lg bg-gray-900 px-4 py-2 text-sm font-medium text-white hover:bg-gray-800 disabled:opacity-50">
                    {{ saving ? 'Creating...' : 'Create extra fee' }}
                </button>
            </div>
        </form>
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

const form = reactive({
    product_id: typeof route.query.product_id === 'string' ? route.query.product_id : '',
    title: '',
    price: '',
    is_active: true
});

const products = ref([]);
const loadingProducts = ref(true);
const saving = ref(false);
const error = ref('');

function toInt(value) {
    if (value === '' || value === null || value === undefined) return null;
    const n = Number(value);
    return Number.isInteger(n) && n >= 0 ? n : null;
}

async function fetchProducts() {
    loadingProducts.value = true;
    try {
        const data = await authFetch(`/system/stores/${uuid.value}/products?per_page=100`, {
            method: 'GET'
        });
        products.value = data?.data || [];
    } catch (e) {
        error.value = e.statusMessage || 'Failed to load products.';
    } finally {
        loadingProducts.value = false;
    }
}

async function submit() {
    const price = toInt(form.price);
    if (!form.product_id || !form.title.trim() || price === null) return;

    saving.value = true;
    error.value = '';
    try {
        await authFetch(`/system/stores/${uuid.value}/product-extra-fees`, {
            method: 'POST',
            body: {
                product_id: form.product_id,
                title: form.title.trim(),
                price,
                is_active: form.is_active
            }
        });

        await navigateTo(`/stores/${uuid.value}/product-extra-fees`);
    } catch (e) {
        error.value = e.statusMessage || 'Failed to create extra fee.';
    } finally {
        saving.value = false;
    }
}

onMounted(fetchProducts);

useHead({
    title: 'Create extra fee - webedit99'
});
</script>