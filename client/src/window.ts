import * as path from "path";
import * as url from "url";
import {app, BrowserWindow} from "electron";

// Keep a global reference of the window object, if you don"t, the window will
// be closed automatically when the JavaScript object is garbage collected.
export const state = {
    mainWindow: null
};

// when need to route mainWindow to different page, create a function here

export function initWindow (cleanUpCb) {
    // This method will be called when Electron has finished
    // initialization and is ready to create browser windows.
    // Some APIs can only be used after this event occurs.
    app.on("ready", createWindow(cleanUpCb));

    // Quit when all windows are closed.
    app.on("window-all-closed", function () {
        // On OS X it is common for applications and their menu bar
        // to stay active until the user quits explicitly with Cmd + Q
        if (process.platform !== "darwin") {
            app.quit();
        }
    });

    app.on("activate", function () {
        // On OS X it"s common to re-create a window in the app when the
        // dock icon is clicked and there are no other windows open.
        if (state.mainWindow === null) {
            createWindow(cleanUpCb);
        }
    });
}

function createWindow (cleanUpCb) {
    return function () {
        // Create the browser window.
        state.mainWindow = new BrowserWindow({width: 800, height: 600});

        // and load the index.html of the app.
        state.mainWindow.loadURL(url.format({
            pathname: path.join(__dirname, "..", "views", "build", "index.html"),
            protocol: "file:",
            slashes: true
        }));

        // Open the DevTools.
        // state.mainWindow.webContents.openDevTools()

        // Emitted when the window is closed.
        state.mainWindow.on("closed", function () {
            // Dereference the window object, usually you would store windows
            // in an array if your app supports multi windows, this is the time
            // when you should delete the corresponding element.
            state.mainWindow = null;
            cleanUpCb();
        });
    };
}
