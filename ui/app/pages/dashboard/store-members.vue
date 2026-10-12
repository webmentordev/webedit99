<template>
    <div class="min-h-screen bg-yellow-50 font-mono text-black p-6">
        <div class="max-w-6xl mx-auto">
            <div class="flex items-center justify-between gap-4">
                <h1 class="text-2xl font-bold bg-black text-yellow-300 px-3 py-1 inline-block">store_members_</h1>
                <button type="button" @click="fetchMembers" :disabled="loading"
                    class="border-2 border-black bg-white px-4 py-2 text-sm font-bold shadow-[3px_3px_0_0_#000] hover:translate-x-px hover:translate-y-px hover:shadow-[2px_2px_0_0_#000] transition-all disabled:opacity-50">
                    {{ loading ? 'Loading...' : 'Refresh' }}
                </button>
            </div>

            <div class="mt-6 border-4 border-black bg-white shadow-[6px_6px_0_0_#000]">
                <div class="border-b-4 border-black bg-yellow-100 p-3">
                    <input v-model="search" type="text" placeholder="Search store members..."
                        class="w-full border-2 border-black bg-white px-3 py-2 text-sm font-bold placeholder:text-gray-500 focus:outline-none focus:bg-yellow-50" />
                </div>

                <div v-if="loading" class="p-10 text-center text-sm font-bold">Loading store members...</div>

                <div v-else-if="error" class="p-6 text-center">
                    <p class="inline-block border-2 border-black bg-red-300 px-3 py-2 text-sm font-bold">{{ error }}</p>
                    <div>
                        <button type="button" @click="fetchMembers"
                            class="mt-4 border-2 border-black bg-white px-4 py-2 text-sm font-bold hover:bg-yellow-100">
                            Try again
                        </button>
                    </div>
                </div>

                <div v-else-if="members.length === 0" class="p-10 text-center">
                    <p class="font-bold">No store members yet</p>
                </div>

                <div v-else-if="filtered.length === 0" class="p-10 text-center">
                    <p class="font-bold">No results found</p>
                    <p class="mt-1 text-sm">Try a different search term.</p>
                </div>

                <div v-else class="overflow-x-auto">
                    <table class="w-full text-sm">
                        <thead>
                            <tr class="border-b-4 border-black bg-yellow-300 text-left">
                                <th class="px-4 py-2 font-bold">Store</th>
                                <th class="px-4 py-2 font-bold border-l-2 border-black">Name</th>
                                <th class="px-4 py-2 font-bold border-l-2 border-black">Email</th>
                                <th class="px-4 py-2 font-bold border-l-2 border-black">Role</th>
                                <th class="px-4 py-2 font-bold border-l-2 border-black">Joined</th>
                                <th class="px-4 py-2 font-bold border-l-2 border-black">Updated</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr v-for="member in filtered" :key="member.id"
                                class="border-b-2 border-black last:border-b-0 hover:bg-yellow-100">
                                <td class="px-4 py-2">
                                    <p class="font-bold">{{ member.store_title }}</p>
                                    <span
                                        class="mt-1 inline-block border-2 border-black bg-black px-2 py-0.5 text-xs font-bold text-yellow-300">
                                        {{ member.store_slug }}
                                    </span>
                                </td>
                                <td class="px-4 py-2 border-l-2 border-black font-bold">{{ member.name }}</td>
                                <td class="px-4 py-2 border-l-2 border-black">{{ member.email }}</td>
                                <td class="px-4 py-2 border-l-2 border-black">
                                    <div class="flex flex-wrap gap-1">
                                        <span v-if="member.is_owner"
                                            class="border-2 border-black bg-black px-2 py-0.5 text-xs font-bold text-yellow-300">
                                            Owner
                                        </span>
                                        <span v-if="member.is_worker"
                                            class="border-2 border-black bg-green-300 px-2 py-0.5 text-xs font-bold">
                                            Worker
                                        </span>
                                        <span v-if="!member.is_owner && !member.is_worker"
                                            class="border-2 border-black bg-gray-300 px-2 py-0.5 text-xs font-bold">
                                            Member
                                        </span>
                                    </div>
                                </td>
                                <td class="px-4 py-2 border-l-2 border-black whitespace-nowrap">
                                    {{ formatDate(member.created_at) }}
                                </td>
                                <td class="px-4 py-2 border-l-2 border-black whitespace-nowrap">
                                    {{ formatDate(member.updated_at) }}
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>

                <div v-if="!loading && !error && members.length > 0"
                    class="border-t-4 border-black bg-yellow-50 px-4 py-2 text-xs font-bold">
                    Showing {{ filtered.length }} of {{ members.length }} store members
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

const members = ref([]);
const search = ref('');
const loading = ref(true);
const error = ref('');

const filtered = computed(() => {
    const q = search.value.trim().toLowerCase();
    if (!q) return members.value;
    return members.value.filter((member) =>
        [member.store_title, member.store_slug, member.name, member.email]
            .filter(Boolean)
            .some((value) => String(value).toLowerCase().includes(q))
    );
});

function formatDate(value) {
    if (!value) return '-';
    return new Date(value).toLocaleString();
}

async function fetchMembers() {
    loading.value = true;
    error.value = '';
    try {
        const data = await authFetch('/admin/store-members', {
            method: 'GET'
        });
        if (Array.isArray(data)) {
            members.value = data;
        } else {
            members.value = data?.store_members || data?.members || data?.data || [];
        }
    } catch (e) {
        error.value = e.statusMessage || 'Something went wrong!';
    } finally {
        loading.value = false;
    }
}

onMounted(fetchMembers);

useHead({
    title: 'Store members - webedit99'
});
</script>