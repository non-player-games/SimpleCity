extern crate regex;
extern crate systems;
extern crate serde_json;

use regex::Regex;
use systems::simulation::{PopulationGrid, RCINeed, SimulationManager, Vector2, Zone, ZoneGrid};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::io::{self, Write};
use std::{thread, time};


fn main() {
    println!("Starting systems ...");
    listen();
}

fn send_client_message(uuid: &String, message: &String){
    write_message(&format!("{} {}", uuid, message).as_bytes());
}

/// Sends a message to stdout
// We might end of scratching this out
fn write_message(message: &[u8]) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let wrote_msg = handle.write(message).is_ok();
    let wrote_newline = handle.write(b"\n").is_ok();
    let flushed_buffer = handle.flush().is_ok();

    if !(wrote_msg && wrote_newline && flushed_buffer){
        // @Todo send this to a logger
        println!("Error while writing message");
    }
}

fn listen(){
    let data: Arc<Mutex<VecDeque<String>>> = Arc::new(Mutex::new(VecDeque::new()));
    {
        let d = data.clone();
        // This thread is dedicated to reading from stdin because stdin blocks
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
    let grid_size = v2(16, 16);
    let mut sim_manager_opt: Option<SimulationManager> = None;
    let compiled_regex = CompiledRegex::init();
    //let command_regex: Regex = Regex::new(r"^([0-9a-f-]{36})\s+([a-zA-Z]+)(\s+(.+))?$").unwrap();
    while !done {
        // READ COMMANDS
        //{
        let mut messages_rcvd: Vec<String> = Vec::with_capacity(0);
        {
            if let Ok(mut messages_queue) = data.lock() {
                messages_rcvd = messages_queue.drain(..).collect();
            }
        }
        for message_rcvd in messages_rcvd {
            let client_msg_opt = parse_client_message(&compiled_regex.command_regex, &message_rcvd);
            if client_msg_opt.is_none(){
                continue;
            }
            // @Incomplete: we may want to move this elsewhere and make methods for
            // each command perhaps.
            // @Cleanup message format
            let client_msg = client_msg_opt.unwrap();
            let uuid = &client_msg.uuid;
            let cmd = &client_msg.command;
            // we need to check whether no commands matched despite there being a game in progress
            let mut matched_cmd_during_active_game = true && sim_manager_opt.is_some();
            if  cmd.len() > 0 {
                if let Some(ref mut sim_manager) = sim_manager_opt {
                    match cmd.as_str() {
                        "getZoneGrid" => {
                            let res = get_zone_grid(&sim_manager.zone_grid);
                            send_client_message(uuid, &res);
                        }, 
                        "getMoney" => {
                            let money = get_money(sim_manager.player_money);
                            send_client_message(uuid, &money);
                        },
                        "getPeopleLocation" => {
                            let m = get_people_location(&sim_manager.population_grid);
                            send_client_message(uuid, &m);
                        },
                        "getTime" => {
                            let time = get_time(sim_manager.time);
                            send_client_message(uuid, &time);
                        },
                        "getRCINeed" => {
                            let rci_need = get_rci_need(&sim_manager.rci_need);
                            send_client_message(uuid, &rci_need);
                        },
                        "setZoneGrid" => {
                            if client_msg.arguments.is_some() {
                                set_zone(&compiled_regex.set_zone_regex, &client_msg.arguments.unwrap(), &mut sim_manager.zone_grid);
                            } else {
                                let m = format!("ERR no arguments");
                                send_client_message(uuid, &m);
                            }
                            //let m = format!("OK set");
                            //send_client_message(uuid, &m);
                        },
                        _ => {
                            matched_cmd_during_active_game = false;
                        }
                    }
                }
                match cmd.as_str() {
                    "quitGame" => { 
                        if sim_manager_opt.is_some() {
                            send_client_message(uuid, &"OK quitting game".to_string());
                        } else {
                            send_client_message(uuid, &"ERR there is no game to quit".to_string());
                        }
                        sim_manager_opt = None;
                        continue;
                    },
                    "startGame" => { 
                        match sim_manager_opt {
                            Some(_) => send_client_message(uuid, &"ERR game already in progress; quitGame first".to_string()),
                            None => {
                                sim_manager_opt = Some(start_game(&grid_size));
                                send_client_message(uuid, &"OK starting game".to_string());
                            },
                        }
                    },
                    "shutdown" => {
                        send_client_message(uuid, &"OK shutting down simulation systems process".to_string());
                        done = true;
                        continue;
                    },
                    "gameExists" => {
                        send_client_message(uuid, &format!("{}", sim_manager_opt.is_some()));
                    },
                    _ => {
                        if !matched_cmd_during_active_game {
                            send_client_message(uuid, &format!("ERR Unknown command or no game in progress!"));
                        }
                    }
                }
            }
        }
        //}
        if let Some(ref mut sim_manager) = sim_manager_opt {
            sim_manager.advance_time(); 
        }
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


// Commands
fn start_game(dimensions_v2: &Vector2) -> SimulationManager {
    SimulationManager::new(dimensions_v2)
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

fn get_money(money: i64) -> String {
    money.to_string()
}

fn get_time(time: u64) -> String {
    time.to_string()
}

fn get_rci_need(rci_need: &RCINeed) -> String {
    match serde_json::to_string(&rci_need) {
        Ok(s) => s,
        Err(_) => String::new()
    }
}


fn set_zone(re: &Regex, args: &String, zone_grid: &mut ZoneGrid) {
    let caps_opt = re.captures(args);
    if let Some(caps) = caps_opt {
        if caps.len() == 4 {
            let x_opt    = caps.get(1).map(|c| c.as_str().parse::<usize>().ok()).unwrap_or(None);
            let y_opt    = caps.get(2).map(|c| c.as_str().parse::<usize>().ok()).unwrap_or(None);
            let zone_opt = caps.get(3).map(|c| serde_json::from_str(c.as_str()).unwrap_or(None)).unwrap_or(None);
            
            if let (Some(x), Some(y), Some(zone)) = (x_opt, y_opt, zone_opt) {
                zone_grid.set_zone(&v2(x, y), zone);
            }
        }

    }
}

struct CompiledRegex {
    command_regex: Regex,
    set_zone_regex: Regex,
}

impl CompiledRegex {
    fn init() -> CompiledRegex {
        let command_regex: Regex = Regex::new(r"^([0-9a-f-]{36})\s+([a-zA-Z]+)(\s+(.+))?$").unwrap();
        let set_zone_regex: Regex = Regex::new(r"^(\d+)\s(\d+)\s(\d+)$").unwrap();
        CompiledRegex {
            command_regex,
            set_zone_regex,
        }
    }
}
