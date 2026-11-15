<template>
    <div class="mx-auto flex max-w-xl flex-col gap-4">
        <div class="flex items-center justify-between">
            <h1 class="text-xl font-semibold text-gray-900">Create sub variant</h1>
            <NuxtLink :to="`/stores/${uuid}/product-sub-variants`"
                class="rounded-lg border border-gray-300 bg-white px-3 py-1.5 text-sm font-medium hover:bg-gray-50">
                Back
            </NuxtLink>
        </div>

        <form class="flex flex-col gap-4" @submit.prevent="submit">
            <div class="rounded-xl border border-gray-300 bg-white p-4 shadow-sm">
                <label for="variant" class="mb-1 block text-sm font-medium">Variant</label>
                <select id="variant" v-model="form.product_variant_id"
                    class="w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-400">
                    <option value="" disabled>Select a variant</option>
                    <option v-for="v in variants" :key="v.id" :value="v.id">{{ v.product_title }} - {{ v.title }}
                    </option>
                </select>
                <p v-if="!loadingVariants && variants.length === 0" class="mt-1 text-xs text-gray-500">
                    No variants yet.
                    <NuxtLink :to="`/stores/${uuid}/product-variants`" class="underline">Create one first</NuxtLink>.
                </p>

                <label for="title" class="mb-1 mt-3 block text-sm font-medium">Title</label>
                <StoreInput id="title" v-model="form.title" type="text" placeholder="e.g. Large" />

                <label for="sku" class="mb-1 mt-3 block text-sm font-medium">SKU</label>
                <StoreInput id="sku" v-model="form.sku" type="text" placeholder="Generated if empty" />

                <div class="mt-3 grid grid-cols-1 gap-3 sm:grid-cols-3">
                    <div>
                        <label for="price" class="mb-1 block text-sm font-medium">Price</label>
                        <input id="price" v-model="form.price" type="number" min="0"
                            class="w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-400" />
                    </div>
                    <div>
                        <label for="compare_at_price" class="mb-1 block text-sm font-medium">Compare at</label>
                        <input id="compare_at_price" v-model="form.compare_at_price" type="number" min="0"
                            class="w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-400" />
                    </div>
                    <div>
                        <label for="cost_price" class="mb-1 block text-sm font-medium">Cost</label>
                        <input id="cost_price" v-model="form.cost_price" type="number" min="0"
                            class="w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-400" />
                    </div>
                </div>

                <div class="mt-3 grid grid-cols-1 gap-3 sm:grid-cols-3">
                    <div>
                        <label for="quantity" class="mb-1 block text-sm font-medium">Quantity</label>
                        <input id="quantity" v-model="form.quantity" type="number" min="0"
                            class="w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-400" />
                    </div>
                    <div>
                        <label for="low_stock_threshold" class="mb-1 block text-sm font-medium">Low stock at</label>
                        <input id="low_stock_threshold" v-model="form.low_stock_threshold" type="number" min="0"
                            class="w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-400" />
                    </div>
                    <div>
                        <label for="sort_order" class="mb-1 block text-sm font-medium">Sort order</label>
                        <input id="sort_order" v-model.number="form.sort_order" type="number"
                            class="w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-400" />
                    </div>
                </div>

                <label class="mt-4 flex items-center gap-2 text-sm">
                    <input v-model="form.is_active" type="checkbox" class="h-4 w-4" />
                    Active
                </label>
            </div>

            <CardsError v-if="error" :text="error" />

            <div class="flex justify-end gap-2">
                <NuxtLink :to="`/stores/${uuid}/product-sub-variants`"
                    class="rounded-lg border border-gray-300 bg-white px-4 py-2 text-sm font-medium hover:bg-gray-50">
                    Cancel
                </NuxtLink>
                <button type="submit"
                    :disabled="saving || !form.product_variant_id || !form.title.trim() || toInt(form.price) === null"
                    class="rounded-lg bg-gray-900 px-4 py-2 text-sm font-medium text-white hover:bg-gray-800 disabled:opacity-50">
                    {{ saving ? 'Creating...' : 'Create sub variant' }}
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
    product_variant_id: typeof route.query.product_variant_id === 'string' ? route.query.product_variant_id : '',
    title: '',
    sku: '',
    price: '',
    compare_at_price: '',
    cost_price: '',
    quantity: '',
    low_stock_threshold: '',
    sort_order: 1,
    is_active: true
});

const variants = ref([]);
const loadingVariants = ref(true);
const saving = ref(false);
const error = ref('');

function toInt(value) {
    if (value === '' || value === null || value === undefined) return null;
    const n = Number(value);
    return Number.isInteger(n) && n >= 0 ? n : null;
}

async function fetchVariants() {
    loadingVariants.value = true;
    try {
        const data = await authFetch(`/system/stores/${uuid.value}/product-sub-variants/options`, {
            method: 'GET'
        });
        variants.value = Array.isArray(data) ? data : [];
    } catch (e) {
        error.value = e.statusMessage || 'Failed to load variants.';
    } finally {
        loadingVariants.value = false;
    }
}

async function submit() {
    const price = toInt(form.price);
    if (!form.product_variant_id || !form.title.trim() || price === null) return;

    saving.value = true;
    error.value = '';
    try {
        await authFetch(`/system/stores/${uuid.value}/product-sub-variants`, {
            method: 'POST',
            body: {
                product_variant_id: form.product_variant_id,
                title: form.title.trim(),
                sku: form.sku.trim() || null,
                price,
                compare_at_price: toInt(form.compare_at_price),
                cost_price: toInt(form.cost_price),
                quantity: toInt(form.quantity),
                low_stock_threshold: toInt(form.low_stock_threshold),
                sort_order: Number.isInteger(form.sort_order) ? form.sort_order : 1,
                is_active: form.is_active
            }
        });

        await navigateTo(`/stores/${uuid.value}/product-sub-variants`);
    } catch (e) {
        error.value = e.statusMessage || 'Failed to create sub variant.';
    } finally {
        saving.value = false;
    }
}

onMounted(fetchVariants);

useHead({
    title: 'Create sub variant - webedit99'
});
</script>