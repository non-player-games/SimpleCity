#[macro_use] extern crate serde_derive;
extern crate systems;
extern crate serde_json;

use systems::simulation::{ZoneGrid, PopulationGrid, Zone, Vector2};

use std::collections::vec_deque::Drain;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::io::{self, Write};
use std::mem;
use std::{thread, time};


fn main() {
    println!("Starting systems ...");

    // Examples: 
    // ZoneGrid init, print, change, print
    let mut z = ZoneGrid::new(v2(16, 16));
    {
        let mut e = z.get_zone(&v2(0,0));
        match e {
            Some(old_zone) => {
                mem::replace(old_zone, Zone::Residential);
            },
            None => {},
        }
    }
    println!("ZoneGrid changed:\n{:?}", &z);
    let serialized = serde_json::to_string(&z).unwrap();
    println!("Serialized: {}", &serialized);

    // Population Grid init
    let p = PopulationGrid::new(v2(16, 16));
    println!("PopulationGrid default:\n{:?}", p);
    listen();
}

/// Sends a message to stdout
// We might end of scratching this out
fn send_message(message: &[u8]) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write(b"SYSTEM: ");
    handle.write(message);
    handle.write(b"\n");
    handle.flush();
}

fn listen(){
    //let data = Arc::new(Mutex::new(String::from("")));
    let data: Arc<Mutex<VecDeque<String>>> = Arc::new(Mutex::new(VecDeque::new()));
    {
        let d = data.clone();
        // This thread is dedicated to reading from stdin because stdin
        // is blocking
        thread::spawn(move || {
            let mut input = String::from("");
            loop {
                match io::stdin().read_line(&mut input) {
                    Ok(_) => {
                        input = input.trim().to_string();
                        let mut dd = d.lock().unwrap();
                        //*dd = input.clone();
                        dd.push_back(input.clone());
                        input.clear();
                        // @Robustness when we terminate the function, how does this thread get cleaned
                        // up?!?!
                    }
                    Err(error) => println!("error: {}", error),
                }
            }
        });
    }
    
    // @Incomplete this is the skeleton of what the systems side game/simulation loop
    // pause it every second to simulate it doing processing
    // will terminate once it receives QUIT from stdin
    let sec = time::Duration::from_millis(1000);
    let mut done = false;
    while !done {
        thread::sleep(sec * 2); 
        let mut cmd: String;
        // READ COMMANDS
        {
            let mut drained: Vec<String> = Vec::with_capacity(0);
            {
                let mut d = data.lock().unwrap();
                drained = d.drain(..).collect();
            }
            for d in drained {
                cmd = d;
                if cmd.len() > 0 {
                    if cmd.as_str().starts_with("quitGame") {
                        done = true;
                    }
                    if cmd.as_str().starts_with("echo") {
                        send_message(&cmd.as_bytes());
                    }
                }
            }
        }
        
    }
}

/// Convenience function for making Vector2
// @Copypaste
fn v2(x: usize, y: usize) -> Vector2 {
    Vector2 { x: x, y: y }
}
