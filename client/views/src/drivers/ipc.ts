import { ipcRenderer } from "electron";
import xs, { Stream } from "xstream";
import { adapt } from "@cycle/run/lib/adapt";
import fromEvent from "xstream/extra/fromEvent";
import * as EventEmitter from "events";

export interface Sink {
    events: Stream<any>;
}
export type Input = any;

// TODO: change EventEmitter to RxJS Observable for consistency
class EventProducer extends EventEmitter {};
const eventName: string = "ipc-message";

export function makeIPCDriver(): (m: Stream<Input>) => Sink {
    const producer = new EventProducer();
    const sink = {
        events: adapt(fromEvent(producer, eventName))
    };

    return function IPCDriver(sink$: Stream<Input>): Sink {
        ipcRenderer.on("ipc-message", (_: any, data: any) => {
            producer.emit(eventName, data);
        });
        sink$.subscribe({
            next: (data: any): void => {
            	console.log("got data", data);
                ipcRenderer.send("ipc-message", data);
            },
            error: (err: any): void => {
                console.error("ipc sink sends error", err);
            },
            complete: (): void => {
                console.log("ipc sink closes sink");
            }
        });
        return sink;
    }
}
