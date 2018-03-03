import { initWindow } from "./window";
import handlers from "./handlers";
import { start, createSource } from "./comms";

// create window
const window = initWindow();

const input$ = createSource();

// Initialize communications with system
const events$ = start(input$);

// define ipc message handling
handlers({
    window,
    systems: events$,
    events: input$
});
