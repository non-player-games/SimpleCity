import { ipcMain } from "electron";

import { Producer, Action } from "./comms";

interface Input {
    windowState: any;
    systems: Producer;
    events: any;
}

/**
 * set up ipc binding between the renderer to actual function
 */
export default function (input: Input) {
    ipcMain.on("ipc-message", ( _: any, args: any ) => {
        input.events.next(args);
    });
    input.systems.subscribe(sendMessageToAll(input.windowState));
}

/**
 * internal helper functions (not exported by default)
 */

/**
 * to send message to all window, used for main process to send message to
 * renderer (regardless of the window)
 */
function sendMessageToAll(windowState): (d: Action) => void {
    return (data: Action) => {
        if (windowState.mainWindow) {
            windowState.mainWindow.webContents.send("ipc-message", data);
        } else {
            console.log("Window not initialized yet");
        }
    };
}
