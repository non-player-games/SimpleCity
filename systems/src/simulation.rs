extern crate serde;
extern crate serde_json;

use rand;
use rand::Rng;
use std::collections::{HashSet, VecDeque};
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
    pub tax_rate: f64,
}

impl SimulationManager {
   pub fn new(dimensions_v2: &Vector2) -> SimulationManager {
        // @Incomplete we'll assign starting money differently 
        let population_grid = PopulationGrid::new(dimensions_v2);
        let zone_grid = ZoneGrid::new(dimensions_v2);
        let size = dimensions_v2.clone();
        let time: u64 = 0;
        let player_money: u64 = 100;
        let tax_rate: f64 = 0.09;
        let rci_need = RCINeed { residential: 0, commercial: 0, industrial: 0 };
        SimulationManager {
            population_grid,
            zone_grid,
            size,
            time,
            player_money,
            rci_need,
            tax_rate,
        }
   }

    pub fn next_tick(&mut self) {
        self.time += 1;
        self.increase_population();
        self.collect_tax();
    }

    pub fn buy_zone(&mut self, location: &Vector2, new_zone: Zone) -> bool {
        let cost = match new_zone {
            Zone::Empty       => 0,
            Zone::Road        => 1,
            Zone::Residential => 2,
            Zone::Commercial  => 2,
            Zone::Industrial  => 2,
        };
        
        if cost > self.player_money { return false; }

        /* Validation. We allow the change to go through in the following cases
         * 1. The old zone differs than the new zone.
         * 2. Any of the following
         *   a. The new zone will be made EMPTY
         *   b. The old zone is EMPTY and will be a road
         *   c. The old zone is EMPTY  and the new zone is not a ROAD and there is at least 1 adjacent road
        */
        let mut can_set = false;
        let adj_to_road = self.zone_grid.adjacent_to_road(location);
        if let Some(old_zone) = self.zone_grid.get_zone(location) {
            let zone_changing = *old_zone != new_zone;
            let make_empty    = new_zone  == Zone::Empty;
            let make_road     = *old_zone == Zone::Empty && new_zone == Zone::Road;
            let make_rci      = *old_zone == Zone::Empty && new_zone != Zone::Road && adj_to_road;
            can_set = zone_changing && make_empty || make_road || make_rci;
        }

        if !can_set { return false; }

        let was_zone_set = self.zone_grid.set_zone(location, &new_zone);
        if was_zone_set {
            self.player_money -= cost;
        }
        // TEMP
        self.zone_grid.district();
        return was_zone_set;
    }

    pub fn money(&self) -> u64 {
        self.player_money
    }

    fn increase_population(&mut self) {
        let pop_grid = &mut self.population_grid;
        let zone_grid = &self.zone_grid;
        let pop_count = pop_grid.population_count();
        let pop_increase = 2 + (pop_count as f64 * 0.05) as u64;
        let mut rng = rand::thread_rng();
        // Only residential for now. 
        let residential = zone_grid.get_zone_residential();
        for _ in 0..pop_increase {
            let index_opt = rng.choose(&residential);
            if let Some(index) = index_opt {
                // @Robust: use get_zone method
                if let Some(zone) = pop_grid.zones.get_mut(*index) {
                    if *zone < MAX_ZONE_POPULATION {
                        *zone += 1;
                    } 
                }
            }
        }

    }

    fn collect_tax(&mut self) {
        if self.time % 10 == 0 {
            let tax: f64 = self.population_grid.population_count() as f64 * (self.tax_rate + 1.0);
            self.player_money += tax.round() as u64;
        }
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

    pub fn adjacent_to_road(&self, pos: &Vector2) -> bool {
        let width = self.size.x;
        let height = self.size.y;
        if pos.x >= width || pos.y >= height {
            return false;
        }

        let index: usize = height * pos.y + pos.x;
        if index >= self.zones.len() {
            return false;
        }

        let above_index: isize = index as isize - width as isize;
        let above: Option<Zone> = if above_index >= 0 {
            Some(self.zones[above_index as usize].clone())
        } else {
            None
        };

        let below_index: usize = index + width;
        let below: Option<Zone> = if below_index < self.zones.len() {
            Some(self.zones[below_index].clone())
        } else {
            None
        };
        

        let left: Option<Zone> = if index % width != 0 {
            Some(self.zones[index - 1].clone())
        } else {
            None
        };

        let right: Option<Zone> = if index % width != width - 1 {
            Some(self.zones[index + 1].clone())
        } else {
            None
        };

        let road: Option<Zone> = Some(Zone::Road);
        above == road || below == road || left == road || right == road
    }

    pub fn district(&self) -> Vec<HashSet<usize>> {
        // We need to collect all the adjacent roads into districts
        let mut visited: HashSet<usize> = HashSet::new();
        let mut districts: Vec<HashSet<usize>> = Vec::new();

        for (i, z) in self.zones.iter().enumerate(){
            if visited.contains(&i){ 
                continue; 
            }
            // questionable?
            visited.insert(i);

            if z == &Zone::Road {
                // new district
                eprintln!("Checking {}", i);
                let mut roads_to_visit: VecDeque<usize> = VecDeque::new();
                roads_to_visit.push_back(i);
                let mut district: HashSet<usize> = HashSet::new();
                district.insert(i);
                while roads_to_visit.len() > 0 {
                    // Normally, we might put this into a recursive call
                    // to get all the adjacent of the adjacent
                    let current = roads_to_visit.pop_front().unwrap();
                    let adj_zones = self.adjacent_zones(current);
                    eprintln!("Current: {}, adjacent: {:?}", current, &adj_zones);
                    for adj_z_index in adj_zones {
                        let zone_type = &self.zones[adj_z_index];
                        if !visited.contains(&adj_z_index) && zone_type == &Zone::Road {
                            roads_to_visit.push_back(adj_z_index);
                            district.insert(adj_z_index);
                            visited.insert(adj_z_index);
                        }
                    }
                    eprintln!("Roads to visit: {:?}", &roads_to_visit);

                }
                districts.push(district);
                // put all into districts
            }
        }
        // DEBUG
        #[cfg(debug_assertions)]
        {
            eprintln!("DEBUGGIN");
            for (i, d) in districts.iter().enumerate() {
                eprintln!("{}: {:?}", i, d);
            }
            for (i, e) in self.zones.iter().enumerate() {
                let width = self.size.x;
                if i % width == 0 { 
                    eprint!("\n"); 
                }
                eprint!("{:?}\t", e);
            }
        }

        districts
    }

    fn adjacent_zones(&self, index: usize) -> Vec<usize>{
        let width = self.size.x;
        let height = self.size.y;
        let mut adjacent: Vec<usize> = Vec::new();
        if index >= width || index >= height{
            return adjacent;
        }
        let above_index: isize = index as isize - width as isize;
        if above_index >= 0 { 
            adjacent.push(above_index as usize); 
        }

        let below_index: usize = index + width;
        if below_index < self.zones.len() {
            adjacent.push(below_index);
        };
        

        if index % width != 0 {
            adjacent.push(index - 1);
        }

        if index % width != width - 1 {
            adjacent.push(index + 1);
        }
        eprintln!("{} is adjacent to {:?}", index, &adjacent);
        adjacent
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
    Empty       = 0,
    Road        = 1,
    Residential = 2,
    Commercial  = 3,
    Industrial  = 4,
});
