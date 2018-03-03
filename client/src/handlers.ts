import { ipcMain } from "electron";

import { Producer } from "./comms";

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
    ipcMain.on("ipc-message", ( args: any ) => {
        console.debug(args);
    });
}

/**
 * internal helper functions (not exported by default)
 */

/**
 * to send message to all window, used for main process to send message to
 * renderer (regardless of the window)
 */
function sendMessageToAll(window): (d: string) => void {
    return (data: string) => {
        window.webContents.send("ipc-data", data);
    };
}
