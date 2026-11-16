<template>
    <div class="mx-auto flex max-w-xl flex-col gap-4">
        <div class="flex items-center justify-between">
            <h1 class="text-xl font-semibold text-gray-900">Create variant</h1>
            <NuxtLink :to="`/stores/${uuid}/product-variants`"
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
                <StoreInput id="title" v-model="form.title" type="text" placeholder="e.g. Red" />

                <label for="sort_order" class="mb-1 mt-3 block text-sm font-medium">Sort order</label>
                <input id="sort_order" v-model.number="form.sort_order" type="number"
                    class="w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-400" />

                <div class="mt-4 flex flex-col gap-2 text-sm">
                    <label class="flex items-center gap-2">
                        <input v-model="form.is_active" type="checkbox" class="h-4 w-4" />
                        Active
                    </label>
                    <label class="flex items-center gap-2">
                        <input v-model="form.is_default" type="checkbox" class="h-4 w-4" />
                        Default variant for this product
                    </label>
                </div>
            </div>

            <CardsError v-if="error" :text="error" />

            <div class="flex justify-end gap-2">
                <NuxtLink :to="`/stores/${uuid}/product-variants`"
                    class="rounded-lg border border-gray-300 bg-white px-4 py-2 text-sm font-medium hover:bg-gray-50">
                    Cancel
                </NuxtLink>
                <button type="submit" :disabled="saving || !form.product_id || !form.title.trim()"
                    class="rounded-lg bg-gray-900 px-4 py-2 text-sm font-medium text-white hover:bg-gray-800 disabled:opacity-50">
                    {{ saving ? 'Creating...' : 'Create variant' }}
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
    sort_order: 1,
    is_active: true,
    is_default: false
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
    if (!form.product_id || !form.title.trim()) return;

    saving.value = true;
    error.value = '';
    try {
        await authFetch(`/system/stores/${uuid.value}/product-variants`, {
            method: 'POST',
            body: {
                product_id: form.product_id,
                title: form.title.trim(),
                sort_order: Number.isInteger(form.sort_order) ? form.sort_order : 1,
                is_active: form.is_active,
                is_default: form.is_default
            }
        });

        await navigateTo(`/stores/${uuid.value}/product-variants`);
    } catch (e) {
        error.value = e.statusMessage || 'Failed to create variant.';
    } finally {
        saving.value = false;
    }
}

onMounted(fetchProducts);

useHead({
    title: 'Create variant - webedit99'
});
</script>