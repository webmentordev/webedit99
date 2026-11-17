<template>
    <div class="mx-auto flex max-w-xl flex-col gap-4">
        <div class="flex items-center justify-between">
            <h1 class="text-xl font-semibold text-gray-900">Create address</h1>
            <NuxtLink :to="`/stores/${uuid}/customer-addresses`"
                class="rounded-lg border border-gray-300 bg-white px-3 py-1.5 text-sm font-medium hover:bg-gray-50">
                Back
            </NuxtLink>
        </div>

        <form class="flex flex-col gap-4" @submit.prevent="submit">
            <div class="rounded-xl border border-gray-300 bg-white p-4 shadow-sm">
                <label for="customer" class="mb-1 block text-sm font-medium">Customer</label>
                <select id="customer" v-model="form.customer_id"
                    class="w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-400">
                    <option value="" disabled>Select a customer</option>
                    <option v-for="c in customers" :key="c.id" :value="c.id">{{ c.name }} ({{ c.email }})</option>
                </select>
                <p v-if="!loadingCustomers && customers.length === 0" class="mt-1 text-xs text-gray-500">
                    No customers yet.
                    <NuxtLink :to="`/stores/${uuid}/customers/create`" class="underline">Create one first</NuxtLink>.
                </p>

                <label for="label" class="mb-1 mt-3 block text-sm font-medium">Label</label>
                <StoreInput id="label" v-model="form.label" type="text" placeholder="e.g. Home" />

                <label for="name" class="mb-1 mt-3 block text-sm font-medium">Recipient name</label>
                <StoreInput id="name" v-model="form.name" type="text" />

                <label for="phone" class="mb-1 mt-3 block text-sm font-medium">Phone</label>
                <StoreInput id="phone" v-model="form.phone" type="text" />

                <label for="address_1" class="mb-1 mt-3 block text-sm font-medium">Address line 1</label>
                <StoreInput id="address_1" v-model="form.address_1" type="text" />

                <label for="address_2" class="mb-1 mt-3 block text-sm font-medium">Address line 2</label>
                <StoreInput id="address_2" v-model="form.address_2" type="text" />

                <div class="mt-3 grid grid-cols-1 gap-3 sm:grid-cols-3">
                    <div>
                        <label for="city" class="mb-1 block text-sm font-medium">City</label>
                        <StoreInput id="city" v-model="form.city" type="text" />
                    </div>
                    <div>
                        <label for="postal_code" class="mb-1 block text-sm font-medium">Postal code</label>
                        <StoreInput id="postal_code" v-model="form.postal_code" type="text" />
                    </div>
                    <div>
                        <label for="country" class="mb-1 block text-sm font-medium">Country</label>
                        <StoreInput id="country" v-model="form.country" type="text" />
                    </div>
                </div>

                <div class="mt-4 flex flex-col gap-2 text-sm">
                    <label class="flex items-center gap-2">
                        <input v-model="form.is_default_shipping" type="checkbox" class="h-4 w-4" />
                        Default shipping address
                    </label>
                    <label class="flex items-center gap-2">
                        <input v-model="form.is_default_billing" type="checkbox" class="h-4 w-4" />
                        Default billing address
                    </label>
                </div>
            </div>

            <CardsError v-if="error" :text="error" />

            <div class="flex justify-end gap-2">
                <NuxtLink :to="`/stores/${uuid}/customer-addresses`"
                    class="rounded-lg border border-gray-300 bg-white px-4 py-2 text-sm font-medium hover:bg-gray-50">
                    Cancel
                </NuxtLink>
                <button type="submit" :disabled="saving || !canSubmit"
                    class="rounded-lg bg-gray-900 px-4 py-2 text-sm font-medium text-white hover:bg-gray-800 disabled:opacity-50">
                    {{ saving ? 'Creating...' : 'Create address' }}
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
    customer_id: typeof route.query.customer_id === 'string' ? route.query.customer_id : '',
    label: '',
    name: '',
    phone: '',
    address_1: '',
    address_2: '',
    postal_code: '',
    city: '',
    country: '',
    is_default_shipping: false,
    is_default_billing: false
});

const customers = ref([]);
const loadingCustomers = ref(true);
const saving = ref(false);
const error = ref('');

const canSubmit = computed(
    () =>
        form.customer_id &&
        form.name.trim() &&
        form.address_1.trim() &&
        form.postal_code.trim() &&
        form.city.trim() &&
        form.country.trim()
);

async function fetchCustomers() {
    loadingCustomers.value = true;
    try {
        const data = await authFetch(`/system/stores/${uuid.value}/customers?per_page=100`, {
            method: 'GET'
        });
        customers.value = data?.data || [];
    } catch (e) {
        error.value = e.statusMessage || 'Failed to load customers.';
    } finally {
        loadingCustomers.value = false;
    }
}

async function submit() {
    if (!canSubmit.value) return;

    saving.value = true;
    error.value = '';
    try {
        await authFetch(`/system/stores/${uuid.value}/customer-addresses`, {
            method: 'POST',
            body: {
                customer_id: form.customer_id,
                label: form.label.trim() || null,
                name: form.name.trim(),
                phone: form.phone.trim() || null,
                address_1: form.address_1.trim(),
                address_2: form.address_2.trim() || null,
                postal_code: form.postal_code.trim(),
                city: form.city.trim(),
                country: form.country.trim(),
                is_default_shipping: form.is_default_shipping,
                is_default_billing: form.is_default_billing
            }
        });

        await navigateTo(`/stores/${uuid.value}/customer-addresses`);
    } catch (e) {
        error.value = e.statusMessage || 'Failed to create address.';
    } finally {
        saving.value = false;
    }
}

onMounted(fetchCustomers);

useHead({
    title: 'Create address - webedit99'
});
</script>