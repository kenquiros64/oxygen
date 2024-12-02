import { create } from "zustand";

interface AuthState {
    user: string | null; // Represents the username
    name: string | null; // Represents the full name or display name
    login: (user: string, name: string) => void; // Updated to accept name
    logout: () => void;
}

export const useAuthStore = create<AuthState>((set) => ({
    user: null,
    name: null,
    login: (username: string, displayName: string) =>
        set({ user: username, name: displayName }), // Updates both user and name
    logout: () => set({ user: null, name: null }), // Clears both user and name
}));