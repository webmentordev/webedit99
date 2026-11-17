<template>
    <div class="flex flex-col gap-4">
        <div class="flex items-center justify-between">
            <h1 class="text-xl font-semibold text-gray-900">Addresses</h1>
            <NuxtLink :to="`/stores/${uuid}/customer-addresses/create`"
                class="rounded-lg bg-gray-900 px-4 py-2 text-sm font-medium text-white hover:bg-gray-800">
                Create address
            </NuxtLink>
        </div>

        <div class="flex flex-wrap items-center gap-2">
            <div class="w-64">
                <StoreInput id="search" v-model="search" type="text" placeholder="Search name, street, city" />
            </div>
            <select v-model="customerId"
                class="rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-400">
                <option value="">All customers</option>
                <option v-for="c in customers" :key="c.id" :value="c.id">{{ c.name }} ({{ c.email }})</option>
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
                        <th class="px-4 py-2 font-semibold">Customer</th>
                        <th class="px-4 py-2 font-semibold">Recipient</th>
                        <th class="px-4 py-2 font-semibold">Address</th>
                        <th class="px-4 py-2 font-semibold">City</th>
                        <th class="px-4 py-2 font-semibold">Country</th>
                        <th class="px-4 py-2 font-semibold">Defaults</th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-if="loading">
                        <td colspan="6" class="px-4 py-10 text-center text-gray-500">Loading addresses...</td>
                    </tr>
                    <tr v-else-if="items.length === 0">
                        <td colspan="6" class="px-4 py-10 text-center text-gray-500">No addresses found.</td>
                    </tr>
                    <template v-else>
                        <tr v-for="a in items" :key="a.id"
                            class="border-b border-gray-200 last:border-0 hover:bg-gray-50">
                            <td class="px-4 py-2 font-medium text-gray-900">{{ a.customer_name }}</td>
                            <td class="px-4 py-2 text-gray-600">
                                {{ a.name }}
                                <span v-if="a.label" class="ml-1 text-xs text-gray-500">({{ a.label }})</span>
                            </td>
                            <td class="px-4 py-2 text-gray-600">
                                {{ a.address_1 }}<span v-if="a.address_2">, {{ a.address_2 }}</span>
                                <p class="text-xs text-gray-500">{{ a.postal_code }}</p>
                            </td>
                            <td class="px-4 py-2 text-gray-600">{{ a.city }}</td>
                            <td class="px-4 py-2 text-gray-600">{{ a.country }}</td>
                            <td class="px-4 py-2">
                                <span v-if="a.is_default_shipping"
                                    class="mr-1 rounded-full bg-blue-100 px-2 py-0.5 text-xs font-medium text-blue-700">
                                    Shipping
                                </span>
                                <span v-if="a.is_default_billing"
                                    class="rounded-full bg-green-100 px-2 py-0.5 text-xs font-medium text-green-700">
                                    Billing
                                </span>
                            </td>
                        </tr>
                    </template>
                </tbody>
            </table>
        </div>

        <div class="flex items-center justify-between text-sm text-gray-600">
            <span>{{ total }} address{{ total === 1 ? '' : 'es' }}</span>
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
const customers = ref([]);
const total = ref(0);
const loading = ref(true);
const error = ref('');

const page = ref(1);
const perPage = 20;
const search = ref('');
const customerId = ref('');

const totalPages = computed(() => Math.max(1, Math.ceil(total.value / perPage)));

async function fetchCustomers() {
    try {
        const data = await authFetch(`/system/stores/${uuid.value}/customers?per_page=100`, {
            method: 'GET'
        });
        customers.value = data?.data || [];
    } catch (e) {
        customers.value = [];
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
        if (customerId.value) params.set('customer_id', customerId.value);

        const data = await authFetch(`/system/stores/${uuid.value}/customer-addresses?${params}`, {
            method: 'GET'
        });

        items.value = data?.data || [];
        total.value = data?.total || 0;
    } catch (e) {
        error.value = e.statusMessage || 'Failed to load addresses.';
    } finally {
        loading.value = false;
    }
}

let timer;
watch(search, () => {
    clearTimeout(timer);
    timer = setTimeout(() => {
        if (page.value === 1) fetchItems();
        else page.value = 1;
    }, 300);
});

watch(customerId, () => {
    if (page.value === 1) fetchItems();
    else page.value = 1;
});

watch(page, fetchItems);

onMounted(() => {
    fetchCustomers();
    fetchItems();
});
onBeforeUnmount(() => clearTimeout(timer));

useHead({
    title: 'Addresses - webedit99'
});
</script>