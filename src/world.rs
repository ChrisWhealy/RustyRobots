use std::vec::Vec;
use std::str;
use std::io::prelude::*;
use std::io::BufReader;

use crate::location::Location;
use crate::heading::Heading;

pub const WORLD_MIN_WIDTH  : i32 = 1;
pub const WORLD_MIN_HEIGHT : i32 = 1;
pub const WORLD_MAX_WIDTH  : i32 = 50;
pub const WORLD_MAX_HEIGHT : i32 = 50;

const PARSE_ERROR_MISSING_DIMS : &str = &"Please specify world dimensions";
const PARSE_ERROR_MISSING_DIM  : &str = &"Expecting two world dimensions, only found one";
const PARSE_ERROR_BAD_WIDTH    : &str = &"World width must be an integer";
const PARSE_ERROR_BAD_HEIGHT   : &str = &"World height must be an integer";

const ERROR_INVALID_WORLD_DIMS : &str = &"Both world dimensions must be in the range 1 to 50";

pub const PROMPT_NEW_WORLD : &str = &"Enter width and height of world";
pub const EOF_ENCOUNTERED  : &str = &"EOF stdin";

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
  pub fn is_location_occupied(&self, x : &i32, y : &i32) -> bool {
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

// *********************************************************************************************************************
// Create a new world from stdin data
// *********************************************************************************************************************
pub fn create_world() -> Result<World, &'static str> {
  // Keep reading stdin until we get some valid world dimensions or hit EOF
  prompt(PROMPT_NEW_WORLD);

  for stdin_data in BufReader::new(std::io::stdin()).lines() {
    match &stdin_data.unwrap().trim().parse::<Dimensions>() {
      Ok(dims)     => return Ok(World::new(&dims.width, &dims.height))
    , Err(err_msg) => {
        eprintln!("Error: {}", err_msg);
        prompt(PROMPT_NEW_WORLD);
      }
    }
  }

  Err(EOF_ENCOUNTERED)
}

// *********************************************************************************************************************
// World dimensions definition
// *********************************************************************************************************************
#[derive(Debug)]
pub struct Dimensions {
  pub width  : i32
, pub height : i32
}

// *********************************************************************************************************************
// World dimensions implementation
// *********************************************************************************************************************
impl str::FromStr for Dimensions {
  type Err = &'static str;

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Parse line from stdin that we expect to contain world dimensions: width then height
  fn from_str(s: &str) -> Result<Dimensions, Self::Err> {
    let mut line_iter = s.split_ascii_whitespace();

    let w = match line_iter.next() {
      Some(val) =>
        match val.parse::<i32>() {
          Ok(int_val) => int_val
        , Err(_)      => return Err(PARSE_ERROR_BAD_WIDTH)
        }
    , None => return Err(PARSE_ERROR_MISSING_DIMS)
    };

    let h = match line_iter.next() {
      Some(val) => 
      match val.parse::<i32>() {
        Ok(int_val) => int_val
      , Err(_)      => return Err(PARSE_ERROR_BAD_HEIGHT)
      }
    , None => return Err(PARSE_ERROR_MISSING_DIM)
    };

    // Check that the parsed dimensions are within the permitted range
    if w >= WORLD_MIN_WIDTH  && w <= WORLD_MAX_WIDTH &&
       h >= WORLD_MIN_HEIGHT && h <= WORLD_MAX_HEIGHT { 
        Ok(Dimensions{ width : w, height : h })
    }
    else {
      Err(ERROR_INVALID_WORLD_DIMS)
    }
  }
}

// *********************************************************************************************************************
// Private API
// *********************************************************************************************************************
fn index_from_x_y(height : &i32, x : &i32, y : &i32) -> usize {
  (y * height + x) as usize  
}

fn create_world_locations(width : &i32, height : &i32) -> Vec<Location> {
  let mut w : Vec<Location> = vec!();

  for i in 0..*height {
    for j in 0..*width {
      w.push(Location::new(j, i));
    }
  }

  w
}

fn prompt(prompt_msg : &str) {
  print!("{} : ", prompt_msg);
  let _ = std::io::stdout().flush();
}

