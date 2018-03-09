import { initWindow, state } from "./window";
import handlers from "./handlers";
import { start, createSource } from "./comms";

// create window
initWindow();

const input$ = createSource();

// Initialize communications with system
const events$ = start(input$);

// define ipc message handling
handlers({
    windowState: state,
    systems: events$,
    events: input$
});
