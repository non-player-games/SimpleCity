use std::fmt;
use std::io::{self, Write};




fn main() {
    println!("Starting systems ...");
}

// @Refactor: we'll keep everything here for now 
// and move out once we get a better sense for our modules

/// Sends a message to stdout
fn send_message(message: &[u8]){
    let stdout = io::stdout();
    let mut handle =  stdout.lock();
    handle.write(message);
    handle.flush();
}

/// Convenience function for making Vector2
fn v2(x: usize, y: usize) -> Vector2{
    Vector2{x: x, y: y}
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
        PopulationGrid{zones: zones, size: grid_size}
    }

    fn get_zone(&self, location: &Vector2) -> Option<usize> {
        let width = self.size.x; 
        let height = self.size.y;
        let mut zone = None;
        if location.x < width && location.y < height {
            let index: usize = height * location.y + location.x;
            if index < self.zones.len(){
                zone = Some(self.zones[index]);
            }
        }
        zone
    }

    fn populationCount(&self) -> usize {
       self.zones.iter().sum();  
    }
}

impl fmt::Debug for PopulationGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in 0..self.size.x {
            for y in 0..self.size.y {
                let val = self.get_zone(&Vector2{x,y}).unwrap();
                write!(f, "{:?}", &val);
            }
            write!(f, "\n");
        }
        Ok(())
    }
}


struct ResidentialZone {
    name: String,
}

struct CommercialZone {
    name: String,
}

struct IndustrialZone {
    name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_location() {
        let x_high = 16;
        let y_high = 16;
        let dimensions = Vector2{x: x_high, y: y_high};
        let mut population_grid = PopulationGrid::new(dimensions);

        let zone_v2 = Vector2{x: 0, y: 0};
        let zone = population_grid.get_zone(&zone_v2);
        assert!(zone.is_some());
    }

    #[test]
    fn invalid_location() {
        let x_high = 16;
        let y_high = 16;
        let mut population_grid = PopulationGrid::new(Vector2{x: x_high, y: y_high});

        let zone_no_exist0 = population_grid.get_zone(&Vector2{x: x_high, y: y_high});
        assert!(!zone_no_exist0.is_some());

        let zone_no_exist1 = population_grid.get_zone(&Vector2{x: x_high * y_high + 100000, y: 0});
        assert!(!zone_no_exist1.is_some());
    }
}
