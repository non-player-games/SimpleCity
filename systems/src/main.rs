use std::sync::{Arc, Mutex};
use std::fmt;
use std::io::{self, Write};
use std::mem;
use std::{thread, time};

// @Refactor: we'll keep everything here for now
// and move out once we get a better sense for our modules
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
    let data = Arc::new(Mutex::new(String::from("")));
    {
        let d = data.clone();
        // This thread is dedicated to reading from stdin because stdin
        // is blocking
        thread::spawn(move || {
            let mut input = String::from("");
            loop {
                match io::stdin().read_line(&mut input) {
                    Ok(n) => {
                        input = input.trim().to_string();
                        let mut dd = d.lock().unwrap();
                        *dd = input.clone();
                        input = String::from("");
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
        thread::sleep(sec); 
        let mut cmd: String = String::from("");
        {
            let mut d = data.lock().unwrap();
            cmd = d.clone();
            *d = String::from("");
        }
        if cmd.len() > 0 {
            send_message(&cmd.as_bytes());
            if &cmd == "QUIT" {
                done = true;
            }
        } else {
            println!("No update");
        }
    }
}

/// A 2-dimensional vector
struct Vector2 {
    x: usize,
    y: usize,
}

impl fmt::Debug for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[x: {}, y: {}]", self.x, self.y)
    }
}

/// Convenience function for making Vector2
fn v2(x: usize, y: usize) -> Vector2 {
    Vector2 { x: x, y: y }
}

/// Grid that keeps track of the population in all of the zones
struct PopulationGrid {
    zones: Vec<usize>,
    size: Vector2,
}

impl PopulationGrid {
    /// Returns a new population grid of size Vector2.x * Vector2.y
    /// # Argumens
    /// * `grid_size` - a Vector2 representing the dimensions of the grid
    fn new(grid_size: Vector2) -> PopulationGrid {
        let zones: Vec<usize> = vec![0; grid_size.x * grid_size.y];
        PopulationGrid {
            zones: zones,
            size: grid_size,
        }
    }

    fn get_zone(&mut self, location: &Vector2) -> Option<&mut usize> {
        let width = self.size.x;
        let height = self.size.y;
        let mut zone = None;
        if location.x < width && location.y < height {
            let index: usize = height * location.y + location.x;
            if index < self.zones.len() {
                zone = Some(&mut self.zones[index]);
            }
        }
        zone
    }

    fn population_count(&self) -> usize {
        self.zones.iter().sum()
    }
}

impl fmt::Debug for PopulationGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, e) in self.zones.iter().enumerate() {
            if i > 0 && i % self.size.x == 0 {
                write!(f, "\n");
            }
            write!(f, "{:?}", e);
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
enum Zone {
    Residential,
    Commercial,
    Industrial,
    // @Refactor: Empty sounded like the right name for no land. But perhaps Open or Vacant are better.
    Empty,
}

/// Grid that keeps track of the population in all of the zones
struct ZoneGrid {
    zones: Vec<Zone>,
    size: Vector2,
}

impl ZoneGrid {
    /// Returns a new population grid of size Vector2.x * Vector2.y
    /// # Argumens
    /// * `grid_size` - a Vector2 representing the dimensions of the grid
    fn new(grid_size: Vector2) -> ZoneGrid {
        let zones: Vec<Zone> = vec![Zone::Empty; grid_size.x * grid_size.y];
        ZoneGrid {
            zones: zones,
            size: grid_size,
        }
    }

    fn get_zone(&mut self, location: &Vector2) -> Option<&mut Zone> {
        let width = self.size.x;
        let height = self.size.y;
        let mut zone = None;
        if location.x < width && location.y < height {
            let index: usize = height * location.y + location.x;
            if index < self.zones.len() {
                zone = Some(&mut self.zones[index]);
            }
        }
        zone
    }
}

impl fmt::Debug for ZoneGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, e) in self.zones.iter().enumerate() {
            if i > 0 && i % self.size.x == 0 {
                write!(f, "\n");
            }
            write!(f, "{:?} ", e);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_location() {
        let x_high = 16;
        let y_high = 16;
        let dimensions = Vector2 {
            x: x_high,
            y: y_high,
        };
        let mut population_grid = PopulationGrid::new(dimensions);

        let zone_v2 = Vector2 { x: 0, y: 0 };
        let zone = population_grid.get_zone(&zone_v2);
        assert!(zone.is_some());
    }

    #[test]
    fn invalid_location() {
        let x_high = 16;
        let y_high = 16;
        let mut population_grid = PopulationGrid::new(Vector2 {
            x: x_high,
            y: y_high,
        });

        let zone_no_exist0 = population_grid.get_zone(&Vector2 {
            x: x_high,
            y: y_high,
        });
        assert!(!zone_no_exist0.is_some());

        let zone_no_exist1 = population_grid.get_zone(&Vector2 {
            x: x_high * y_high + 100000,
            y: 0,
        });
        assert!(!zone_no_exist1.is_some());
    }
}
