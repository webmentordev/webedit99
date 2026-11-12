<template>
    <div class="mx-auto flex max-w-3xl flex-col gap-4">
        <div class="flex items-center justify-between">
            <h1 class="text-xl font-semibold text-gray-900">Create product</h1>
            <NuxtLink :to="`/stores/${uuid}/products`"
                class="rounded-lg border border-gray-300 bg-white px-3 py-1.5 text-sm font-medium hover:bg-gray-50">
                Back
            </NuxtLink>
        </div>

        <form class="flex flex-col gap-4" @submit.prevent="submit">
            <div class="rounded-xl border border-gray-300 bg-white p-4 shadow-sm">
                <h2 class="mb-3 text-sm font-semibold text-gray-900">General</h2>

                <label for="title" class="mb-1 block text-sm font-medium">Title</label>
                <StoreInput id="title" v-model="form.title" type="text" placeholder="Product title" />

                <label for="slug" class="mb-1 mt-3 block text-sm font-medium">Slug</label>
                <StoreInput id="slug" v-model="form.slug" type="text" placeholder="Generated from title if empty" />

                <label for="description" class="mb-1 mt-3 block text-sm font-medium">Description</label>
                <textarea id="description" v-model="form.description" rows="3"
                    class="w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-400"></textarea>

                <label for="content" class="mb-1 mt-3 block text-sm font-medium">Content</label>
                <textarea id="content" v-model="form.content" rows="8"
                    class="w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-400"></textarea>
            </div>

            <div class="rounded-xl border border-gray-300 bg-white p-4 shadow-sm">
                <h2 class="mb-3 text-sm font-semibold text-gray-900">Settings</h2>

                <div class="grid grid-cols-1 gap-3 sm:grid-cols-3">
                    <div>
                        <label for="sort_order" class="mb-1 block text-sm font-medium">Sort order</label>
                        <input id="sort_order" v-model.number="form.sort_order" type="number"
                            class="w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-400" />
                    </div>
                    <div>
                        <label for="min_order_qty" class="mb-1 block text-sm font-medium">Min order qty</label>
                        <input id="min_order_qty" v-model="form.min_order_qty" type="number" min="1"
                            class="w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-400" />
                    </div>
                    <div>
                        <label for="max_order_qty" class="mb-1 block text-sm font-medium">Max order qty</label>
                        <input id="max_order_qty" v-model="form.max_order_qty" type="number" min="1"
                            class="w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-400" />
                    </div>
                </div>

                <div class="mt-4 grid grid-cols-1 gap-2 text-sm sm:grid-cols-2">
                    <label class="flex items-center gap-2">
                        <input v-model="form.is_active" type="checkbox" class="h-4 w-4" />
                        Active
                    </label>
                    <label class="flex items-center gap-2">
                        <input v-model="form.is_featured" type="checkbox" class="h-4 w-4" />
                        Featured
                    </label>
                    <label class="flex items-center gap-2">
                        <input v-model="form.is_digital" type="checkbox" class="h-4 w-4" />
                        Digital product
                    </label>
                    <label class="flex items-center gap-2">
                        <input v-model="form.has_inventory" type="checkbox" class="h-4 w-4" />
                        Track inventory
                    </label>
                    <label class="flex items-center gap-2">
                        <input v-model="form.has_warranty" type="checkbox" class="h-4 w-4" />
                        Has warranty
                    </label>
                </div>

                <div v-if="form.has_warranty" class="mt-4 grid grid-cols-1 gap-3 sm:grid-cols-2">
                    <div>
                        <label for="warranty_value" class="mb-1 block text-sm font-medium">Warranty length</label>
                        <input id="warranty_value" v-model="form.warranty_value" type="number" min="1"
                            class="w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-400" />
                    </div>
                    <div>
                        <label for="warranty_type" class="mb-1 block text-sm font-medium">Warranty unit</label>
                        <select id="warranty_type" v-model="form.warranty_type"
                            class="w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-400">
                            <option value="hour">Hours</option>
                            <option value="day">Days</option>
                            <option value="month">Months</option>
                            <option value="year">Years</option>
                        </select>
                    </div>
                </div>
            </div>

            <div class="rounded-xl border border-gray-300 bg-white p-4 shadow-sm">
                <h2 class="mb-3 text-sm font-semibold text-gray-900">SEO</h2>

                <label for="seo_title" class="mb-1 block text-sm font-medium">SEO title</label>
                <StoreInput id="seo_title" v-model="form.seo_title" type="text" />

                <label for="seo_description" class="mb-1 mt-3 block text-sm font-medium">SEO description</label>
                <textarea id="seo_description" v-model="form.seo_description" rows="3"
                    class="w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-400"></textarea>
            </div>

            <CardsError v-if="error" :text="error" />

            <div class="flex justify-end gap-2">
                <NuxtLink :to="`/stores/${uuid}/products`"
                    class="rounded-lg border border-gray-300 bg-white px-4 py-2 text-sm font-medium hover:bg-gray-50">
                    Cancel
                </NuxtLink>
                <button type="submit" :disabled="saving || !form.title.trim()"
                    class="rounded-lg bg-gray-900 px-4 py-2 text-sm font-medium text-white hover:bg-gray-800 disabled:opacity-50">
                    {{ saving ? 'Creating...' : 'Create product' }}
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
    title: '',
    slug: '',
    description: '',
    content: '',
    sort_order: 1,
    min_order_qty: '',
    max_order_qty: '',
    is_active: true,
    is_featured: false,
    is_digital: false,
    has_inventory: false,
    has_warranty: false,
    warranty_type: 'month',
    warranty_value: '',
    seo_title: '',
    seo_description: ''
});

const saving = ref(false);
const error = ref('');

function toInt(value) {
    if (value === '' || value === null || value === undefined) return null;
    const n = Number(value);
    return Number.isInteger(n) ? n : null;
}

async function submit() {
    if (!form.title.trim()) return;

    saving.value = true;
    error.value = '';
    try {
        await authFetch(`/system/stores/${uuid.value}/products`, {
            method: 'POST',
            body: {
                title: form.title.trim(),
                slug: form.slug.trim() || null,
                description: form.description,
                content: form.content,
                sort_order: toInt(form.sort_order) ?? 1,
                min_order_qty: toInt(form.min_order_qty),
                max_order_qty: toInt(form.max_order_qty),
                is_active: form.is_active,
                is_featured: form.is_featured,
                is_digital: form.is_digital,
                has_inventory: form.has_inventory,
                has_warranty: form.has_warranty,
                warranty_type: form.has_warranty ? form.warranty_type : null,
                warranty_value: form.has_warranty ? toInt(form.warranty_value) : null,
                seo_title: form.seo_title.trim() || null,
                seo_description: form.seo_description.trim() || null
            }
        });

        await navigateTo(`/stores/${uuid.value}/products`);
    } catch (e) {
        error.value = e.statusMessage || 'Failed to create product.';
    } finally {
        saving.value = false;
    }
}

useHead({
    title: 'Create product - webedit99'
});
</script>