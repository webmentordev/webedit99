<template>
    <div class="min-h-screen bg-yellow-50 font-mono text-black p-6">
        <div class="max-w-xl mx-auto">
            <div class="flex items-center justify-between gap-4">
                <h1 class="text-2xl font-bold bg-black text-yellow-300 px-3 py-1 inline-block">new_currency_</h1>
                <NuxtLink to="/dashboard/currencies"
                    class="border-2 border-black bg-white px-4 py-2 text-sm font-bold shadow-[3px_3px_0_0_#000] hover:translate-x-px hover:translate-y-px hover:shadow-[2px_2px_0_0_#000] transition-all">
                    Back
                </NuxtLink>
            </div>

            <form class="mt-6 border-4 border-black bg-white shadow-[6px_6px_0_0_#000]" @submit.prevent="submit">
                <div class="flex flex-col gap-4 p-4">
                    <div>
                        <label for="currency_code" class="mb-1 block text-sm font-bold">Currency code</label>
                        <input id="currency_code" v-model="form.currency_code" type="text" placeholder="USD"
                            maxlength="10" required
                            class="w-full border-2 border-black bg-white px-3 py-2 text-sm font-bold uppercase placeholder:text-gray-500 focus:outline-none focus:bg-yellow-50" />
                    </div>

                    <div>
                        <label for="symbol" class="mb-1 block text-sm font-bold">Symbol</label>
                        <input id="symbol" v-model="form.symbol" type="text" placeholder="$" required
                            class="w-full border-2 border-black bg-white px-3 py-2 text-sm font-bold placeholder:text-gray-500 focus:outline-none focus:bg-yellow-50" />
                    </div>

                    <div>
                        <label for="decimal_places" class="mb-1 block text-sm font-bold">
                            Decimal places <span class="font-normal text-gray-600">(optional)</span>
                        </label>
                        <input id="decimal_places" v-model="form.decimal_places" type="number" min="0" max="10" step="1"
                            placeholder="2"
                            class="w-full border-2 border-black bg-white px-3 py-2 text-sm font-bold placeholder:text-gray-500 focus:outline-none focus:bg-yellow-50" />
                    </div>

                    <div>
                        <label for="value" class="mb-1 block text-sm font-bold">Value</label>
                        <input id="value" v-model="form.value" type="number" step="any" min="0" placeholder="1.00"
                            required
                            class="w-full border-2 border-black bg-white px-3 py-2 text-sm font-bold placeholder:text-gray-500 focus:outline-none focus:bg-yellow-50" />
                    </div>

                    <p v-if="error" class="border-2 border-black bg-red-300 px-3 py-2 text-sm font-bold">{{ error }}</p>
                </div>

                <div class="flex justify-end gap-2 border-t-4 border-black bg-yellow-100 p-3">
                    <NuxtLink to="/dashboard/currencies"
                        class="border-2 border-black bg-white px-4 py-2 text-sm font-bold hover:bg-yellow-50">
                        Cancel
                    </NuxtLink>
                    <button type="submit" :disabled="saving || !canSubmit"
                        class="border-2 border-black bg-yellow-300 px-4 py-2 text-sm font-bold shadow-[3px_3px_0_0_#000] hover:translate-x-px hover:translate-y-px hover:shadow-[2px_2px_0_0_#000] transition-all disabled:opacity-50">
                        {{ saving ? 'Creating...' : 'Create currency' }}
                    </button>
                </div>
            </form>
        </div>
    </div>
</template>

<script setup>
definePageMeta({
    middleware: 'admin',
    layout: 'admin'
});

const { authFetch } = useAuthFetch();

const form = reactive({
    currency_code: '',
    symbol: '',
    decimal_places: '',
    value: ''
});
const saving = ref(false);
const error = ref('');

const canSubmit = computed(() =>
    form.currency_code.trim() && form.symbol.trim() && String(form.value).trim() !== ''
);

async function submit() {
    if (!canSubmit.value) return;

    saving.value = true;
    error.value = '';
    try {
        const body = {
            currency_code: form.currency_code.trim().toUpperCase(),
            symbol: form.symbol.trim(),
            value: String(form.value)
        };
        if (String(form.decimal_places).trim() !== '') {
            body.decimal_places = Number(form.decimal_places);
        }

        await authFetch('/admin/currencies', {
            method: 'POST',
            body
        });

        await navigateTo('/dashboard/currencies');
    } catch (e) {
        error.value = e.statusMessage || 'Failed to create currency.';
    } finally {
        saving.value = false;
    }
}

useHead({
    title: 'New currency - webedit99'
});
</script>