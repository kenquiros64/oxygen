import {create} from "zustand/index";
import { invoke } from "@tauri-apps/api/core";
import {Route} from "../models/Route.ts";

type RoutesState = {
    // LISTS
    routes: Route[];

    // VALUES
    loading: boolean;
    error: string | null;
}


type Actions = {
    fetchRoutes: () => void;
    resetRoutesState: () => void;
}

const initialState: RoutesState = {
    routes: [],
    loading: true,
    error: null,
}

export const useRoutesState = create<RoutesState & Actions>()((set) => ({
    ...initialState,
    fetchRoutes: () => {
        set({ loading: true });
        invoke<any[]>("fetch_routes").then((routes: any[]) => {

            const parsedRoutes = routes.map((routeJson) => {
                try {
                    return Route.fromJSON(routeJson);
                } catch (e) {
                    return null;
                }
            }).filter((route): route is Route =>
                route !== null && route.stops.length > 0 && route.timetable.length > 0 && route.holidayTimetable.length > 0
            );

            set({ routes: parsedRoutes, loading: false });
        }).catch((error) => {
            set({ error: error.message, loading: false });
        });
    },
    resetRoutesState: () => {
        set(initialState)
    },
}))
