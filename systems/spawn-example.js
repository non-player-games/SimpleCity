const { spawn } = require('child_process');


// This is an example of how node could interact with a 
// rust child process and perform IPC

const child = spawn('./target/release/systems', []);

child.stdout.on('data', (data) => {
    let s = new Buffer(data).toString('ascii');
    if(s.startsWith('SYSTEM:')) {
        console.log(`child stdout:\n${s}`);
    }
});

child.stderr.on('data', (data) => {
    console.error(`child stderr:\n${data}`);
});

function sendMessage(messages, timeout){
    setTimeout(function(){
        if(messages.length > 0){
            let m = messages[0];
            console.log('sending ' + m);
            child.stdin.write(m + '\n');
            sendMessage(messages.slice(1), timeout);
        }
    }, timeout);
}

let messages = [
    'Message 1',
    'Message 2',
    'Message 3',
    'QUIT'
];

sendMessage(messages, 1000);
