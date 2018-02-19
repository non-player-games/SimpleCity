import {initWindow} from "./window";
import handlers from "./handlers";

// create window
initWindow();

// define ipc message handling
handlers();
