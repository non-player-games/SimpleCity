extern crate regex;
#[macro_use] extern crate serde_derive;
extern crate systems;
extern crate serde_json;

use regex::Regex;
use systems::simulation::{ZoneGrid, PopulationGrid, Zone, Vector2};
use std::collections::vec_deque::Drain;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::io::{self, Write};
use std::mem;
use std::{thread, time};


fn main() {
    println!("Starting systems ...");
    /*
    // Examples: 
    // ZoneGrid init, print, change, print
    let serialized = serde_json::to_string(&z).unwrap();
    println!("Serialized: {}", &serialized);

    // Population Grid init
    let p = PopulationGrid::new(v2(16, 16));
    println!("PopulationGrid default:\n{:?}", &p);
    let serialized_pop = serde_json::to_string(&p).unwrap();
    println!("Serialized: {}", &serialized_pop);
    */
    listen();
}

/// Sends a message to stdout
// We might end of scratching this out
fn send_message(message: &[u8]) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write(message);
    handle.write(b"\n");
    handle.flush();
}

fn listen(){
    // Initialize grids
    let mut zone_grid = ZoneGrid::new(v2(16, 16));
    let mut pop_grid = PopulationGrid::new(v2(16, 16));
    let command_regex: Regex = Regex::new(r"^([0-9a-f-]{36})\s+([a-zA-Z]+)(\s+(.+))?$").unwrap();

    //
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
    let mut logical_time: u64 = 0;
    while !done {
        let mut line: String;
        // READ COMMANDS
        {
            let mut drained: Vec<String> = Vec::with_capacity(0);
            {
                let mut d = data.lock().unwrap();
                drained = d.drain(..).collect();
            }
            for d in drained {
                line = d;
                let client_msg_opt = parse_client_message(&command_regex, &line);
                if client_msg_opt.is_none(){
                    continue;
                }
                // @Incomplete: we may want to move this elsewhere and make methods for
                // each command perhaps.
                let client_msg = client_msg_opt.unwrap();
                let cmd = &client_msg.command;
                if  cmd.len() > 0 {
                    match cmd.as_str() {
                        "quitGame" => { 
                            done = true; 
                        },
                        "startGame" => { 
                            send_message("starting game".as_bytes());
                        }, 
                        "echo" => {
                            send_message(&cmd.as_bytes());
                        },
                        "getZoneGrid" => {
                            let res = get_zone_grid(&zone_grid);
                            send_message(res.as_bytes());
                        }, 
                        "getMoney" => {
                            send_message("0".as_bytes());
                        },
                        "getPeopleLocation" => {
                            let m = get_people_location(&pop_grid);
                            send_message(m.as_bytes());
                        },
                        "getTime" => {
                            send_message(logical_time.to_string().as_bytes());
                        },
                        "getRCINeed" => {
                            send_message("[0,4,2]".as_bytes());
                        },
                        "setZoneGrid" => {
                            let m = format!("set");
                            send_message(m.as_bytes());
                        },
                        _ => {
                            let m = format!("Unknown command '{}'!", &cmd);
                            send_message(m.as_bytes());
                        }
                    }
                }
            }
        }
       
        logical_time += 1;
        thread::sleep(sec); 
    }
}

#[derive(Debug)]
struct ClientMessage {
    // we could use an actual uuid type...
    uuid: String,
    command: String,
    arguments: Option<String>,

}

/// Convenience function for making Vector2
// @Copypaste
fn v2(x: usize, y: usize) -> Vector2 {
    Vector2 { x: x, y: y }
}

fn parse_client_message(re: &Regex, message: &String) -> Option<ClientMessage> {
    let caps_opt = re.captures(message);
    let mut res: Option<ClientMessage> = None;
    match caps_opt {
        Some(caps) => {
            if caps.len() == 5 {
                let uuid_opt = match  caps.get(1) {
                    Some(c) => Some(c.as_str().to_string()),
                    None => None,
                };
                let command_opt = match  caps.get(2) {
                    Some(c) => Some(c.as_str().to_string()),
                    None => None,
                };
                let arguments = match  caps.get(4) {
                    Some(c) => Some(c.as_str().to_string()),
                    None => None,
                };
                if uuid_opt != None && command_opt != None {
                    let uuid = uuid_opt.unwrap();
                    let command = command_opt.unwrap();
                    res = Some( ClientMessage { uuid, command, arguments });
                }
                
            }

        },
        None => {}
    }
    res
}

fn get_zone_grid(zone_grid: &ZoneGrid) -> String {
    let serialized = serde_json::to_string(&zone_grid);
    match serialized {
        Ok(s) => s,
        Err(_) => String::new(),
    }
}

fn get_people_location(pop_grid: &PopulationGrid) -> String {
    let serialized = serde_json::to_string(&pop_grid);
    match serialized {
        Ok(s) => s,
        Err(_) => String::new(),
    }
}
