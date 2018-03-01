// import { ipcMain } from "electron";

import { eventName, EventProducer } from "./comms";

/**
 * set up ipc binding between the renderer to actual function
 */
export default function (window, events$: EventProducer) {
    events$.on(eventName, (data: string) => {
        window.webContents.send("ipc-data", data);
    });
}
