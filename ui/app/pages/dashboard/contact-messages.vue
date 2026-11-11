<template>
    <div class="min-h-screen bg-yellow-50 font-mono text-black p-6">
        <div class="max-w-6xl mx-auto">
            <div class="flex items-center justify-between gap-4">
                <h1 class="text-2xl font-bold bg-black text-yellow-300 px-3 py-1 inline-block">contact_messages_</h1>
                <button type="button" @click="fetchMessages" :disabled="loading"
                    class="border-2 border-black bg-white px-4 py-2 text-sm font-bold shadow-[3px_3px_0_0_#000] hover:translate-x-px hover:translate-y-px hover:shadow-[2px_2px_0_0_#000] transition-all disabled:opacity-50">
                    {{ loading ? 'Loading...' : 'Refresh' }}
                </button>
            </div>

            <div class="mt-6 border-4 border-black bg-white shadow-[6px_6px_0_0_#000]">
                <div class="border-b-4 border-black bg-yellow-100 p-3">
                    <input v-model="search" type="text" placeholder="Search messages..."
                        class="w-full border-2 border-black bg-white px-3 py-2 text-sm font-bold placeholder:text-gray-500 focus:outline-none focus:bg-yellow-50" />
                </div>

                <div v-if="loading" class="p-10 text-center text-sm font-bold">Loading messages...</div>

                <div v-else-if="error" class="p-6 text-center">
                    <p class="inline-block border-2 border-black bg-red-300 px-3 py-2 text-sm font-bold">{{ error }}</p>
                    <div>
                        <button type="button" @click="fetchMessages"
                            class="mt-4 border-2 border-black bg-white px-4 py-2 text-sm font-bold hover:bg-yellow-100">
                            Try again
                        </button>
                    </div>
                </div>

                <div v-else-if="messages.length === 0" class="p-10 text-center">
                    <p class="font-bold">No messages yet</p>
                </div>

                <div v-else-if="filtered.length === 0" class="p-10 text-center">
                    <p class="font-bold">No results found</p>
                    <p class="mt-1 text-sm">Try a different search term.</p>
                </div>

                <div v-else class="overflow-x-auto">
                    <table class="w-full text-sm">
                        <thead>
                            <tr class="border-b-4 border-black bg-yellow-300 text-left">
                                <th class="px-4 py-2 font-bold">Sender</th>
                                <th class="px-4 py-2 font-bold border-l-2 border-black">Store</th>
                                <th class="px-4 py-2 font-bold border-l-2 border-black">Message</th>
                                <th class="px-4 py-2 font-bold border-l-2 border-black">Status</th>
                                <th class="px-4 py-2 font-bold border-l-2 border-black">Received</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr v-for="message in filtered" :key="message.id"
                                class="border-b-2 border-black last:border-b-0 hover:bg-yellow-100">
                                <td class="px-4 py-2">
                                    <p class="font-bold">{{ message.name }}</p>
                                    <p class="text-xs">{{ message.email }}</p>
                                    <p v-if="message.phone" class="text-xs">{{ message.phone }}</p>
                                    <p class="text-xs">{{ message.ip_address }}</p>
                                </td>
                                <td class="px-4 py-2 border-l-2 border-black">
                                    <p class="font-bold">{{ message.store_title }}</p>
                                    <p class="text-xs">{{ message.store_slug }}</p>
                                </td>
                                <td class="px-4 py-2 border-l-2 border-black max-w-md">
                                    <p v-if="message.subject" class="font-bold">{{ message.subject }}</p>
                                    <p class="whitespace-pre-wrap break-words">{{ message.message }}</p>
                                </td>
                                <td class="px-4 py-2 border-l-2 border-black">
                                    <span class="border-2 border-black px-2 py-0.5 text-xs font-bold"
                                        :class="message.is_read ? 'bg-gray-300' : 'bg-green-300'">
                                        {{ message.is_read ? 'Read' : 'Unread' }}
                                    </span>
                                </td>
                                <td class="px-4 py-2 border-l-2 border-black whitespace-nowrap">
                                    {{ formatDate(message.created_at) }}
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>

                <div v-if="!loading && !error && messages.length > 0"
                    class="border-t-4 border-black bg-yellow-50 px-4 py-2 text-xs font-bold">
                    Showing {{ filtered.length }} of {{ messages.length }} messages
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

const messages = ref([]);
const search = ref('');
const loading = ref(true);
const error = ref('');

const filtered = computed(() => {
    const q = search.value.trim().toLowerCase();
    if (!q) return messages.value;
    return messages.value.filter((message) =>
        [
            message.name,
            message.email,
            message.phone,
            message.subject,
            message.message,
            message.ip_address,
            message.store_title,
            message.store_slug
        ]
            .filter(Boolean)
            .some((value) => String(value).toLowerCase().includes(q))
    );
});

function formatDate(value) {
    if (!value) return '-';
    return new Date(value).toLocaleString();
}

async function fetchMessages() {
    loading.value = true;
    error.value = '';
    try {
        const data = await authFetch('/admin/contact-messages', {
            method: 'GET'
        });
        if (Array.isArray(data)) {
            messages.value = data;
        } else {
            messages.value = data?.messages || data?.data || [];
        }
    } catch (e) {
        error.value = e.statusMessage || 'Something went wrong!';
    } finally {
        loading.value = false;
    }
}

onMounted(fetchMessages);

useHead({
    title: 'Contact Messages - webedit99'
});
</script>