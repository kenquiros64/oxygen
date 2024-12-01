import {create} from "zustand";

interface AuthState {
    user: string | null;
    login: (user: string) => void;
    logout: () => void;
}

export const useAuthStore = create<AuthState>((set) => ({
    user: null,
    login: (username: string) => set({ user: username }),
    logout: () => set({ user: null }),
}));