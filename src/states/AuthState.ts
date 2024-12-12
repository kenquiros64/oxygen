import { create } from "zustand";
import {invoke} from "@tauri-apps/api/core";
import {User} from "../models/User.ts";

type AuthState = {
    user: User | null;
}

type Actions = {
    login: (username: string, password: string) => Promise<void>;
    logout: () => void;
}

const initialState: AuthState = {
    user: null,
}

export const useAuthState = create<AuthState & Actions>()((set) => ({
    ...initialState,
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
}))