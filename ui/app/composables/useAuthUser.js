export const useAuthUser = () => {
    const user = useCookie('webedit_token', {
        maxAge: 3600 * (24 * 30),
        sameSite: 'strict',
        secure: import.meta.client ? window.location.protocol === 'https:' : true,
        path: '/'
    });

    const setUser = (newUser) => {
        user.value = newUser;
    };
    const removeUser = () => {
        user.value = null;
    };
    const getUser = () => {
        return user.value;
    };
    return {
        user,
        setUser,
        removeUser,
        getUser
    };
};