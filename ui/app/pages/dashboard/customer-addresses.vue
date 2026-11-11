<template>
    <div class="min-h-screen bg-yellow-50 font-mono text-black p-6">
        <div class="max-w-6xl mx-auto">
            <div class="flex items-center justify-between gap-4">
                <h1 class="text-2xl font-bold bg-black text-yellow-300 px-3 py-1 inline-block">customer_addresses_</h1>
                <button type="button" @click="fetchAddresses" :disabled="loading"
                    class="border-2 border-black bg-white px-4 py-2 text-sm font-bold shadow-[3px_3px_0_0_#000] hover:translate-x-px hover:translate-y-px hover:shadow-[2px_2px_0_0_#000] transition-all disabled:opacity-50">
                    {{ loading ? 'Loading...' : 'Refresh' }}
                </button>
            </div>

            <div class="mt-6 border-4 border-black bg-white shadow-[6px_6px_0_0_#000]">
                <div class="border-b-4 border-black bg-yellow-100 p-3">
                    <input v-model="search" type="text" placeholder="Search addresses..."
                        class="w-full border-2 border-black bg-white px-3 py-2 text-sm font-bold placeholder:text-gray-500 focus:outline-none focus:bg-yellow-50" />
                </div>

                <div v-if="loading" class="p-10 text-center text-sm font-bold">Loading addresses...</div>

                <div v-else-if="error" class="p-6 text-center">
                    <p class="inline-block border-2 border-black bg-red-300 px-3 py-2 text-sm font-bold">{{ error }}</p>
                    <div>
                        <button type="button" @click="fetchAddresses"
                            class="mt-4 border-2 border-black bg-white px-4 py-2 text-sm font-bold hover:bg-yellow-100">
                            Try again
                        </button>
                    </div>
                </div>

                <div v-else-if="addresses.length === 0" class="p-10 text-center">
                    <p class="font-bold">No addresses yet</p>
                </div>

                <div v-else-if="filtered.length === 0" class="p-10 text-center">
                    <p class="font-bold">No results found</p>
                    <p class="mt-1 text-sm">Try a different search term.</p>
                </div>

                <div v-else class="overflow-x-auto">
                    <table class="w-full text-sm">
                        <thead>
                            <tr class="border-b-4 border-black bg-yellow-300 text-left">
                                <th class="px-4 py-2 font-bold">Recipient</th>
                                <th class="px-4 py-2 font-bold border-l-2 border-black">Customer</th>
                                <th class="px-4 py-2 font-bold border-l-2 border-black">Store</th>
                                <th class="px-4 py-2 font-bold border-l-2 border-black">Address</th>
                                <th class="px-4 py-2 font-bold border-l-2 border-black">Defaults</th>
                                <th class="px-4 py-2 font-bold border-l-2 border-black">Created</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr v-for="address in filtered" :key="address.id"
                                class="border-b-2 border-black last:border-b-0 hover:bg-yellow-100">
                                <td class="px-4 py-2">
                                    <p class="font-bold">{{ address.name }}</p>
                                    <span v-if="address.label"
                                        class="mt-1 inline-block border-2 border-black bg-black px-2 py-0.5 text-xs font-bold text-yellow-300">
                                        {{ address.label }}
                                    </span>
                                    <p v-if="address.phone" class="mt-1 text-xs">{{ address.phone }}</p>
                                </td>
                                <td class="px-4 py-2 border-l-2 border-black">
                                    <p class="font-bold">{{ address.customer_name }}</p>
                                    <p class="text-xs">{{ address.customer_email }}</p>
                                </td>
                                <td class="px-4 py-2 border-l-2 border-black">
                                    <p class="font-bold">{{ address.store_title }}</p>
                                    <p class="text-xs">{{ address.store_slug }}</p>
                                </td>
                                <td class="px-4 py-2 border-l-2 border-black">
                                    <p>{{ address.address_1 }}</p>
                                    <p v-if="address.address_2">{{ address.address_2 }}</p>
                                    <p class="text-xs">{{ address.postal_code }} {{ address.city }}</p>
                                    <p class="text-xs">{{ address.country }}</p>
                                </td>
                                <td class="px-4 py-2 border-l-2 border-black">
                                    <div class="flex flex-wrap gap-1">
                                        <span v-if="address.is_default_shipping"
                                            class="border-2 border-black bg-green-300 px-2 py-0.5 text-xs font-bold">
                                            Shipping
                                        </span>
                                        <span v-if="address.is_default_billing"
                                            class="border-2 border-black bg-orange-300 px-2 py-0.5 text-xs font-bold">
                                            Billing
                                        </span>
                                        <span
                                            v-if="!address.is_default_shipping && !address.is_default_billing">-</span>
                                    </div>
                                </td>
                                <td class="px-4 py-2 border-l-2 border-black whitespace-nowrap">
                                    {{ formatDate(address.created_at) }}
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>

                <div v-if="!loading && !error && addresses.length > 0"
                    class="border-t-4 border-black bg-yellow-50 px-4 py-2 text-xs font-bold">
                    Showing {{ filtered.length }} of {{ addresses.length }} addresses
                </div>
            </div>
        </div>
    </div>
</template>

<script setup>
definePageMeta({
    middleware: 'admin',
    layout: 'admin'
});

const { authFetch } = useAuthFetch();

const addresses = ref([]);
const search = ref('');
const loading = ref(true);
const error = ref('');

const filtered = computed(() => {
    const q = search.value.trim().toLowerCase();
    if (!q) return addresses.value;
    return addresses.value.filter((address) =>
        [
            address.label,
            address.name,
            address.phone,
            address.address_1,
            address.address_2,
            address.postal_code,
            address.city,
            address.country,
            address.customer_name,
            address.customer_email,
            address.store_title,
            address.store_slug
        ]
            .filter(Boolean)
            .some((value) => String(value).toLowerCase().includes(q))
    );
});

function formatDate(value) {
    if (!value) return '-';
    return new Date(value).toLocaleString();
}

async function fetchAddresses() {
    loading.value = true;
    error.value = '';
    try {
        const data = await authFetch('/admin/customer-addresses', {
            method: 'GET'
        });
        if (Array.isArray(data)) {
            addresses.value = data;
        } else {
            addresses.value = data?.addresses || data?.data || [];
        }
    } catch (e) {
        error.value = e.statusMessage || 'Something went wrong!';
    } finally {
        loading.value = false;
    }
}

onMounted(fetchAddresses);

useHead({
    title: 'Customer Addresses - webedit99'
});
</script>