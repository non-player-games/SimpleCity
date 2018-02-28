import * as path from "path";
import {spawn} from "child_process";

const systemFile = path.join(__dirname, "../systems/systems");

const child = spawn(systemFile, []);

child.stdout.on("data", (data: Buffer) => {
    let s = new Buffer(data).toString("ascii");
    if (s.startsWith("SYSTEM:")) {
        console.log(`child stdout:\n${s}`);
    }
});

child.stderr.on("data", (data: Buffer) => {
    console.error(`child stderr:\n${data}`);
});

function sendMessage(messages, timeout) {
    setTimeout(function () {
        if (messages.length > 0) {
            let m = messages[0];
            console.log("sending " + m);
            child.stdin.write(m + "\n");
            sendMessage(messages.slice(1), timeout);
        }
    }, timeout);
}

export function initSystem () {
    let messages = [
        "Message 1",
        "Message 2",
        "Message 3",
        "QUIT"
    ];

    sendMessage(messages, 4000);
}
