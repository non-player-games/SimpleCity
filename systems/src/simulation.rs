extern crate serde;
extern crate serde_json;

use rand;
use rand::Rng;
use std::fmt;
use std::mem;

/// A 2-dimensional vector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector2 {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone)]
pub struct SimulationManager {
    pub population_grid: PopulationGrid,
    pub zone_grid: ZoneGrid,
    pub size: Vector2,
    pub time: u64,
    pub player_money: u64,
    pub rci_need: RCINeed,
}

impl SimulationManager {
   pub fn new(dimensions_v2: &Vector2) -> SimulationManager {
        // @Incomplete we'll assign starting money differently 
        let population_grid = PopulationGrid::new(dimensions_v2);
        let zone_grid = ZoneGrid::new(dimensions_v2);
        let size = dimensions_v2.clone();
        let time: u64 = 0;
        let player_money: u64 = 100;
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

    pub fn buy_zone(&mut self, location: &Vector2, new_zone: Zone) -> bool {
        let cost = match new_zone {
            Zone::Empty => 0,
            Zone::Residential => 2,
            Zone::Commercial => 2,
            Zone::Industrial => 2,
        };

        if cost <= self.player_money {
            let mut can_set = false;
            if let Some(z) = self.zone_grid.get_zone(location) {
                can_set = *z != Zone::Empty && new_zone == Zone::Empty || *z == Zone::Empty;
            }
            if can_set {
                let was_zone_set = self.zone_grid.set_zone(location, &new_zone);
                if was_zone_set {
                    self.player_money -= cost;
                }
                return was_zone_set;
            }
        }
        false
    }

    // this method should call all of the life cycle methods for all of the grids
    pub fn update_lifecycles(&mut self) {
        increase_population(&mut self.population_grid, &self.zone_grid);
    }

    pub fn money(&self) -> u64 {
        self.player_money
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RCINeed {
    residential: i64,
    commercial: i64,
    industrial: i64,
}


// The maximum allowed population in a single residential zone
static MAX_ZONE_POPULATION: usize = 50;

/// Grid that keeps track of the population in all of the zones
#[derive(Debug, Clone, Serialize, Deserialize)]
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


/// Grid that keeps track of the population in all of the zones
#[derive(Debug, Clone, Serialize, Deserialize)]
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

    pub fn set_zone(&mut self, location: &Vector2, new_zone: &Zone) -> bool {
        let old_zone_opt = self.get_zone(location);
        if let Some(old_zone) = old_zone_opt {
            mem::replace(old_zone, *new_zone); 
            return true
        }
        return false
    }

    // @Incomeplete: we can make this a more general type of function
    // that accepts a zone type
    pub fn get_zone_residential(&self) -> Vec<usize> {
        let mut indexes = vec!();
        for (i,z) in self.zones.iter().enumerate() {
            if let &Zone::Residential = z {
                indexes.push(i as usize);
            }
        }
        indexes
    }
    
}

fn increase_population(pop_grid: &mut PopulationGrid, zone_grid: &ZoneGrid) {
    let pop_count = pop_grid.population_count();
    let pop_increase = 2 + (pop_count as f64 * 0.05) as u64;
    let mut rng = rand::thread_rng();
    // Only residential for now. 
    let residential = zone_grid.get_zone_residential();
    for _ in 0..pop_increase {
        let index = rng.choose(&residential);
        if let Some(e) = index {
            // @Robust: use get_zone method
            if let Some(z) = pop_grid.zones.get_mut(*e) {
                if *z < MAX_ZONE_POPULATION {
                    *z += 1;
                } 
            }
        }
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
