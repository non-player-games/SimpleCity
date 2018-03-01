extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use std::fmt;
use std::io::{self, Write};
use std::mem;
use std::{thread, time};

/// A 2-dimensional vector
#[derive(Clone, Serialize, Deserialize)]
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

#[derive(Debug)]
pub struct SimulationManager {
    pub population_grid: PopulationGrid,
    pub zone_grid: ZoneGrid,
    pub size: Vector2,
    pub time: u64,
    pub player_money: i64,
    pub rci_need: RCINeed,
}

impl SimulationManager {
   pub fn new(dimensions_v2: &Vector2) -> SimulationManager {
        // @Incomplete we'll assign starting money differently 
        let population_grid = PopulationGrid::new(dimensions_v2);
        let zone_grid = ZoneGrid::new(dimensions_v2);
        let size = dimensions_v2.clone();
        let time: u64 = 0;
        let player_money: i64 = 100;
        let rci_need = RCINeed { residential: 0, commercial: 0, industrial: 0 };
        SimulationManager {
            population_grid,
            zone_grid,
            size,
            time,
            player_money,
            rci_need,
        }
   }

   pub fn advance_time(&mut self) {
        self.time += 1;
   }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RCINeed {
    residential: i64,
    commercial: i64,
    industrial: i64,
}

/// Grid that keeps track of the population in all of the zones
#[derive(Serialize, Deserialize)]
pub struct PopulationGrid {
    zones: Vec<usize>,
    size: Vector2,
}

impl PopulationGrid {
    /// Returns a new population grid of size Vector2.x * Vector2.y
    /// # Argumens
    /// * `grid_size` - a Vector2 representing the dimensions of the grid
    pub fn new(grid_size: &Vector2) -> PopulationGrid {
        let zones: Vec<usize> = vec![0; grid_size.x * grid_size.y];
        PopulationGrid {
            zones: zones,
            size: grid_size.clone(),
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

/*
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Zone {
    // @Refactor: Empty sounded like the right name for no land. But perhaps Open or Vacant are better.
    Empty,
    Residential,
    Commercial,
    Industrial,
}
*/


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
    pub fn new(grid_size: &Vector2) -> ZoneGrid {
        let zones: Vec<Zone> = vec![Zone::Empty; grid_size.x * grid_size.y];
        ZoneGrid {
            zones: zones,
            size: grid_size.clone(),
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

    pub fn set_zone(&mut self, location: &Vector2, new_zone: Zone) {
        let old_zone_opt = self.get_zone(location);
        if let Some(old_zone) = old_zone_opt {
            mem::replace(old_zone, new_zone); 
        }
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
        let mut population_grid = PopulationGrid::new(&dimensions);

        let zone_v2 = Vector2 { x: 0, y: 0 };
        let zone = population_grid.get_zone(&zone_v2);
        assert!(zone.is_some());
    }

    #[test]
    fn invalid_location() {
        let x_high = 16;
        let y_high = 16;
        let mut population_grid = PopulationGrid::new(&Vector2 {
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




/*  
 *  Create an Enum that serializes to a number
 *  Example Taken from https://serde.rs/enum-number.html
*/

macro_rules! enum_number {
    ($name:ident { $($variant:ident = $value:expr, )* }) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum $name {
            $($variant = $value,)*
        }

        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: serde::Serializer
            {
                // Serialize the enum as a u64.
                serializer.serialize_u64(*self as u64)
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: serde::Deserializer<'de>
            {
                struct Visitor;

                impl<'de> serde::de::Visitor<'de> for Visitor {
                    type Value = $name;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("positive integer")
                    }

                    fn visit_u64<E>(self, value: u64) -> Result<$name, E>
                        where E: serde::de::Error
                    {
                        // Rust does not come with a simple way of converting a number to an enum, so use a big `match`.
                        match value {
                            $( $value => Ok($name::$variant), )*
                            _ => Err(E::custom(
                                format!("unknown {} value: {}",
                                stringify!($name), value))),
                        }
                    }
                }

                // Deserialize the enum from a u64.
                deserializer.deserialize_u64(Visitor)
            }
        }
    }
}

enum_number!(Zone {
    Empty = 0,
    Residential = 1,
    Commercial = 2,
    Industrial = 3,
});
