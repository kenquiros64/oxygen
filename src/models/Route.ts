import {Time} from "./Time.ts";
import {Stop} from "./Stop.ts";

export class Route {
    _id?: string; // MongoDB uses "_id" for unique identifiers; optional
    departure: string;
    destination: string;
    stops: Stop[];
    timetable: Time[];
    holidayTimetable: Time[];

    constructor(
        departure: string,
        destination: string,
        stops: Stop[],
        timetable: Time[],
        holidayTimetable: Time[],
        _id?: string
    ) {
        this._id = _id;
        this.departure = departure;
        this.destination = destination;
        this.stops = stops;
        this.timetable = timetable;
        this.holidayTimetable = holidayTimetable;
    }

    static fromJSON(json: any): Route {
        return new Route(
            json.departure,
            json.destination,
            json.stops.map((stop: any) => Stop.fromJSON(stop)),
            json.timetable.map((time: any) => Time.fromJSON(time)),
            json.holiday_timetable.map((time: any) => Time.fromJSON(time)),
            json._id
        );
    }

    toJSON(): any {
        return {
            _id: this._id,
            departure: this.departure,
            destination: this.destination,
            stops: this.stops.map((stop) => stop.toJSON()),
            timetable: this.timetable.map((time) => time.toJSON()),
            holiday_timetable: this.holidayTimetable.map((time) => time.toJSON())
        };
    }

    toKey(): string {
        return `${this.departure.toLowerCase()}-${this.destination.toLowerCase()}`;
    }

    nextDeparture(timetable: 'normal' | 'holiday'): Time  {
        let times = this.timetable;
        if (timetable === "holiday") {
            times = this.holidayTimetable;
        }

        const currentTimeInMinutes = Time.fromCurrentTime();

        // Filter and sort times to find the next departure
        const sortedTimes = times
            .map((time) => ({
                time,
                minutes: time.toMinutes(),
            }))
            .filter(({ minutes }) => minutes >= currentTimeInMinutes)
            .sort((a, b) => a.minutes - b.minutes);

        return sortedTimes.length > 0 ? sortedTimes[0].time : times[0];
    }

    generateCounterKeys(time: Time): string[] {
        return this.stops.map((stop) => `${this.toKey()}-${stop.name.toLowerCase()}-${time.toShortString()}`);
    }
}

