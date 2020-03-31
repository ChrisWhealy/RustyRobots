use std::vec::Vec;
use std::fmt;

use crate::location::Location;

#[derive(Debug, PartialEq, Clone)]
pub enum Heading {
  North
, South
, East
, West
}

impl fmt::Display for Heading {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let txt = match self {
      Heading::North => &"N"
    , Heading::South => &"S"
    , Heading::East  => &"E"
    , Heading::West  => &"W"
    };
    write!(f, "{}", txt)
}}

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
  pub fn is_it_safe(&self, x : &i32, y : &i32, heading : &Heading) -> bool {
    let loc = &self.locations[index_from_x_y(&self.height, &x, &y)];

    match heading {
      Heading::North => loc.can_go_north,
      Heading::East  => loc.can_go_east,
      Heading::South => loc.can_go_south,
      Heading::West  => loc.can_go_west
    }
  }

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Going that way was a bad idea...
  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  pub fn here_be_monsters(&mut self, x : &i32, y : &i32, heading : &Heading) {
    let loc = &mut self.locations[index_from_x_y(&self.height, &x, &y)];

    match heading {
      Heading::North => loc.can_go_north = false,
      Heading::East  => loc.can_go_east  = false,
      Heading::South => loc.can_go_south = false,
      Heading::West  => loc.can_go_west  = false
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