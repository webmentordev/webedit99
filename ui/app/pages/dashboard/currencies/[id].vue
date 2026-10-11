<template>
    <div class="min-h-screen bg-yellow-50 font-mono text-black p-6">
        <div class="max-w-xl mx-auto">
            <div class="flex items-center justify-between gap-4">
                <h1 class="text-2xl font-bold bg-black text-yellow-300 px-3 py-1 inline-block">edit_currency_</h1>
                <NuxtLink to="/dashboard/currencies"
                    class="border-2 border-black bg-white px-4 py-2 text-sm font-bold shadow-[3px_3px_0_0_#000] hover:translate-x-px hover:translate-y-px hover:shadow-[2px_2px_0_0_#000] transition-all">
                    Back
                </NuxtLink>
            </div>

            <div class="mt-6 border-4 border-black bg-white shadow-[6px_6px_0_0_#000]">
                <div v-if="loading" class="p-10 text-center text-sm font-bold">Loading currency...</div>

                <div v-else-if="loadError" class="p-6 text-center">
                    <p class="inline-block border-2 border-black bg-red-300 px-3 py-2 text-sm font-bold">{{ loadError }}
                    </p>
                    <div>
                        <button type="button" @click="fetchCurrency"
                            class="mt-4 border-2 border-black bg-white px-4 py-2 text-sm font-bold hover:bg-yellow-100">
                            Try again
                        </button>
                    </div>
                </div>

                <form v-else @submit.prevent="submit">
                    <div class="flex flex-col gap-4 p-4">
                        <div>
                            <label for="currency_code" class="mb-1 block text-sm font-bold">Currency code</label>
                            <input id="currency_code" v-model="form.currency_code" type="text" maxlength="10" required
                                class="w-full border-2 border-black bg-white px-3 py-2 text-sm font-bold uppercase focus:outline-none focus:bg-yellow-50" />
                        </div>

                        <div>
                            <label for="symbol" class="mb-1 block text-sm font-bold">Symbol</label>
                            <input id="symbol" v-model="form.symbol" type="text" required
                                class="w-full border-2 border-black bg-white px-3 py-2 text-sm font-bold focus:outline-none focus:bg-yellow-50" />
                        </div>

                        <div>
                            <label for="decimal_places" class="mb-1 block text-sm font-bold">Decimal places</label>
                            <input id="decimal_places" v-model="form.decimal_places" type="number" min="0" max="10"
                                step="1"
                                class="w-full border-2 border-black bg-white px-3 py-2 text-sm font-bold focus:outline-none focus:bg-yellow-50" />
                        </div>

                        <div>
                            <label for="value" class="mb-1 block text-sm font-bold">Value</label>
                            <input id="value" v-model="form.value" type="number" step="any" min="0" required
                                class="w-full border-2 border-black bg-white px-3 py-2 text-sm font-bold focus:outline-none focus:bg-yellow-50" />
                        </div>

                        <p v-if="error" class="border-2 border-black bg-red-300 px-3 py-2 text-sm font-bold">{{ error
                        }}</p>
                        <p v-if="saved" class="border-2 border-black bg-green-300 px-3 py-2 text-sm font-bold">Saved.
                        </p>
                    </div>

                    <div class="flex justify-end gap-2 border-t-4 border-black bg-yellow-100 p-3">
                        <NuxtLink to="/dashboard/currencies"
                            class="border-2 border-black bg-white px-4 py-2 text-sm font-bold hover:bg-yellow-50">
                            Cancel
                        </NuxtLink>
                        <button type="submit" :disabled="saving || !canSubmit"
                            class="border-2 border-black bg-yellow-300 px-4 py-2 text-sm font-bold shadow-[3px_3px_0_0_#000] hover:translate-x-px hover:translate-y-px hover:shadow-[2px_2px_0_0_#000] transition-all disabled:opacity-50">
                            {{ saving ? 'Saving...' : 'Save changes' }}
                        </button>
                    </div>
                </form>
            </div>
        </div>
    </div>
</template>

<script setup>
definePageMeta({
    middleware: 'admin',
    layout: 'admin'
});

const route = useRoute();
const { authFetch } = useAuthFetch();

const id = computed(() => route.params.id);

const form = reactive({
    currency_code: '',
    symbol: '',
    decimal_places: '',
    value: ''
});
const loading = ref(true);
const loadError = ref('');
const saving = ref(false);
const error = ref('');
const saved = ref(false);

const canSubmit = computed(() =>
    form.currency_code.trim() && form.symbol.trim() && String(form.value).trim() !== ''
);

async function fetchCurrency() {
    loading.value = true;
    loadError.value = '';
    try {
        const data = await authFetch(`/admin/currencies/${id.value}`, {
            method: 'GET'
        });
        const currency = data?.currency || data?.data || data;

        form.currency_code = currency.currency_code ?? '';
        form.symbol = currency.symbol ?? '';
        form.decimal_places = currency.decimal_places ?? '';
        form.value = currency.value ?? '';
    } catch (e) {
        loadError.value = e.statusMessage || 'Failed to load currency.';
    } finally {
        loading.value = false;
    }
}

async function submit() {
    if (!canSubmit.value) return;

    saving.value = true;
    error.value = '';
    saved.value = false;
    try {
        const body = {
            currency_code: form.currency_code.trim().toUpperCase(),
            symbol: form.symbol.trim(),
            value: String(form.value)
        };
        if (String(form.decimal_places).trim() !== '') {
            body.decimal_places = Number(form.decimal_places);
        }

        await authFetch(`/admin/currencies/${id.value}`, {
            method: 'PATCH',
            body
        });

        saved.value = true;
        await navigateTo('/dashboard/currencies');
    } catch (e) {
        error.value = e.statusMessage || 'Failed to update currency.';
    } finally {
        saving.value = false;
    }
}

onMounted(fetchCurrency);

useHead({
    title: 'Edit currency - webedit99'
});
</script>