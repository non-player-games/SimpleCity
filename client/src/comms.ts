import * as path from "path";
import { Observable, Subject } from "rxjs";
import { spawn } from "child_process";

// TODO: change string type to be consise type
export type Producer = Observable<string>;
export type Consumer = Observable<string>;

// TODO: change the path to be configuration based
const systemFile = path.join(__dirname, "../systems/systems");

export function createSource (): Consumer {
    return new Subject<string>();
}

export function start (input: Consumer): Producer {
    const output = Observable.create(observer => {
        const process = spawn(systemFile, []);
        process.stdout.on("data", (data: Buffer) => {
            const dataString = new Buffer(data).toString("ascii");
            console.log(`process stdout:\n${dataString}`);
            observer.next(dataString);
        });
        process.stderr.on("data", (data: Buffer) => {
            console.error(`process stderr:\n${data}`);
            observer.error(data);
        });

        input.subscribe((data: string) => {
            process.stdin.write(data + "\n");
        });
    });
    return output;
}
