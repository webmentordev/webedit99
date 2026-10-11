export const useAuthFetch = () => {
    const { getToken, removeToken } = useAuthToken();
    const { removeUser } = useAuthUser();

    const authFetch = async (url, options = {}) => {
        const token = getToken();
        const isFormData = options.body instanceof FormData;

        const defaultOptions = {
            ...options,
            headers: {
                ...(token ? { 'Authorization': `Bearer ${token}` } : {}),
                'Accept': 'application/json',
                ...(isFormData ? {} : { 'Content-Type': 'application/json' }),
                ...options.headers
            }
        };
        try {
            const apiUrl = useRuntimeConfig().public.apiUrl;
            return await $fetch(`${apiUrl}${url}`, defaultOptions);
        } catch (error) {
            const isAuthEndpoint = url.startsWith('/account/');
            if (error.status === 401 && !isAuthEndpoint) {
                removeToken();
                removeUser();
                await navigateTo('/login');
            }
            throw error;
        }
    };
    return { authFetch };
};