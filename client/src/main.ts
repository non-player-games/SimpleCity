import {initWindow} from "./window";
import handlers from "./handlers";
import {initSystem} from "./comms";

// create window
initWindow();

// define ipc message handling
handlers();

// Initialize communications with system
// Does not do anything very interesting at the moment
initSystem();
