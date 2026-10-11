<template>
    <aside
        class="w-64 shrink-0 h-screen overflow-y-auto bg-gray-100 border-r border-gray-300 text-gray-900 flex flex-col">
        <div class="px-4 py-3 border-b border-gray-300 flex items-center gap-3">
            <div class="w-8 h-8 rounded-lg bg-gray-900 text-white flex items-center justify-center text-sm font-bold">w
            </div>
            <div class="min-w-0">
                <p class="text-sm font-semibold leading-tight">webedit99</p>
                <p class="text-xs text-gray-500 truncate">{{ uuid }}</p>
            </div>
        </div>

        <nav class="flex-1 px-2 py-3 flex flex-col gap-0.5 text-sm">
            <template v-for="item in main" :key="item.label">
                <NuxtLink v-if="!item.children" :to="item.to"
                    class="block rounded-lg px-3 py-1.5 font-medium hover:bg-gray-200"
                    :class="isActive(item.to, item.exact) ? 'bg-white shadow-sm' : ''">
                    {{ item.label }}
                </NuxtLink>

                <div v-else>
                    <button type="button" @click="toggle(item.label)"
                        class="w-full flex items-center justify-between rounded-lg px-3 py-1.5 font-medium hover:bg-gray-200"
                        :class="isGroupActive(item) ? 'bg-white shadow-sm' : ''">
                        <span>{{ item.label }}</span>
                        <span class="text-xs text-gray-500 transition-transform"
                            :class="isOpen(item.label) ? 'rotate-90' : ''">&#9656;</span>
                    </button>
                    <ul v-show="isOpen(item.label)"
                        class="mt-0.5 mb-1 ml-3 pl-3 border-l border-gray-300 flex flex-col gap-0.5">
                        <li v-for="child in item.children" :key="child.to">
                            <NuxtLink :to="child.to"
                                class="block rounded-lg px-3 py-1.5 text-gray-700 hover:bg-gray-200"
                                :class="isActive(child.to) ? 'bg-white shadow-sm font-medium text-gray-900' : ''">
                                {{ child.label }}
                            </NuxtLink>
                        </li>
                    </ul>
                </div>
            </template>

            <p class="mt-5 mb-1 px-3 text-xs font-semibold uppercase tracking-wide text-gray-500">Sales channels</p>
            <template v-for="item in channels" :key="item.label">
                <div>
                    <button type="button" @click="toggle(item.label)"
                        class="w-full flex items-center justify-between rounded-lg px-3 py-1.5 font-medium hover:bg-gray-200"
                        :class="isGroupActive(item) ? 'bg-white shadow-sm' : ''">
                        <span>{{ item.label }}</span>
                        <span class="text-xs text-gray-500 transition-transform"
                            :class="isOpen(item.label) ? 'rotate-90' : ''">&#9656;</span>
                    </button>
                    <ul v-show="isOpen(item.label)"
                        class="mt-0.5 mb-1 ml-3 pl-3 border-l border-gray-300 flex flex-col gap-0.5">
                        <li v-for="child in item.children" :key="child.to">
                            <NuxtLink :to="child.to"
                                class="block rounded-lg px-3 py-1.5 text-gray-700 hover:bg-gray-200"
                                :class="isActive(child.to) ? 'bg-white shadow-sm font-medium text-gray-900' : ''">
                                {{ child.label }}
                            </NuxtLink>
                        </li>
                    </ul>
                </div>
            </template>
        </nav>

        <div class="border-t border-gray-300 px-2 py-3 flex flex-col gap-0.5 text-sm">
            <div>
                <button type="button" @click="toggle(settings.label)"
                    class="w-full flex items-center justify-between rounded-lg px-3 py-1.5 font-medium hover:bg-gray-200"
                    :class="isGroupActive(settings) ? 'bg-white shadow-sm' : ''">
                    <span>{{ settings.label }}</span>
                    <span class="text-xs text-gray-500 transition-transform"
                        :class="isOpen(settings.label) ? 'rotate-90' : ''">&#9656;</span>
                </button>
                <ul v-show="isOpen(settings.label)"
                    class="mt-0.5 mb-1 ml-3 pl-3 border-l border-gray-300 flex flex-col gap-0.5">
                    <li v-for="child in settings.children" :key="child.to">
                        <NuxtLink :to="child.to" class="block rounded-lg px-3 py-1.5 text-gray-700 hover:bg-gray-200"
                            :class="isActive(child.to) ? 'bg-white shadow-sm font-medium text-gray-900' : ''">
                            {{ child.label }}
                        </NuxtLink>
                    </li>
                </ul>
            </div>
            <NuxtLink to="/stores" class="rounded-lg px-3 py-1.5 text-gray-700 hover:bg-gray-200">All stores</NuxtLink>
            <NuxtLink to="/status" class="rounded-lg px-3 py-1.5 text-gray-700 hover:bg-gray-200">System status
            </NuxtLink>
            <button type="button" @click="logout"
                class="w-full text-left rounded-lg px-3 py-1.5 font-medium text-red-700 hover:bg-red-100">
                Log out
            </button>
        </div>
    </aside>
