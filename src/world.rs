use std::vec::Vec;

use crate::location::Location;

// *********************************************************************************************************************
// World definition
// *********************************************************************************************************************
#[derive(Debug)]
pub struct World {
  pub width     : i32
, pub height    : i32
,     locations : Vec<Location>
}

// *********************************************************************************************************************
// World implementation
// *********************************************************************************************************************
impl World {
  pub fn is_location_occupied(&self, x : i32, y : i32) -> bool {
    &self.locations[index_from_x_y(&self.height, &x, &y)].id != &-1
  }

  pub fn place_robot_at(&mut self, robot_id : &i32, x : &i32, y : &i32) {
    self.locations[index_from_x_y(&self.height, &x, &y)].id = *robot_id;
  }

  pub fn remove_robot_from(&mut self, x : &i32, y : &i32) {
    self.locations[index_from_x_y(&self.height, &x, &y)].id = -1;
  }

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Should I go that way?
  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  pub fn is_it_safe(&self, x : &i32, y : &i32, heading : &char) -> bool {
    let loc = &self.locations[index_from_x_y(&self.height, &x, &y)];

    match heading {
      'N' => loc.can_go_north,
      'E' => loc.can_go_east,
      'S' => loc.can_go_south,
      'W' => loc.can_go_west,
      _   => false
    }
  }

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Going that way was a bad idea...
  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  pub fn here_be_monsters(&mut self, x : &i32, y : &i32, heading : &char) {
    let loc = &mut self.locations[index_from_x_y(&self.height, &x, &y)];

    match heading {
      'N' => loc.can_go_north = false,
      'E' => loc.can_go_east  = false,
      'S' => loc.can_go_south = false,
      'W' => loc.can_go_west  = false,
      _   => ()
    }
  }

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Constructor
  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  pub fn new(width : &i32, height : &i32) -> World {
    return World {
      height    : *height
    , width     : *width
    , locations : create_world_locations(&width, &height)
    }
  }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Utilities
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn index_from_x_y(height : &i32, x : &i32, y : &i32) -> usize {
  (y * height + x) as usize  
}

fn create_world_locations(width : &i32, height : &i32) -> Vec<Location> {
  let mut w : Vec<Location> = vec!();

  for i in 0..*height+1 {
    for j in 0..*width+1 {
      w.push(Location::new(j, i));
    }
  }

  w
}