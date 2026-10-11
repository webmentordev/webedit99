export default defineNuxtRouteMiddleware(async () => {
    const { getToken } = useAuthToken();
    const { getUser } = useAuthUser();
    const token = getToken();
    const user = getUser();

    if (!token || !user) {
        return await navigateTo('/login');
    }
});