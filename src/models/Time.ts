export class Time {
    hour: number;
    minute: number;

    constructor(hour: number, minute: number) {
        this.hour = hour;
        this.minute = minute;
    }

    static fromJSON(json: any): Time {
        return new Time(json.hour, json.minute);
    }

    static from12HourString(time: string): Time {
        const match = time.match(/^(\d{1,2}):(\d{2})\s*(AM|PM)$/i);

        if (!match) {
            throw new Error("Invalid time format. Use 'hh:mm AM/PM'.");
        }

        let [_, hourStr, minuteStr, meridiem] = match;

        let hour = parseInt(hourStr, 10);
        const minute = parseInt(minuteStr, 10);

        // Convert to 24-hour format
        if (meridiem.toUpperCase() === "PM" && hour !== 12) {
            hour += 12;
        } else if (meridiem.toUpperCase() === "AM" && hour === 12) {
            hour = 0;
        }

        return new Time(hour, minute);
    }

    toJSON(): any {
        return {
            hour: this.hour,
            minute: this.minute
        };
    }

    toMinutes(): number {
        return this.hour * 60 + this.minute;
    }

    to12HourString(): string {
        const hour = this.hour % 12 || 12;
        const minute = this.minute.toString().padStart(2, "0");
        return `${hour.toString().padStart(2, '0')}:${minute} ${this.hour < 12 ? "AM" : "PM"}`;
    }

    toShortString(): string {
        return `${this.hour.toString().padStart(2, '0')}:${this.minute.toString().padStart(2, '0')}`;
    }

    static fromCurrentTime(): number {
        const now = new Date();
        return now.getHours() * 60 + now.getMinutes();
    }
}


