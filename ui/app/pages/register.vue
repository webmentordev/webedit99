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
                    <span class="ml-3 text-sm font-bold">register.exe</span>
                </div>

                <form @submit.prevent="register" method="post" class="p-6">
                    <div class="grid grid-cols-1 gap-4">
                        <div class="flex flex-col">
                            <label for="name" class="text-sm font-bold mb-1">Name</label>
                            <Input id="name" v-model="name" type="text" placeholder="Name" />
                            <p v-if="errors.name"
                                class="mt-2 border-2 border-black bg-red-400 px-3 py-1 text-xs font-bold">
                                {{ errors.name }}
                            </p>
                        </div>
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
                        <div class="flex flex-col mb-3">
                            <label for="confirm_password" class="text-sm font-bold mb-1">Confirm password</label>
                            <Input id="confirm_password" v-model="confirm_password" type="password"
                                placeholder="Confirm password" />
                            <p v-if="errors.confirm_password"
                                class="mt-2 border-2 border-black bg-red-400 px-3 py-1 text-xs font-bold">
                                {{ errors.confirm_password }}
                            </p>
                        </div>
                    </div>

                    <div class="my-5 m-auto w-fit">
                        <NuxtTurnstile ref="turnstile" v-model="ct_token" />
                    </div>

                    <button type="submit" :disabled="processing"
                        class="w-full border-2 border-black inline-block bg-pink-300 px-6 py-3 font-bold shadow-[4px_4px_0_0_#000] hover:translate-x-0.5 hover:translate-y-0.5 hover:shadow-[2px_2px_0_0_#000] transition-all disabled:opacity-60 disabled:cursor-not-allowed mt-6">
                        {{ processing ? 'Processing signup request...' : 'Register &rarr;' }}
                    </button>

                    <div class="mt-5 flex flex-col gap-2 text-sm">
                        <p>
                            Already have an account?
                            <NuxtLink to="/login" class="font-bold underline">Login here</NuxtLink>
                        </p>
                    </div>

                    <p class="mt-5 pt-6 border-t-4 border-black text-sm">
                        By continuing, you agree to our
                        <NuxtLink to="/terms-of-service" class="font-bold underline">Terms of service</NuxtLink>
                        &amp;
                        <NuxtLink to="/privacy-policy" class="font-bold underline">Privacy policy</NuxtLink>
                    </p>

                    <div class="mt-5 grid gap-4">
                        <div v-if="message">
                            <AlertsSuccess :text="message" />
                            <p class="mt-3 text-sm">
                                Didn't get the email?
                                <NuxtLink to="/verify-email" class="font-bold underline">Resend verification link
                                </NuxtLink>
                            </p>
                        </div>
                        <AlertsError v-if="errors.message" :text="errors.message" />
                    </div>
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

const name = ref("");
const email = ref("");
const password = ref("");
const confirm_password = ref("");
const processing = ref(false);
const message = ref("");
const ct_token = ref("");
const turnstile = ref(null);
const errors = ref({
    count: 0
});

async function register() {
    processing.value = true;
    message.value = "";
    reset_errors();
    if (name.value.trim() == "") {
        errors.value.name = "Name is required";
        errors.value.count += 1;
    }
    if (email.value.trim() == "") {
        errors.value.email = "Email is required";
        errors.value.count += 1;
    }
    if (password.value == "") {
        errors.value.password = "Password is required";
        errors.value.count += 1;
    } else if (password.value.length < 8) {
        errors.value.password = "Password must be at least 8 characters";
        errors.value.count += 1;
    } else if (password.value.length > 128) {
        errors.value.password = "Password must be at most 128 characters";
        errors.value.count += 1;
    }
    if (confirm_password.value == "") {
        errors.value.confirm_password = "Password confirmation is required";
        errors.value.count += 1;
    } else if (password.value != confirm_password.value) {
        errors.value.confirm_password = "Passwords do not match";
        errors.value.count += 1;
    }
    if (errors.value.count > 0) {
        processing.value = false;
        return;
    }
    try {
        const data = await authFetch('/api/account/register', {
            method: "POST",
            body: {
                name: name.value.trim(),
                email: email.value.trim(),
                password: password.value,
                confirm_password: confirm_password.value,
                ct_token: ct_token.value
            }
        });
        if (data) {
            message.value = "Account created! We've sent a verification link to your email address. Please check your inbox.";
            reset_values();
        }
    } catch (e) {
        if (e.statusCode === 409 || e.status === 409) {
            errors.value.message = 'An account with this email already exists.';
        } else {
            errors.value.message = e.statusMessage || 'Something went wrong!';
        }
    } finally {
        processing.value = false;
        turnstile.value?.reset();
    }
}

function reset_errors() {
    errors.value = {
        count: 0
    };
}

function reset_values() {
    name.value = "";
    email.value = "";
    password.value = "";
    confirm_password.value = "";
}
</script>