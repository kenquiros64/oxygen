import { create } from "zustand";
import {User} from "../models/models.ts";
import {invoke} from "@tauri-apps/api/core";

interface AuthState {
    user: User | null;
    login: (username: string, password: string) => Promise<void>;
    logout: () => void;
}

export const useAuthState = create<AuthState>((set) => ({
    user: null,
    login: (username, password) => {
        return new Promise<void>(async (resolve, reject) => {
            try {
                const user = await invoke("login", { username, password });
                set({ user: user as User });
                resolve();
            } catch (error) {
                set({ user: null });
                reject(error as Error); // Mark the promise as failed
            }
        });
    },
    logout: () => {
        set({ user: null });
    },
}));