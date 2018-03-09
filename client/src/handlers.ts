import { ipcMain } from "electron";

import { Producer, Action } from "./comms";

interface Input {
    window: any;
    systems: Producer;
    events: any;
}

/**
 * set up ipc binding between the renderer to actual function
 */
export default function (input: Input) {
    input.systems.subscribe(sendMessageToAll(input.window));
    ipcMain.on("ipc-message", ( _: any, args: any ) => {
        console.log("main process got message", args);
        input.events.next(args);
    });
}

/**
 * internal helper functions (not exported by default)
 */

/**
 * to send message to all window, used for main process to send message to
 * renderer (regardless of the window)
 */
function sendMessageToAll(window): (d: Action) => void {
    return (data: Action) => {
        if (window) {
            window.webContents.send("ipc-message", data);
        } else {
            console.log("Window not initialized yet");
        }
    };
}
