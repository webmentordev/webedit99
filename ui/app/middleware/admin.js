export default defineNuxtRouteMiddleware(async () => {
    const { getUser } = useAuthUser();
    const user = getUser();

    if (!user || user.is_admin !== true) {
        return await navigateTo('/stores');
    }
});