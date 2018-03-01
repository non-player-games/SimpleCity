import * as path from "path";
import * as EventEmitter from "events";
import {spawn} from "child_process";

const systemFile = path.join(__dirname, "../systems/systems");
export const eventName = "ipc-data";

let process = null;
export class EventProducer extends EventEmitter {}

export function start (): EventProducer {
    const producer = new EventProducer();
    process = spawn(systemFile, []);
    process.stdout.on("data", (data: Buffer) => {
        const dataString = new Buffer(data).toString("ascii");
        producer.emit(eventName, dataString);
        console.log(`process stdout:\n${dataString}`);
    });

    process.stderr.on("data", (data: Buffer) => {
        console.error(`process stderr:\n${data}`);
    });

    return producer;
}

export function send (message: string) {
    if (!process) {
        console.error("Unable to find process (could be null)");
        return;
    }
    console.debug("sending " + message);
    process.stdin.write(message + "\n");
}
