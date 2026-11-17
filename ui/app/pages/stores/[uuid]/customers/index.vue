<template>
    <div class="flex flex-col gap-4">
        <div class="flex items-center justify-between">
            <h1 class="text-xl font-semibold text-gray-900">Customers</h1>
            <NuxtLink :to="`/stores/${uuid}/customers/create`"
                class="rounded-lg bg-gray-900 px-4 py-2 text-sm font-medium text-white hover:bg-gray-800">
                Create customer
            </NuxtLink>
        </div>

        <div class="flex flex-wrap items-center gap-2">
            <div class="w-64">
                <StoreInput id="search" v-model="search" type="text" placeholder="Search name, email, phone" />
            </div>
            <select v-model="status"
                class="rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-400">
                <option value="">All statuses</option>
                <option value="active">Active</option>
                <option value="inactive">Inactive</option>
                <option value="banned">Banned</option>
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
                        <th class="px-4 py-2 font-semibold">Name</th>
                        <th class="px-4 py-2 font-semibold">Email</th>
                        <th class="px-4 py-2 font-semibold">Phone</th>
                        <th class="px-4 py-2 font-semibold">Status</th>
                        <th class="px-4 py-2 font-semibold">Last login</th>
                        <th class="px-4 py-2 font-semibold">Created</th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-if="loading">
                        <td colspan="6" class="px-4 py-10 text-center text-gray-500">Loading customers...</td>
                    </tr>
                    <tr v-else-if="items.length === 0">
                        <td colspan="6" class="px-4 py-10 text-center text-gray-500">No customers found.</td>
                    </tr>
                    <template v-else>
                        <tr v-for="c in items" :key="c.id"
                            class="border-b border-gray-200 last:border-0 hover:bg-gray-50">
                            <td class="px-4 py-2 font-medium text-gray-900">
                                {{ c.name }}
                                <p class="text-xs font-normal text-gray-500">@{{ c.username }}</p>
                            </td>
                            <td class="px-4 py-2 text-gray-600">
                                {{ c.email }}
                                <span v-if="c.verified_at"
                                    class="ml-1 rounded-full bg-blue-100 px-2 py-0.5 text-xs font-medium text-blue-700">
                                    Verified
                                </span>
                            </td>
                            <td class="px-4 py-2 text-gray-600">{{ c.phone || '-' }}</td>
                            <td class="px-4 py-2">
                                <span v-if="c.is_banned"
                                    class="rounded-full bg-red-100 px-2 py-0.5 text-xs font-medium text-red-700">
                                    Banned
                                </span>
                                <span v-else class="rounded-full px-2 py-0.5 text-xs font-medium"
                                    :class="c.is_active ? 'bg-green-100 text-green-700' : 'bg-gray-200 text-gray-700'">
                                    {{ c.is_active ? 'Active' : 'Inactive' }}
                                </span>
                            </td>
                            <td class="px-4 py-2 text-gray-600">{{ c.last_login ? formatDate(c.last_login) : '-' }}</td>
                            <td class="px-4 py-2 text-gray-600">{{ formatDate(c.created_at) }}</td>
                        </tr>
                    </template>
                </tbody>
            </table>
        </div>

        <div class="flex items-center justify-between text-sm text-gray-600">
            <span>{{ total }} customer{{ total === 1 ? '' : 's' }}</span>
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
const total = ref(0);
const loading = ref(true);
const error = ref('');

const page = ref(1);
const perPage = 20;
const search = ref('');
const status = ref('');

const totalPages = computed(() => Math.max(1, Math.ceil(total.value / perPage)));

async function fetchItems() {
    loading.value = true;
    error.value = '';
    try {
        const params = new URLSearchParams({
            page: String(page.value),
            per_page: String(perPage)
        });
        if (search.value.trim()) params.set('search', search.value.trim());
        if (status.value === 'active') params.set('is_active', 'true');
        if (status.value === 'inactive') params.set('is_active', 'false');
        if (status.value === 'banned') params.set('is_banned', 'true');

        const data = await authFetch(`/system/stores/${uuid.value}/customers?${params}`, {
            method: 'GET'
        });

        items.value = data?.data || [];
        total.value = data?.total || 0;
    } catch (e) {
        error.value = e.statusMessage || 'Failed to load customers.';
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

watch(status, () => {
    if (page.value === 1) fetchItems();
    else page.value = 1;
});

watch(page, fetchItems);

onMounted(fetchItems);
onBeforeUnmount(() => clearTimeout(timer));

useHead({
    title: 'Customers - webedit99'
});
</script>