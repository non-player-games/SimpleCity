import * as path from "path";
import { Observable, Subject } from "rxjs";
import { spawn } from "child_process";
import * as uuid from "uuid/v4";

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
        process.stdout.on("data", (data: Buffer) => {
            const dataString = new Buffer(data).toString("ascii");
            console.log(`process stdout:\n${dataString}`);
            // TODO: process data string as `uuid methodName args`
            const parts = dataString.split(" ");
            if (parts.length < 2) {
                console.error("Got unsupported format of message from system. Ignoring ...", dataString);
                return;
            }
            const type = parts[1];
            let payload = "";
            if (parts.length === 3) {
                payload = parts[2];
            }
            observer.next({
                type,
                payload
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
            let msg = `${uuid()} ${action.type}`;
            if (action.payload) {
                msg = msg + ` ${action.payload}`;
            }
            msg = msg + "\n";
            console.log("communication got message", action, msg);
            process.stdin.write(msg);
            process.stdin.end();
        });
    });
    return output;
}