// *********************************************************************************************************************
// Suppose we'd better test it...
// *********************************************************************************************************************
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_test_valid_dims() {
    // Minimum valid dimensions
    let d = format!("{} {}", WORLD_MIN_WIDTH, WORLD_MIN_HEIGHT).parse::<Dimensions>().unwrap();
    assert_eq!(&d.width, &WORLD_MIN_WIDTH);
    assert_eq!(&d.height, &WORLD_MIN_HEIGHT);

    // Midpoint valid dimensions
    let test_width  = ((WORLD_MAX_WIDTH  - WORLD_MIN_WIDTH)  as f32 / 2.0).ceil() as i32;
    let test_height = ((WORLD_MAX_HEIGHT - WORLD_MIN_HEIGHT) as f32 / 2.0).ceil() as i32;
    let d = format!("{} {}", test_width, test_height).parse::<Dimensions>().unwrap();
    assert_eq!(&d.width, &test_width);
    assert_eq!(&d.height, &test_height);

    // Maximum valid dimensions
    let d = format!("{} {}", WORLD_MAX_WIDTH, WORLD_MAX_HEIGHT).parse::<Dimensions>().unwrap();
    assert_eq!(&d.width, &WORLD_MAX_WIDTH);
    assert_eq!(&d.height, &WORLD_MAX_HEIGHT);
  }  

  #[test]
  fn parse_test_invalid_dims() {
    // Both dimensions missing
    let d = "".parse::<Dimensions>();
    assert_eq!(d.err(), Some(PARSE_ERROR_MISSING_DIMS));

    // One valid dimension, but should be two
    let d = "1".parse::<Dimensions>();
    assert_eq!(d.err(), Some(PARSE_ERROR_MISSING_DIM));

    // One invalid dimension
    let d = "a".parse::<Dimensions>();
    assert_eq!(d.err(), Some(PARSE_ERROR_BAD_WIDTH));

    // Two dimensions, but the first one is invalid
    let d = "a 1".parse::<Dimensions>();
    assert_eq!(d.err(), Some(PARSE_ERROR_BAD_WIDTH));

    // Two dimensions, but the second one is invalid
    let d = "1 b".parse::<Dimensions>();
    assert_eq!(d.err(), Some(PARSE_ERROR_BAD_HEIGHT));
    
    // Both dimensions parse correctly but at least one is invalid
    let d = "0 0".parse::<Dimensions>();
    assert_eq!(d.err(), Some(ERROR_INVALID_WORLD_DIMS));

    // Both dimensions parse correctly but at least one is invalid
    let d = "-1 -1".parse::<Dimensions>();
    assert_eq!(d.err(), Some(ERROR_INVALID_WORLD_DIMS));

    // Both dimensions parse correctly but at least one is invalid
    let d = "25 51".parse::<Dimensions>();
    assert_eq!(d.err(), Some(ERROR_INVALID_WORLD_DIMS));

    // Both dimensions parse correctly but at least one is invalid
    let d = "52 50".parse::<Dimensions>();
    assert_eq!(d.err(), Some(ERROR_INVALID_WORLD_DIMS));
  }

  #[test]
  fn create_world_test_valid_dims() {
    // Minimum valid dimensions
    let world = World::new(&WORLD_MIN_WIDTH, &WORLD_MIN_HEIGHT);

    assert_eq!(&world.width, &WORLD_MIN_WIDTH);
    assert_eq!(&world.height, &WORLD_MIN_HEIGHT);

    // Midpoint valid dimensions
    let test_width  = ((WORLD_MAX_WIDTH  - WORLD_MIN_WIDTH)  as f32 / 2.0).ceil() as i32;
    let test_height = ((WORLD_MAX_HEIGHT - WORLD_MIN_HEIGHT) as f32 / 2.0).ceil() as i32;
    let world       = World::new(&test_width, &test_height);

    assert_eq!(&world.width, &test_width);
    assert_eq!(&world.height, &test_height);

    // Maximum valid dimensions
    let world = World::new(&WORLD_MAX_WIDTH, &WORLD_MAX_HEIGHT);

    assert_eq!(&world.width, &WORLD_MAX_WIDTH);
    assert_eq!(&world.height, &WORLD_MAX_HEIGHT);
  }  
}