</template>

<script setup>
const route = useRoute();

const { logout } = useLogout();

const uuid = computed(() => route.params.uuid);
const base = computed(() => `/stores/${uuid.value}`);

const main = computed(() => [
    { label: 'Home', to: base.value, exact: true },
    {
        label: 'Orders',
        children: [
            { label: 'All orders', to: `${base.value}/orders` },
            { label: 'Payments', to: `${base.value}/payments` },
            { label: 'Shipments', to: `${base.value}/shipments` },
            { label: 'Abandoned carts', to: `${base.value}/carts` },
            { label: 'Wishlists', to: `${base.value}/wishlists` }
        ]
    },
    {
        label: 'Products',
        children: [
            { label: 'All products', to: `${base.value}/products` },
            { label: 'Variants', to: `${base.value}/product-variants` },
            { label: 'Sub variants', to: `${base.value}/product-sub-variants` },
            { label: 'Specifications', to: `${base.value}/product-specifications` },
            { label: 'Extra fees', to: `${base.value}/product-extra-fees` },
            { label: 'Collections', to: `${base.value}/collections` },
            { label: 'Sub collections', to: `${base.value}/sub-collections` },
            { label: 'Brands', to: `${base.value}/brands` },
            { label: 'Digital files', to: `${base.value}/digital-files` },
            { label: 'Inventory', to: `${base.value}/inventory-movements` },
            { label: 'Reviews', to: `${base.value}/product-reviews` }
        ]
    },
    {
        label: 'Customers',
        children: [
            { label: 'All customers', to: `${base.value}/customers` },
            { label: 'Addresses', to: `${base.value}/customer-addresses` },
            { label: 'Messages', to: `${base.value}/contact-messages` }
        ]
    },
    {
        label: 'Discounts',
        children: [
            { label: 'Coupons', to: `${base.value}/coupons` },
            { label: 'Coupon usages', to: `${base.value}/coupon-usages` }
        ]
    },
    {
        label: 'Content',
        children: [
            { label: 'Pages', to: `${base.value}/pages` },
            { label: 'Files', to: `${base.value}/media-files` },
            { label: 'Menus', to: `${base.value}/navigation-menus` },
            { label: 'Redirects', to: `${base.value}/redirects` }
        ]
    }
]);

const channels = computed(() => [
    {
        label: 'Online Store',
        children: [
            { label: 'Themes', to: `${base.value}/themes` },
            { label: 'Editor', to: `${base.value}/editor` },
            { label: 'Code editor', to: `${base.value}/code-editor` },
            { label: 'Blog posts', to: `${base.value}/blog-posts` },
            { label: 'Blog categories', to: `${base.value}/blog-categories` }
        ]
    }
]);

const settings = computed(() => ({
    label: 'Settings',
    children: [
        { label: 'Store details', to: `${base.value}/settings` },
        { label: 'Staff', to: `${base.value}/store-members` },
        { label: 'Permissions', to: `${base.value}/store-member-permissions` },
        { label: 'Currencies', to: `${base.value}/currencies` },
        { label: 'Shipping', to: `${base.value}/shipping-methods` },
        { label: 'Taxes', to: `${base.value}/tax-rates` },
        { label: 'Activity log', to: `${base.value}/store-logs` }
    ]
}));

const open = ref({});

const all = computed(() => [...main.value, ...channels.value, settings.value]);

watchEffect(() => {
    all.value.forEach((item) => {
        if (item.children && isGroupActive(item)) {
            open.value[item.label] = true;
        }
    });
});

function isOpen(label) {
    return !!open.value[label];
}

function toggle(label) {
    open.value[label] = !open.value[label];
}

function isActive(to, exact = false) {
    if (exact) return route.path === to;
    return route.path === to || route.path.startsWith(to + '/');
}

function isGroupActive(item) {
    return item.children.some((child) => isActive(child.to));
}
</script>