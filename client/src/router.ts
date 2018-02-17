import {ipcMain} from "electron";
import glue from "./glue";

/**
 * set up ipc binding between the renderer to actual function
 */
export default function () {
    // set up routing
    ipcMain.on("fib", (event, arg) => {
        event.sender.send(glue.calc(arg));
    });
}
