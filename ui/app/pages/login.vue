<template>
    <div class="w-full min-h-[80vh] flex items-center justify-center py-12 bg-yellow-50 font-mono text-black px-4">
        <div class="flex flex-col max-w-md w-full">
            <div class="m-auto mb-6">
                <NuxtLink to="/" class="text-xl font-bold bg-black text-yellow-300 px-3 py-1">webedit99_</NuxtLink>
            </div>

            <div class="border-4 border-black bg-white shadow-[8px_8px_0_0_#000]">
                <div class="flex items-center gap-2 border-b-4 border-black bg-sky-300 px-4 py-2">
                    <span class="w-3 h-3 border-2 border-black bg-red-400 rounded-full"></span>
                    <span class="w-3 h-3 border-2 border-black bg-yellow-300 rounded-full"></span>
                    <span class="w-3 h-3 border-2 border-black bg-green-400 rounded-full"></span>
                    <span class="ml-3 text-sm font-bold">login.exe</span>
                </div>

                <form @submit.prevent="login" method="post" class="p-6">
                    <div class="grid grid-cols-1 gap-4">
                        <div class="flex flex-col">
                            <label for="email" class="text-sm font-bold mb-1">Email</label>
                            <Input id="email" v-model="email" type="email" placeholder="Email address" />
                            <p v-if="errors.email"
                                class="mt-2 border-2 border-black bg-red-400 px-3 py-1 text-xs font-bold">
                                {{ errors.email }}
                            </p>
                        </div>
                        <div class="flex flex-col">
                            <label for="password" class="text-sm font-bold mb-1">Password</label>
                            <Input id="password" v-model="password" type="password" placeholder="Password" />
                            <p v-if="errors.password"
                                class="mt-2 border-2 border-black bg-red-400 px-3 py-1 text-xs font-bold">
                                {{ errors.password }}
                            </p>
                        </div>
                    </div>

                    <button type="submit" :disabled="processing"
                        class="mt-6 w-full border-2 border-black bg-pink-300 px-6 py-3 font-bold shadow-[4px_4px_0_0_#000] hover:translate-x-0.5 hover:translate-y-0.5 hover:shadow-[2px_2px_0_0_#000] transition-all disabled:opacity-60 disabled:cursor-not-allowed">
                        {{ processing ? 'Processing request...' : 'Login &rarr;' }}
                    </button>

                    <div class="mt-6 flex flex-col gap-2 text-sm">
                        <NuxtLink to="/forgot-password" class="font-bold underline">Forgot password?</NuxtLink>
                        <p>
                            Don't have an account?
                            <NuxtLink to="/register" class="font-bold underline">Register here</NuxtLink>
                        </p>
                    </div>

                    <div class="mt-5 grid gap-4">
                        <AlertsSuccess v-if="message" :text="message" />
                        <AlertsError v-if="errors.message" :text="errors.message" />
                    </div>

                    <p v-if="needsVerification" class="mt-4 text-sm">
                        Didn't get the email?
                        <NuxtLink to="/verify-email" class="font-bold underline">Resend verification link</NuxtLink>
                    </p>
                </form>
            </div>
        </div>
    </div>
</template>

<script setup>
definePageMeta({
    middleware: 'guest',
    layout: 'guest'
});
const { authFetch } = useAuthFetch();
const { setToken } = useAuthToken();
const { setUser } = useAuthUser();

const email = ref("");
const password = ref("");
const processing = ref(false);
const message = ref("");
const needsVerification = ref(false);
const errors = ref({
    count: 0
});

async function login() {
    processing.value = true;
    message.value = "";
    needsVerification.value = false;
    reset_errors();
    if (email.value.trim() == "") {
        errors.value.email = "Email is required";
        errors.value.count += 1;
    }
    if (password.value == "") {
        errors.value.password = "Password is required";
        errors.value.count += 1;
    }
    if (errors.value.count > 0) {
        processing.value = false;
        return;
    }
    try {
        const data = await authFetch('/api/account/login', {
            method: "POST",
            body: {
                email: email.value.trim(),
                password: password.value
            }
        });
        if (data) {
            setToken(data.token);
            setUser(data.user);
            if (data.user.is_admin == true) {
                await navigateTo('/admin/dashboard');
            } else {
                await navigateTo('/');
            }
        }
    } catch (e) {
        const code = e.data?.data?.code || e.data?.code;
        if (code === 'email_not_verified') {
            needsVerification.value = true;
        }
        errors.value.message = e.statusMessage || 'Something went wrong!';
    } finally {
        processing.value = false;
    }
}

function reset_errors() {
    errors.value = {
        count: 0
    };
}
</script>