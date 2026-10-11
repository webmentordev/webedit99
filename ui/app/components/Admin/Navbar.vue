<template>
    <aside
        class="w-64 shrink-0 h-screen overflow-y-auto border-r-4 border-black bg-white font-mono text-black flex flex-col">
        <div class="border-b-4 border-black bg-yellow-50 px-4 py-4">
            <NuxtLink to="/dashboard/dashboard"
                class="inline-block text-lg font-bold bg-black text-yellow-300 px-3 py-1">
                webedit99_
            </NuxtLink>
        </div>

        <nav class="flex-1 px-3 py-4 flex flex-col gap-5">
            <div v-for="group in groups" :key="group.title">
                <button type="button" @click="toggle(group.title)"
                    class="w-full flex items-center justify-between border-2 border-black px-3 py-2 text-sm font-bold shadow-[3px_3px_0_0_#000] hover:translate-x-px hover:translate-y-px hover:shadow-[2px_2px_0_0_#000] transition-all"
                    :class="group.color">
                    <span>{{ group.title }}</span>
                    <span>{{ isOpen(group.title) ? '[-]' : '[+]' }}</span>
                </button>

                <ul v-show="isOpen(group.title)" class="mt-3 flex flex-col gap-1 border-l-4 border-black ml-2 pl-2">
                    <li v-for="item in group.items" :key="item.to">
                        <NuxtLink :to="item.to"
                            class="block px-3 py-1.5 text-sm font-bold border-2 border-transparent hover:border-black hover:bg-yellow-100"
                            :class="isActive(item.to) ? 'border-black bg-yellow-300' : ''">
                            {{ item.label }}
                        </NuxtLink>
                    </li>
                </ul>
            </div>
        </nav>

        <div class="border-t-4 border-black bg-yellow-50 p-3 flex flex-col gap-2">
            <NuxtLink to="/"
                class="border-2 border-black bg-white px-3 py-2 text-sm font-bold text-center hover:bg-yellow-100">
                View store
            </NuxtLink>
            <NuxtLink to="/status"
                class="border-2 border-black bg-white px-3 py-2 text-sm font-bold text-center hover:bg-yellow-100">
                System status
            </NuxtLink>
            <button @click="logout"
                class="bg-red-300 border-2 border-black px-3 py-2 text-sm font-bold text-center hover:bg-red-100">Logout</button>
        </div>
    </aside>
</template>

<script setup>
const route = useRoute();

const { logout } = useLogout();

const groups = [
    {
        title: 'General',
        color: 'bg-sky-300',
        items: [
            { label: 'Dashboard', to: '/dashboard' },
            { label: 'Stores', to: '/dashboard/stores' },
            { label: 'Store Members', to: '/dashboard/store-members' },
            { label: 'Permissions', to: '/dashboard/store-member-permissions' },
            { label: 'Currencies', to: '/dashboard/currencies' },
            { label: 'Store Logs', to: '/dashboard/store-logs' }
        ]
    },
    {
        title: 'Catalog',
        color: 'bg-yellow-300',
        items: [
            { label: 'Products', to: '/dashboard/products' },
            { label: 'Variants', to: '/dashboard/product-variants' },
            { label: 'Sub Variants', to: '/dashboard/product-sub-variants' },
            { label: 'Specifications', to: '/dashboard/product-specifications' },
            { label: 'Extra Fees', to: '/dashboard/product-extra-fees' },
            { label: 'Brands', to: '/dashboard/brands' },
            { label: 'Collections', to: '/dashboard/collections' },
            { label: 'Sub Collections', to: '/dashboard/sub-collections' },
            { label: 'Digital Files', to: '/dashboard/digital-files' },
            { label: 'Reviews', to: '/dashboard/product-reviews' },
            { label: 'Inventory', to: '/dashboard/inventory-movements' }
        ]
    },
    {
        title: 'Orders',
        color: 'bg-pink-300',
        items: [
            { label: 'Orders', to: '/dashboard/orders' },
            { label: 'Payments', to: '/dashboard/payments' },
            { label: 'Shipments', to: '/dashboard/shipments' },
            { label: 'Shipping Methods', to: '/dashboard/shipping-methods' },
            { label: 'Tax Rates', to: '/dashboard/tax-rates' },
            { label: 'Carts', to: '/dashboard/carts' },
            { label: 'Wishlists', to: '/dashboard/wishlists' }
        ]
    },
    {
        title: 'Customers',
        color: 'bg-green-300',
        items: [
            { label: 'Customers', to: '/dashboard/customers' },
            { label: 'Addresses', to: '/dashboard/customer-addresses' },
            { label: 'Messages', to: '/dashboard/contact-messages' }
        ]
    },
    {
        title: 'Marketing',
        color: 'bg-orange-300',
        items: [
            { label: 'Coupons', to: '/dashboard/coupons' },
            { label: 'Coupon Usages', to: '/dashboard/coupon-usages' }
        ]
    },
    {
        title: 'Content',
        color: 'bg-purple-300',
        items: [
            { label: 'Blog Posts', to: '/dashboard/blog-posts' },
            { label: 'Blog Categories', to: '/dashboard/blog-categories' },
            { label: 'Pages', to: '/dashboard/pages' },
            { label: 'Media Files', to: '/dashboard/media-files' },
            { label: 'Navigation Menus', to: '/dashboard/navigation-menus' },
            { label: 'Redirects', to: '/dashboard/redirects' }
        ]
    },
    {
        title: 'Online Store',
        color: 'bg-red-300',
        items: [
            { label: 'Themes', to: '/dashboard/themes' },
            { label: 'Code Editor', to: '/dashboard/editor' }
        ]
    },
    {
        title: 'Accounts',
        color: 'bg-sky-200',
        items: [
            { label: 'Users', to: '/dashboard/users' },
            { label: 'User Tokens', to: '/dashboard/user-tokens' },
            { label: 'Customer Tokens', to: '/dashboard/customer-tokens' }
        ]
    }
];

const open = ref({});

groups.forEach((group) => {
    open.value[group.title] = group.items.some((item) => route.path.startsWith(item.to));
});

if (!Object.values(open.value).some(Boolean)) {
    open.value[groups[0].title] = true;
}

function isOpen(title) {
    return !!open.value[title];
}

function toggle(title) {
    open.value[title] = !open.value[title];
}

function isActive(to) {
    return route.path === to || route.path.startsWith(to + '/');
}
</script>