import {ipcMain} from "electron";
import xs, { Stream } from "xstream";
import { adapt } from "@cycle/run/lib/adapt";
import fromEvent from "xstream/extra/fromEvent";
import * as EventEmitter from "events";

export interface Sink {
    events: Stream<any>;
}
export type Input = any;

class EventProducer extends EventEmitter {};
const eventName: string = "click";

export function makeIPCDriver(): (m: Stream<Input>) => Sink {
    const producer = new EventProducer();
    const sink = {
        events: adapt(fromEvent(producer, eventName))
    };

    return function IPCDriver(sink$: Stream<Input>): Sink {
        ipcMain.on("message", (data: any) => {
            producer.emit(eventName, data);
        });
        return sink;
    }
}
