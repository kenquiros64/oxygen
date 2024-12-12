export class Stop {
    name: string;
    code: string;
    fare: number;
    goldFare: number;
    isMain: boolean;

    constructor(name: string, code: string, fare: number, goldFare: number, isMain: boolean) {
        this.name = name;
        this.code = code;
        this.fare = fare;
        this.goldFare = goldFare;
        this.isMain = isMain;
    }

    static fromJSON(json: any): Stop {
        return new Stop(json.name, json.code, json.fare, json.gold_fare, json.is_main);
    }

    toJSON(): any {
        return {
            name: this.name,
            code: this.code,
            fare: this.fare,
            gold_fare: this.goldFare,
            is_main: this.isMain
        };
    }
}