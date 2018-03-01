import { initWindow } from "./window";
import handlers from "./handlers";
import { start } from "./comms";

// create window
const window = initWindow();

// Initialize communications with system
const events$ = start();

// define ipc message handling
handlers(window, events$);
