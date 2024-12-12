import {create} from "zustand/index";
import {Route} from "../models/Route.ts";
import {Stop} from "../models/Stop.ts";
import {Time} from "../models/Time.ts";
import {invoke} from "@tauri-apps/api/core";


type TicketState = {
    // SELECTED
    selectedRoute: Route | null;
    selectedStop: Stop | null;
    selectedTimetable: 'normal' | 'holiday';
    selectedTime: Time | null;

    // VALUES
    code: string;
    routeTimeCounts: CounterMap;
    currentCount: number;
    currentGoldCount: number;
    routes: Route[];
}

type Actions = {
    setSelectedRoute: (route: Route) => void;
    setSelectedStop: (stop: Stop) => void;
    setSelectedTime: (time: Time) => void;
    setSelectedTimetable: (timetable: 'normal' | 'holiday') => void;
    setCode: (code: string) => void;

    incrementCount: (qty: number, type: "normal" | "gold") => void;
    getAllCounts: () => void;
    resetTicketState: () => void;
    getCount: (stop: string) => number;
    getStopCounts: () => void;
}

const initialState: TicketState = {
    selectedRoute: null,
    selectedStop: null,
    selectedTime: null,
    selectedTimetable: 'normal',

    code: "",
    routeTimeCounts: {},
    routes: [],
    currentCount: 0,
    currentGoldCount: 0,
}

export const useTicketState = create<TicketState & Actions>()((set, get) => ({
    ...initialState,
    setSelectedRoute: (route: Route) => {
        set({ selectedRoute: route })
    },
    setSelectedStop: (stop: Stop) => {
        set({ selectedStop: stop })
    },
    setSelectedTime: (time: Time) => {
        set({ selectedTime: time })
    },
    setSelectedTimetable: (timetable: 'normal' | 'holiday') => {
        set({ selectedTimetable: timetable })
    },
    setCode: (code: string) => {
        set({ code: code })
    },
    incrementCount: (qty: number, type: "normal" | "gold") => {
        const stop = get().selectedStop;
        if (!stop) {
            return;
        }
        const time = get().selectedTime;
        if (!time) {
            return;
        }
        const route = get().selectedRoute;
        if (!route) {
            return;
        }

        const routeTimeCounts = get().routeTimeCounts;

        let key = `${route.toKey()}-${stop.name.toLowerCase()}-${time.toShortString()}`;
        const original = key;
        if (type === "gold") {
            key += "-gold";
        }
        invoke<number>("increment_stop_counter", { key, qty }).then((value: number) => {
            if (type === "gold") {
                set({ currentGoldCount: value });
            } else {
                set({ currentCount: value });
            }
            routeTimeCounts[original] = (routeTimeCounts[original] || 0) + qty;
        });
    },
    getAllCounts: () => {
        let selectedRoute = get().selectedRoute;
        let selectedTime = get().selectedTime;
        if (!selectedRoute || !selectedTime) {
            return;
        }

        const prefixes = selectedRoute?.generateCounterKeys(selectedTime);
        invoke<CounterMap>("bulk_counts_by_prefixes", { prefixes }).then((counter: CounterMap) => {
            for (let code in counter) {
                console.log(`Code: ${code}, Count: ${counter[code]}`);
            }
            set({ routeTimeCounts: counter });
        });
    },
    getCount: (stop: string) => {
        const time = get().selectedTime;
        if (!time) {
            return 0;
        }
        const route = get().selectedRoute;
        if (!route) {
            return 0;
        }

        const key = `${route.toKey()}-${stop.toLowerCase()}-${time.toShortString()}`;
        if (!key) {
            return 0;
        }
        return get().routeTimeCounts[key] || 0;
    },
    getStopCounts: () => {
        const selectedRoute = get().selectedRoute;
        if (!selectedRoute) {
            return;
        }
        const time = get().selectedTime;
        if (!time) {
            return 0;
        }
        const stop = get().selectedStop?.name;
        if (!stop) {
            return 0;
        }

        const key = `${selectedRoute.toKey()}-${stop.toLowerCase()}-${time.toShortString()}`;
        invoke<CounterMap>("get_stop_counters", { key }).then((counter: CounterMap) => {
            set({ currentGoldCount: 0 });
            set({ currentCount: 0 });
            for (let code in counter) {
                if (code.endsWith("gold")) {
                    set({ currentGoldCount: counter[code] });
                } else {
                    set({ currentCount: counter[code] });
                }
            }
        });
    },
    resetTicketState: () => {
        set(initialState)
    },
}))
  