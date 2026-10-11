export const useLogout = () => {
    const { authFetch } = useAuthFetch();
    const { removeToken } = useAuthToken();
    const { removeUser } = useAuthUser();

    const logout = async () => {
        try {
            await authFetch('/account/logout', { method: 'POST' });
        } catch (e) {
        } finally {
            removeToken();
            removeUser();
            await navigateTo('/login');
        }
    };

    return { logout };
};