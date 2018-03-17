import * as path from "path";
import { Observable, Subject } from "rxjs";
import { spawn } from "child_process";
import * as uuid from "uuid/v4";

// internal memory to track which uuid for which method call
const memory = {};

// TODO: change string type to be consise type
export interface Action {
    type: string;
    payload: any;
}
export type Producer = Observable<Action>;
export type Consumer = Observable<Action>;

// TODO: change the path to be configuration based
const systemFile = path.join(__dirname, "../systems/systems");

export function createSource (): Consumer {
    return new Subject<Action>();
}

export function start (input: Consumer): Producer {
    const output = Observable.create(observer => {
        const process = spawn(systemFile, []);
        process.stdout.on("data", (dataBuffer: Buffer) => {
            const data = new Buffer(dataBuffer).toString("ascii").split("\n");
            console.log("Got message", data);
            data.forEach(line => {
                // TODO: process data string as `uuid methodName args`
                const parts = line.split(" ");
                const msgUUID = parts[0];
                // skip empty lines
                if (!line) {
                    return;
                }
                if (memory[msgUUID]) {
                    observer.next(handler(memory[msgUUID])(line));
                    delete memory[msgUUID];
                } else {
                    console.error("receive unknown uuid method reply", line);
                }
            });
        });
        process.stderr.on("data", (data: Buffer) => {
            console.error(`process stderr:\n${data}`);
            observer.error(data);
        });
        process.on("close", (code) => {
            console.log("System process closed with code", code);
        });

        input.subscribe((action: Action) => {
            const msgUUID = uuid();
            let msg = `${msgUUID} ${action.type}`;
            if (action.payload) {
                msg = msg + ` ${action.payload}`;
            }
            msg = msg + "\n";
            memory[msgUUID] = action.type;
            process.stdin.write(msg);
        });
    });
    return output;
}

// handlers for handling reply from the system
function handler (action: string): (d: string) => Action {
    return (reply: string): Action => {
        switch (action) {
        case "getZoneGrid":
            return {
                type: action,
                payload: buildGrid(reply.split(" ")[1])
            };
        default:
            return {
                type: action,
                payload: reply.split(" ")[1]
            };
        }
    };
}

function buildGrid(rawData: string): number[][] {
    try {
        const data = JSON.parse(rawData);
        const grid = [];
        for (let i = 0; i < data.size.x; i ++) {
            grid[i] = [];
            for (let j = 0; j < data.size.y; j ++) {
                grid[i][j] = data.zones[i * data.size.x + j];
            }
        }
        return grid;
    } catch (e) {
        console.error("Failed to parse JSON for Grid data", rawData);
        return [];
    }
}
