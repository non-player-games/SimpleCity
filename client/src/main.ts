import { initWindow, state } from "./window";
import handlers from "./handlers";
import { start, cleanup, createSource } from "./comms";

// initializing systems
const input$ = createSource();

// Initialize communications with system
const events$ = start(input$);

// define ipc message handling
handlers({
    windowState: state,
    systems: events$,
    events: input$
});

// create window
initWindow(function() {
    // clean up the system listener as well as killing system process
    cleanup();
});
