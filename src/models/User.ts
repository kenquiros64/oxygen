export class User {
    username: string;
    name: string;
    role: "admin" | "user";

    constructor(username: string, name: string, role: "admin" | "user") {
        this.username = username;
        this.name = name;
        this.role = role;
    }

    static fromJSON(json: any): User {
        return new User(json.username, json.name, json.role);
    }

    toJSON(): any {
        return {
            username: this.username,
            name: this.name,
            role: this.role
        };
    }
}

