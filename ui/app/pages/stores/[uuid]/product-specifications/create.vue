<template>
    <div class="mx-auto flex max-w-xl flex-col gap-4">
        <div class="flex items-center justify-between">
            <h1 class="text-xl font-semibold text-gray-900">Create specification</h1>
            <NuxtLink :to="`/stores/${uuid}/product-specifications`"
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
                <StoreInput id="title" v-model="form.title" type="text" placeholder="e.g. Material" />

                <label for="value" class="mb-1 mt-3 block text-sm font-medium">Value</label>
                <StoreInput id="value" v-model="form.value" type="text" placeholder="e.g. Cotton" />

                <label for="description" class="mb-1 mt-3 block text-sm font-medium">Description</label>
                <textarea id="description" v-model="form.description" rows="3"
                    class="w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-400"></textarea>

                <label for="sort_order" class="mb-1 mt-3 block text-sm font-medium">Sort order</label>
                <input id="sort_order" v-model.number="form.sort_order" type="number"
                    class="w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-400" />

                <label class="mt-4 flex items-center gap-2 text-sm">
                    <input v-model="form.is_active" type="checkbox" class="h-4 w-4" />
                    Active
                </label>
            </div>

            <CardsError v-if="error" :text="error" />

            <div class="flex justify-end gap-2">
                <NuxtLink :to="`/stores/${uuid}/product-specifications`"
                    class="rounded-lg border border-gray-300 bg-white px-4 py-2 text-sm font-medium hover:bg-gray-50">
                    Cancel
                </NuxtLink>
                <button type="submit" :disabled="saving || !form.product_id || !form.title.trim() || !form.value.trim()"
                    class="rounded-lg bg-gray-900 px-4 py-2 text-sm font-medium text-white hover:bg-gray-800 disabled:opacity-50">
                    {{ saving ? 'Creating...' : 'Create specification' }}
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
    value: '',
    description: '',
    sort_order: 1,
    is_active: true
});

const products = ref([]);
const loadingProducts = ref(true);
const saving = ref(false);
const error = ref('');

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
    if (!form.product_id || !form.title.trim() || !form.value.trim()) return;

    saving.value = true;
    error.value = '';
    try {
        await authFetch(`/system/stores/${uuid.value}/product-specifications`, {
            method: 'POST',
            body: {
                product_id: form.product_id,
                title: form.title.trim(),
                value: form.value.trim(),
                description: form.description.trim() || null,
                sort_order: Number.isInteger(form.sort_order) ? form.sort_order : 1,
                is_active: form.is_active
            }
        });

        await navigateTo(`/stores/${uuid.value}/product-specifications`);
    } catch (e) {
        error.value = e.statusMessage || 'Failed to create specification.';
    } finally {
        saving.value = false;
    }
}

onMounted(fetchProducts);

useHead({
    title: 'Create specification - webedit99'
});
</script>