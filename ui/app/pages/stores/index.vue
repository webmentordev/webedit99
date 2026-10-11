<template>
    <div class="min-h-screen text-gray-900 flex items-center justify-center w-full">
        <div class="max-w-lg m-auto px-4 py-10 w-full">
            <div class="flex items-center justify-between w-full">
                <h1 class="text-2xl font-semibold">Stores</h1>
                <button type="button" @click="openModal"
                    class="rounded-lg bg-gray-900 px-4 py-2 text-sm font-medium text-white hover:bg-gray-800">
                    Create store
                </button>
            </div>

            <div class="mt-6 rounded-xl border border-gray-300 bg-white shadow-sm w-full">
                <div class="border-b border-gray-300 p-3">
                    <StoreInput id="search" v-model="search" type="text" placeholder="Search stores" />
                </div>

                <div v-if="loading" class="p-10 text-center text-sm text-gray-500">Loading stores...</div>

                <div v-else-if="error" class="p-4">
                    <CardsError :text="error" />
                    <button type="button" @click="fetchStores"
                        class="mt-3 rounded-lg border border-gray-300 bg-white px-3 py-1.5 text-sm font-medium hover:bg-gray-50">
                        Try again
                    </button>
                </div>

                <div v-else-if="stores.length === 0" class="p-10 text-center">
                    <p class="font-semibold">No stores yet</p>
                    <p class="mt-1 text-sm text-gray-500">Create your first store to start selling.</p>
                    <button type="button" @click="openModal"
                        class="inline-block mt-4 rounded-lg bg-gray-900 px-4 py-2 text-sm font-medium text-white hover:bg-gray-800">
                        Create store
                    </button>
                </div>

                <div v-else-if="filtered.length === 0" class="p-10 text-center">
                    <p class="font-semibold">No results found</p>
                    <p class="mt-1 text-sm text-gray-500">Try a different search term.</p>
                </div>

                <ul v-else class="divide-y divide-gray-200">
                    <li v-for="store in filtered" :key="store.id">
                        <NuxtLink :to="`/stores/${store.id}`"
                            class="flex items-center justify-between gap-4 px-4 py-3 hover:bg-gray-50">
                            <div class="flex items-center gap-3 min-w-0">
                                <div
                                    class="w-10 h-10 shrink-0 rounded-lg bg-gray-900 text-white flex items-center justify-center text-sm font-bold uppercase">
                                    {{ (store.title || '?').charAt(0) }}
                                </div>
                                <div class="min-w-0">
                                    <p class="text-sm font-semibold truncate">{{ store.title }}</p>
                                    <p class="text-xs text-gray-500 truncate">{{ store.id }}</p>
                                </div>
                            </div>
                            <div class="flex items-center gap-2 shrink-0">
                                <span v-if="!store.is_active"
                                    class="rounded-full bg-red-100 px-2 py-0.5 text-xs font-medium text-red-700">
                                    Inactive
                                </span>
                                <span v-if="store.is_owner"
                                    class="rounded-full bg-green-100 px-2 py-0.5 text-xs font-medium text-green-700">
                                    Owner
                                </span>
                                <span v-else-if="store.is_worker"
                                    class="rounded-full bg-blue-100 px-2 py-0.5 text-xs font-medium text-blue-700">
                                    Worker
                                </span>
                                <span class="text-gray-400">&#9656;</span>
                            </div>
                        </NuxtLink>
                    </li>
                </ul>

                <div v-if="!loading && !error && stores.length > 0"
                    class="border-t border-gray-300 px-4 py-2 text-xs text-gray-500">
                    Showing {{ filtered.length }} of {{ stores.length }} stores
                </div>
            </div>
        </div>

        <Teleport to="body">
            <div v-if="showModal" class="fixed inset-0 z-50 flex items-center justify-center bg-black/40 px-4">
                <div ref="modalRef" class="w-full max-w-md rounded-xl bg-white p-6 shadow-xl">
                    <h2 class="text-lg font-semibold">Create store</h2>
                    <p class="mt-1 text-sm text-gray-500">Give your store a title. You can change it later.</p>

                    <form class="mt-4" @submit.prevent="createStore">
                        <label for="store-title" class="mb-1 block text-sm font-medium">Store title</label>
                        <StoreInput id="store-title" v-model="title" type="text" placeholder="My awesome store" />

                        <div v-if="createError" class="mt-3">
                            <CardsError :text="createError" />
                        </div>

                        <div class="mt-6 flex justify-end gap-2">
                            <button type="button" @click="closeModal" :disabled="creating"
                                class="rounded-lg border border-gray-300 bg-white px-4 py-2 text-sm font-medium hover:bg-gray-50 disabled:opacity-50">
                                Cancel
                            </button>
                            <button type="submit" :disabled="creating || !title.trim()"
                                class="rounded-lg bg-gray-900 px-4 py-2 text-sm font-medium text-white hover:bg-gray-800 disabled:opacity-50">
                                {{ creating ? 'Creating...' : 'Create' }}
                            </button>
                        </div>
                    </form>
                </div>
            </div>
        </Teleport>
    </div>
</template>

<script setup>
definePageMeta({
    middleware: 'auth',
    layout: 'simple'
});

const { authFetch } = useAuthFetch();

const stores = ref([]);
const search = ref('');
const loading = ref(true);
const error = ref('');

const showModal = ref(false);
const modalRef = ref(null);
const title = ref('');
const creating = ref(false);
const createError = ref('');

const filtered = computed(() => {
    const q = search.value.trim().toLowerCase();
    if (!q) return stores.value;
    return stores.value.filter((store) =>
        [store.title, store.slug, store.id]
            .filter(Boolean)
            .some((value) => String(value).toLowerCase().includes(q))
    );
});

function openModal() {
    title.value = '';
    createError.value = '';
    showModal.value = true;
}

function closeModal() {
    if (creating.value) return;
    showModal.value = false;
}

onClickOutside(modalRef, closeModal);
onKeyStroke('Escape', closeModal);

async function createStore() {
    const value = title.value.trim();
    if (!value) return;

    creating.value = true;
    createError.value = '';
    try {
        const data = await authFetch('/system/stores', {
            method: 'POST',
            body: { title: value }
        });

        showModal.value = false;

        const created = data?.store || data?.data || data;
        if (created?.id) {
            await navigateTo(`/stores/${created.id}`);
        } else {
            await fetchStores();
        }
    } catch (e) {
        createError.value = e.statusMessage || 'Failed to create store.';
    } finally {
        creating.value = false;
    }
}

async function fetchStores() {
    loading.value = true;
    error.value = '';
    try {
        const data = await authFetch('/system/stores', {
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