export const usePublicFetch = () => {
    const publicFetch = (url, options = {}) => {
        const apiUrl = useRuntimeConfig().public.apiUrl;
        return $fetch(`${apiUrl}${url}`, {
            ...options,
            headers: {
                Accept: 'application/json',
                ...options.headers
            }
        });
    };
    return { publicFetch };
};