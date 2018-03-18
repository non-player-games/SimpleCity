import * as path from "path";
import { Observable, Subject } from "rxjs";
import { spawn } from "child_process";
import * as uuid from "uuid/v4";

// internal memory to track which uuid for which method call
const memory = new Map();

// TODO: change string type to be consise type
export interface Action {
    type: string;
    payload: any;
}
export type Producer = Observable<Action>;
export type Consumer = Observable<Action>;

// TODO: change the path to be configuration based
const systemFile = path.join(__dirname, "../systems/systems");
const state = {
    process: null
};

export function createSource (): Consumer {
    return new Subject<Action>();
}

export function cleanup (): void {
    if (state.process) {
        console.log("shutting down systems");
        // DIE POTATO, DIE
        state.process.kill();
    }
}

export function start (input: Consumer): Producer {
    const output = Observable.create(observer => {
        state.process = spawn(systemFile, []);
        state.process.stdout.on("data", (dataBuffer: Buffer) => {
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
                if (memory.has(msgUUID)) {
                    observer.next(handler(memory.get(msgUUID))(line));
                    memory.delete(msgUUID);
                } else {
                    console.error("receive unknown uuid method reply", line);
                }
            });
        });
        state.process.stderr.on("data", (data: Buffer) => {
            console.error(`process stderr:\n${data}`);
        });
        state.process.on("close", (code) => {
            console.log("System process closed with code", code);
        });

        input.subscribe((action: Action) => {
            const msgUUID = uuid();
            let msg = `${msgUUID} ${action.type}`;
            if (action.payload) {
                msg = msg + ` ${action.payload}`;
            }
            msg = msg + "\n";
            memory.set(msgUUID, action.type);
            state.process.stdin.write(msg);
        });

        // when observable is closed, clean up the process
        return () => {
            state.process.stdin.write(`${uuid()} shutdown`);
        };
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
                payload: parseJSONSafe(reply.split(" ")[1], reply.split(" ")[1])
            };
        }
    };
}

function parseJSONSafe(jsonStr, defaultVal) {
    try {
        return JSON.parse(jsonStr);
    } catch (e) {
        return defaultVal;
    }
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
