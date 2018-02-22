extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use std::fmt;
use std::io::{self, Write};
use std::mem;
use std::{thread, time};


/// A 2-dimensional vector
#[derive(Serialize, Deserialize)]
pub struct Vector2 {
    pub x: usize,
    pub y: usize,
}

impl fmt::Debug for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[x: {}, y: {}]", self.x, self.y)
    }
}

/// Convenience function for making Vector2
// @Copypaste
fn v2(x: usize, y: usize) -> Vector2 {
    Vector2 { x: x, y: y }
}

/// Grid that keeps track of the population in all of the zones
pub struct PopulationGrid {
    zones: Vec<usize>,
    size: Vector2,
}

impl PopulationGrid {
    /// Returns a new population grid of size Vector2.x * Vector2.y
    /// # Argumens
    /// * `grid_size` - a Vector2 representing the dimensions of the grid
    pub fn new(grid_size: Vector2) -> PopulationGrid {
        let zones: Vec<usize> = vec![0; grid_size.x * grid_size.y];
        PopulationGrid {
            zones: zones,
            size: grid_size,
        }
    }

    pub fn get_zone(&mut self, location: &Vector2) -> Option<&mut usize> {
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

    pub fn population_count(&self) -> usize {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Zone {
    // @Refactor: Empty sounded like the right name for no land. But perhaps Open or Vacant are better.
    Empty = 0,
    Residential = 1,
    Commercial = 2,
    Industrial = 3,
}

/// Grid that keeps track of the population in all of the zones
#[derive(Serialize, Deserialize)]
pub struct ZoneGrid {
    zones: Vec<Zone>,
    size: Vector2,
}

impl ZoneGrid {
    /// Returns a new population grid of size Vector2.x * Vector2.y
    /// # Argumens
    /// * `grid_size` - a Vector2 representing the dimensions of the grid
    // @Copypaste
    pub fn new(grid_size: Vector2) -> ZoneGrid {
        let zones: Vec<Zone> = vec![Zone::Empty; grid_size.x * grid_size.y];
        ZoneGrid {
            zones: zones,
            size: grid_size,
        }
    }

    pub fn get_zone(&mut self, location: &Vector2) -> Option<&mut Zone> {
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
