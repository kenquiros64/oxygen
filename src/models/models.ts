export interface Route {
    _id?: string; // MongoDB uses "_id" for unique identifiers; optional
    departure: string;
    destination: string;
    stops: Stop[];
    timetable: Time[];
    holidayTimetable: Time[];
}

export interface Stop {
    name: string;
    code: string;
    fare: number;
    goldFare: number;
    isMain: boolean;
}

export interface Time {
    hour: number;
    minute: number;
}

export interface User {
    username: string;
    name: string;
    role: "admin" | "user";
}

export interface ErrorResponse {
    code: number;
    error: string;
    details: string;
}