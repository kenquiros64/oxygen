import {create} from "zustand/index";
import {Route} from "../models/models.ts";
import { invoke } from "@tauri-apps/api/core";

interface RoutesStore {
    // LISTS
    routes: Route[];

    // VALUES
    loading: boolean;
    error: string | null;

    // ACTIONS
    fetchRoutes: () => void;
}

export const useRoutesStore = create<RoutesStore>((set) => ({
    routes: [],
    loading: true,
    error: null,
    fetchRoutes: () => {
        set({ loading: true });
        invoke("fetch_routes").then((routes) => {

            console.log("Fetching...")
            console.log(routes as Route[])
            set({ routes: routes as Route[], loading: false });
        }).catch((error) => {
            console.error("ERROR FETCHING..." + error);
            set({ error: error.message, loading: false });
        });
    }
}));