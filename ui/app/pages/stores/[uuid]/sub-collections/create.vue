<template>
    <div class="mx-auto flex max-w-xl flex-col gap-4">
        <div class="flex items-center justify-between">
            <h1 class="text-xl font-semibold text-gray-900">Create sub collection</h1>
            <NuxtLink :to="`/stores/${uuid}/sub-collections`"
                class="rounded-lg border border-gray-300 bg-white px-3 py-1.5 text-sm font-medium hover:bg-gray-50">
                Back
            </NuxtLink>
        </div>

        <form class="flex flex-col gap-4" @submit.prevent="submit">
            <div class="rounded-xl border border-gray-300 bg-white p-4 shadow-sm">
                <label for="collection" class="mb-1 block text-sm font-medium">Collection</label>
                <select id="collection" v-model="form.collection_id"
                    class="w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-400">
                    <option value="" disabled>Select a collection</option>
                    <option v-for="c in collections" :key="c.id" :value="c.id">{{ c.title }}</option>
                </select>
                <p v-if="!loadingCollections && collections.length === 0" class="mt-1 text-xs text-gray-500">
                    No collections yet.
                    <NuxtLink :to="`/stores/${uuid}/collections/create`" class="underline">Create one first</NuxtLink>.
                </p>

                <label for="title" class="mb-1 mt-3 block text-sm font-medium">Title</label>
                <StoreInput id="title" v-model="form.title" type="text" placeholder="Sub collection title" />

                <label for="slug" class="mb-1 mt-3 block text-sm font-medium">Slug</label>
                <StoreInput id="slug" v-model="form.slug" type="text" placeholder="Generated from title if empty" />

                <label for="sort_order" class="mb-1 mt-3 block text-sm font-medium">Sort order</label>
                <input id="sort_order" v-model.number="form.sort_order" type="number"
                    class="w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-400" />

                <div class="mt-4 flex flex-col gap-2 text-sm">
                    <label class="flex items-center gap-2">
                        <input v-model="form.is_active" type="checkbox" class="h-4 w-4" />
                        Active
                    </label>
                    <label class="flex items-center gap-2">
                        <input v-model="form.is_featured" type="checkbox" class="h-4 w-4" />
                        Featured
                    </label>
                </div>
            </div>

            <CardsError v-if="error" :text="error" />

            <div class="flex justify-end gap-2">
                <NuxtLink :to="`/stores/${uuid}/sub-collections`"
                    class="rounded-lg border border-gray-300 bg-white px-4 py-2 text-sm font-medium hover:bg-gray-50">
                    Cancel
                </NuxtLink>
                <button type="submit" :disabled="saving || !form.title.trim() || !form.collection_id"
                    class="rounded-lg bg-gray-900 px-4 py-2 text-sm font-medium text-white hover:bg-gray-800 disabled:opacity-50">
                    {{ saving ? 'Creating...' : 'Create sub collection' }}
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
    collection_id: '',
    title: '',
    slug: '',
    sort_order: 1,
    is_active: true,
    is_featured: false
});

const collections = ref([]);
const loadingCollections = ref(true);
const saving = ref(false);
const error = ref('');

async function fetchCollections() {
    loadingCollections.value = true;
    try {
        const data = await authFetch(`/system/stores/${uuid.value}/collections?per_page=100`, {
            method: 'GET'
        });
        collections.value = data?.data || [];
    } catch (e) {
        error.value = e.statusMessage || 'Failed to load collections.';
    } finally {
        loadingCollections.value = false;
    }
}

async function submit() {
    if (!form.title.trim() || !form.collection_id) return;

    saving.value = true;
    error.value = '';
    try {
        await authFetch(`/system/stores/${uuid.value}/sub-collections`, {
            method: 'POST',
            body: {
                collection_id: form.collection_id,
                title: form.title.trim(),
                slug: form.slug.trim() || null,
                sort_order: Number.isInteger(form.sort_order) ? form.sort_order : 1,
                is_active: form.is_active,
                is_featured: form.is_featured
            }
        });

        await navigateTo(`/stores/${uuid.value}/sub-collections`);
    } catch (e) {
        error.value = e.statusMessage || 'Failed to create sub collection.';
    } finally {
        saving.value = false;
    }
}

onMounted(fetchCollections);

useHead({
    title: 'Create sub collection - webedit99'
});
</script>