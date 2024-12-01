import {create} from "zustand/index";

type RouteStopTimeCount = {
  [route: string]: {
    [stop: string]: {
      [time: string]: number;
    };
  };
};

interface TicketState {
    selectedRoute: string | null;
    selectedStop: string | null;
    selectedTime: string | null;
    selectedTimetable: string | null;

    routeStopTimeCounts: RouteStopTimeCount;
    totalCountsForRouteTime: { [route: string]: { [time: string]: number } };
  
    setSelectedRoute: (route: string) => void;
    setSelectedStop: (stop: string) => void;
    setSelectedTime: (time: string) => void;
    setSelectedTimetable: (timetable: string) => void;
  
    incrementCount: (route: string, stop: string, time: string) => void;
    getRouteStopTimeCount: (route: string, stop: string, time: string) => number;
    getTotalCountForRouteTime: (route: string, time: string) => number;
}

export const useTicketStore = create<TicketState>((set, get) => ({
    selectedRoute: null,
    selectedStop: null,
    selectedTime: null,
    selectedTimetable: null,
    routeStopTimeCounts: {},
    totalCountsForRouteTime: {},
  
    setSelectedRoute: (route) =>
      set(() => ({
        selectedRoute: route,
      })),
    setSelectedStop: (stop) =>
      set(() => ({
        selectedStop: stop,
      })),
    setSelectedTime: (time) =>
      set(() => ({
        selectedTime: time,
      })),
    setSelectedTimetable: (timetable) =>
      set(() => ({
        selectedTimetable: timetable,
      })),
  
    incrementCount: (route, stop, time) =>
      set((state) => {
        const updatedCounts = { ...state.routeStopTimeCounts };
        if (!updatedCounts[route]) updatedCounts[route] = {};
        if (!updatedCounts[route][stop]) updatedCounts[route][stop] = {};
        if (!updatedCounts[route][stop][time]) updatedCounts[route][stop][time] = 0;
  
        updatedCounts[route][stop][time] += 1;
  
        const updatedTotalCounts = { ...state.totalCountsForRouteTime };
        if (!updatedTotalCounts[route]) updatedTotalCounts[route] = {};
        if (!updatedTotalCounts[route][time]) updatedTotalCounts[route][time] = 0;
  
        updatedTotalCounts[route][time] += 1;
  
        return {
          routeStopTimeCounts: updatedCounts,
          totalCountsForRouteTime: updatedTotalCounts,
        };
      }),
  
    getRouteStopTimeCount: (route, stop, time) => {
      const counts = get().routeStopTimeCounts;
      return counts[route]?.[stop]?.[time] || 0;
    },
  
    getTotalCountForRouteTime: (route, time) => {
      const totalCounts = get().totalCountsForRouteTime;
      return totalCounts[route]?.[time] || 0;
    },
  }));
  