<template>
    <div class="mx-auto flex max-w-xl flex-col gap-4">
        <div class="flex items-center justify-between">
            <h1 class="text-xl font-semibold text-gray-900">Create customer</h1>
            <NuxtLink :to="`/stores/${uuid}/customers`"
                class="rounded-lg border border-gray-300 bg-white px-3 py-1.5 text-sm font-medium hover:bg-gray-50">
                Back
            </NuxtLink>
        </div>

        <form class="flex flex-col gap-4" @submit.prevent="submit">
            <div class="rounded-xl border border-gray-300 bg-white p-4 shadow-sm">
                <label for="name" class="mb-1 block text-sm font-medium">Name</label>
                <StoreInput id="name" v-model="form.name" type="text" />

                <label for="username" class="mb-1 mt-3 block text-sm font-medium">Username</label>
                <StoreInput id="username" v-model="form.username" type="text" />

                <label for="email" class="mb-1 mt-3 block text-sm font-medium">Email</label>
                <StoreInput id="email" v-model="form.email" type="email" />

                <label for="password" class="mb-1 mt-3 block text-sm font-medium">Password</label>
                <StoreInput id="password" v-model="form.password" type="password" placeholder="At least 8 characters" />

                <label for="phone" class="mb-1 mt-3 block text-sm font-medium">Phone</label>
                <StoreInput id="phone" v-model="form.phone" type="text" />

                <div class="mt-4 flex flex-col gap-2 text-sm">
                    <label class="flex items-center gap-2">
                        <input v-model="form.is_active" type="checkbox" class="h-4 w-4" />
                        Active
                    </label>
                    <label class="flex items-center gap-2">
                        <input v-model="form.is_verified" type="checkbox" class="h-4 w-4" />
                        Mark email as verified
                    </label>
                </div>
            </div>

            <CardsError v-if="error" :text="error" />

            <div class="flex justify-end gap-2">
                <NuxtLink :to="`/stores/${uuid}/customers`"
                    class="rounded-lg border border-gray-300 bg-white px-4 py-2 text-sm font-medium hover:bg-gray-50">
                    Cancel
                </NuxtLink>
                <button type="submit" :disabled="saving || !canSubmit"
                    class="rounded-lg bg-gray-900 px-4 py-2 text-sm font-medium text-white hover:bg-gray-800 disabled:opacity-50">
                    {{ saving ? 'Creating...' : 'Create customer' }}
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
    name: '',
    username: '',
    email: '',
    password: '',
    phone: '',
    is_active: true,
    is_verified: false
});

const saving = ref(false);
const error = ref('');

const canSubmit = computed(
    () =>
        form.name.trim() &&
        form.username.trim() &&
        form.email.trim() &&
        form.password.length >= 8
);

async function submit() {
    if (!canSubmit.value) return;

    saving.value = true;
    error.value = '';
    try {
        await authFetch(`/system/stores/${uuid.value}/customers`, {
            method: 'POST',
            body: {
                name: form.name.trim(),
                username: form.username.trim(),
                email: form.email.trim(),
                password: form.password,
                phone: form.phone.trim() || null,
                is_active: form.is_active,
                is_verified: form.is_verified
            }
        });

        await navigateTo(`/stores/${uuid.value}/customers`);
    } catch (e) {
        error.value = e.statusMessage || 'Failed to create customer.';
    } finally {
        saving.value = false;
    }
}

useHead({
    title: 'Create customer - webedit99'
});
</script>