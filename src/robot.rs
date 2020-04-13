use std::{str, fmt};
use std::io::prelude::*;
use std::io::BufReader;

use crate::heading::Heading;
use crate::trace::Trace;

use crate::world::{
  World
, WORLD_MAX_HEIGHT
, WORLD_MIN_HEIGHT
, WORLD_MAX_WIDTH
, WORLD_MIN_WIDTH
};

const LIB_NAME     : &str  = module_path!();
const TRACE_ACTIVE : &bool = &false;

const HEADINGS_LEFT :[Heading; 4] = [Heading::North, Heading::West, Heading::South, Heading::East];
const HEADINGS_RIGHT:[Heading; 4] = [Heading::North, Heading::East, Heading::South, Heading::West];

const PARSE_ERROR_MISSING_VALS  : &str = &"Please specify the new robot's X Y location and its heading";
const PARSE_ERROR_MISSING_Y_VAL : &str = &"Expecting the robot's Y location and a heading, but found only its X location";
const PARSE_ERROR_MISSING_HDNG  : &str = &"Expecting the new robot's heading, but found only its X Y location";
const PARSE_ERROR_BAD_X_VAL     : &str = &"New robot's X location must be an integer";
const PARSE_ERROR_BAD_Y_VAL     : &str = &"New robot's Y location must be an integer";

const ERROR_OUTSIDE_WORLD_BOUNDS : &str = &"Robot location lies outside permissible world boundaries";

pub const PROMPT_NEW_ROBOT : &str = &"Enter the zero-based location and heading for a new robot";
pub const PROMPT_MOVE_TURN : &str = &"Enter move/turn instructions";
pub const EOF_ENCOUNTERED  : &str = &"EOF stdin";

// *********************************************************************************************************************
// Robot definition
// *********************************************************************************************************************
#[derive(Debug)]
pub struct Robot {
  pub id      : i32
, pub x       : i32
, pub y       : i32
, pub heading : Heading
, pub is_lost : bool
}

impl fmt::Display for Robot {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    write!(fmt, "{} {} {}{}", self.x, self.y, self.heading, if self.is_lost { " LOST" } else { "" })
  }
}

// *********************************************************************************************************************
// Robot implementation
// *********************************************************************************************************************
impl Robot {
  pub fn turn_right(&mut self) {
    self.heading = (*turn(&HEADINGS_RIGHT, &self.heading)).clone();
    Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME, &"turn_right")(&format!("New heading = {}", &self.heading));
  }

  pub fn turn_left(&mut self) {
    self.heading = (*turn(&HEADINGS_LEFT, &self.heading)).clone();
    Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME, &"turn_left")(&format!("New heading = {}", &self.heading));
  }

  pub fn position(&mut self) -> (&i32, &i32) {
    Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME, &"position")
                        (&format!("Robot {} at ({},{}) heading {}", &self.id, &self.x, &self.y, &self.heading));
    (&self.x, &self.y)
  }

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Take a step forwards
  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  pub fn forward(&mut self, world : &mut World) {
    const FN_NAME : &str = &"forward";

    let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);
    let trace          = Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);

    trace_boundary(&Some(true));

    // Check whether any previous robot has died by venturing in this direction from this location
    if world.is_it_safe(&self.x, &self.y, &self.heading) {
      trace(&format!("It appears safe to head {} from ({},{})", &self.heading, &self.x, &self.y));

      let (new_x, new_y) = match self.heading {
        Heading::North => (self.x,     self.y + 1),
        Heading::East  => (self.x + 1, self.y),
        Heading::South => (self.x,     self.y - 1),
        Heading::West  => (self.x - 1, self.y),
      };

      // Does the new location lie inside the world?
      if (new_y < 0 || new_y >= world.height) ||
         (new_x < 0 || new_x >= world.width) {
        // Nope - KABOOM!
        trace(&"Ouch! Just been eaten by monsters!");
        // The robot is now lost so remove it from the world, warn other robots not to venture this way,
        // but don't update its x and y values because its last known location needs to be printed
        self.is_lost = true;
        world.remove_robot_from(&self.x, &self.y);
        world.here_be_monsters(&self.x, &self.y, &self.heading);
      }
      else {
        // Is the proposed location already occupied?
        if world.is_location_occupied(&new_x, &new_y) {
          // Yup, so ignore this instruction
          eprintln!("Can't go {} from ({},{}) - location already occupied!", &self.heading, &new_x, &new_y);
        }
        else {
          // Nope, so update the robot's position and update the world grid
          world.remove_robot_from(&self.x, &self.y);
          self.x = new_x;
          self.y = new_y;
          world.place_robot_at(&self.id, &self.x, &self.y);
          trace(&format!("Robot {} is now at ({},{}) heading {}", &self.id, &self.x, &self.y, &self.heading));
        }
      }
    
    }
    else {
      trace(&format!("Ignoring instruction to head {} from ({},{}) - here be monsters!", &self.heading, &self.x, &self.y));
    }

    trace_boundary(&Some(false));
  }

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Obey a set of move/turn instructions
  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  pub fn turn_and_move(&mut self, line : &str, world : &mut World) {
    const FN_NAME : &str = &"turn_and_move";

    let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);
    let trace          = Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);

    trace_boundary(&Some(true));

    let mut line_iter = line.split_ascii_whitespace();

    for c in line_iter.next().unwrap().to_ascii_uppercase().chars() {
      // If I died as a result of following a previous instruction, then bail out
      if self.is_lost {
        break;
      }
      else {
        // Pass commands to robot
        match c {
          'R' => self.turn_right(),
          'L' => self.turn_left(),
          'F' => self.forward(world),
          _z  => trace(&format!("Ignoring invalid move/turn command '{}'", _z))
        }
      }
    }

    trace_boundary(&Some(false));
  }

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Constructor
  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  pub fn new(id : i32, x : i32, y : i32, heading : Heading) -> Robot {
    Robot{
      id
    , x
    , y
    , heading
    , is_lost : false
    }
  }
}

// *********************************************************************************************************************
// Create a new robot from stdin data
// *********************************************************************************************************************
pub fn create_robot(line_arg : &str, world : &mut World, robot_id : &i32) -> Result<Robot, &'static str> {
  const FN_NAME : &str = &"create_robot";

  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);
  let trace          = Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);

  trace_boundary(&Some(true));

  let mut stdin_data : Vec<u8> = Vec::new();
  let mut stdin = BufReader::new(std::io::stdin());
  let mut line  = line_arg.clone();

  // Keep reading stdin until we get some valid robot data
  loop {
    // Parse stdin data to see if its a valid robot
    match line.parse::<Robot>() {
      Ok(mut robot) => {
        // Check that new location is within the world's boundaries
        if robot.x < world.width  &&
           robot.y < world.height {
          // Does the proposed location already contain a robot?
          if world.is_location_occupied(&robot.x, &robot.y) {
            eprintln!("ERROR: Cannot create robot at location ({},{}) - already occupied", robot.x, robot.y);
          }
          else {
            // The robot's location is valid, so assign it the next id and place it at that world location
            robot.id = *robot_id;
            world.place_robot_at(&robot_id, &robot.x, &robot.y);
            trace(&format!("New robot created at ({},{}) heading {}", robot.x, robot.y, robot.heading));
            trace_boundary(&Some(false));
            return Ok(robot)
          }
        }
        else {
          eprintln!("ERROR: Cannot create robot at location ({},{}) - outside world boundaries", robot.x, robot.y);
        }
     }
    , Err(err_msg) => eprintln!("Error: {}", err_msg)
    }

    // Wait for next line from stdin
    stdin_data.clear();
    prompt(PROMPT_NEW_ROBOT);

    if stdin.read_until(b'\n', &mut stdin_data).unwrap() == 0 {
      return Err(EOF_ENCOUNTERED);
    }

    line = str::from_utf8(&stdin_data).unwrap().trim();
  }
}

// *********************************************************************************************************************
// Parser for robot data received from stdin
// *********************************************************************************************************************
impl str::FromStr for Robot {
  type Err = &'static str;

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Parse line from stdin that we expect to contain a new robot definition
  fn from_str(s: &str) -> Result<Robot, Self::Err> {
    let mut line_iter = s.split_ascii_whitespace();

    let x = match line_iter.next() {
      Some(val) => match val.parse::<i32>() {
        Ok(int_val) => int_val
      , Err(_)      => return Err(PARSE_ERROR_BAD_X_VAL)
      }
    , None => return Err(PARSE_ERROR_MISSING_VALS)
    };

    let y = match line_iter.next() {
      Some(val) => match val.parse::<i32>() {
        Ok(int_val) => int_val
      , Err(_)      => return Err(PARSE_ERROR_BAD_Y_VAL)
      }
    , None => return Err(PARSE_ERROR_MISSING_Y_VAL)
    };

    let h = match line_iter.next() {
      Some(val) => match val.parse::<Heading>() {
        Ok(hdg) => hdg
      , Err(err_msg) => return Err(err_msg)
      }
    , None => return Err(PARSE_ERROR_MISSING_HDNG)
    };

    // At this point in time, the only test we can perform on the robot's location is whether or not it falls within the
    // maximum and minimum permissible world boundaries.
    // Robot's (X,Y) location is zero-based, world dimensions are one-based
    if x >= WORLD_MIN_WIDTH-1  && x < WORLD_MAX_WIDTH &&
       y >= WORLD_MIN_HEIGHT-1 && y < WORLD_MAX_HEIGHT {
      // The validity of the robot's location and its id are unknowable at this point in time
      // The id will be assigned once the caller has validated the robot's location
      Ok(Robot {
          id      : -1
        , x       : x
        , y       : y
        , heading : h
        , is_lost : false
        })
    }
    else {
      Err(ERROR_OUTSIDE_WORLD_BOUNDS)
    } 
  }
}

// *********************************************************************************************************************
// Private API
// *********************************************************************************************************************
fn turn<'a>(headings : &'a [Heading; 4], hdg : &Heading) -> &'a Heading {
  let idx = headings.iter().position(|h| h == hdg).unwrap();
  &headings[(idx + 1) % 4]
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
  use crate::world::Dimensions;
  use crate::heading::PARSE_ERROR_INVALID_HEADING;
  
  #[test]
  fn spin_right() {
    let mut test_bot = "1 1 n".parse::<Robot>().unwrap();

    test_bot.id = 1;

    test_bot.turn_right();  assert_eq!(test_bot.heading, Heading::East);
    test_bot.turn_right();  assert_eq!(test_bot.heading, Heading::South);
    test_bot.turn_right();  assert_eq!(test_bot.heading, Heading::West);
    test_bot.turn_right();  assert_eq!(test_bot.heading, Heading::North);
  }

  #[test]
  fn spin_left() {
    let mut test_bot = "1 1 n".parse::<Robot>().unwrap();

    test_bot.id = 1;

    test_bot.turn_left();  assert_eq!(test_bot.heading, Heading::West);
    test_bot.turn_left();  assert_eq!(test_bot.heading, Heading::South);
    test_bot.turn_left();  assert_eq!(test_bot.heading, Heading::East);
    test_bot.turn_left();  assert_eq!(test_bot.heading, Heading::North);
  }

  #[test]
  fn navigate() {
    let     world_dims = "5 5".parse::<Dimensions>().unwrap();
    let mut test_world = World::new(&world_dims.width, &world_dims.height);
    let mut test_bot   = "1 1 n".parse::<Robot>().unwrap();

    test_bot.id = 1;

    test_bot.forward(&mut test_world);  assert_eq!(test_bot.position(),(&1, &2));
    test_bot.turn_left();               assert_eq!(test_bot.heading, Heading::West);
    test_bot.forward(&mut test_world);
    test_bot.forward(&mut test_world);  assert!(test_bot.is_lost);
  }  

  #[test]
  fn create_invalid_robots() {
    // Arguments missing
    let robot = "".parse::<Robot>();
    assert_eq!(robot.err(), Some(PARSE_ERROR_MISSING_VALS));

    // Invalid X argument
    let robot = "a".parse::<Robot>();
    assert_eq!(robot.err(), Some(PARSE_ERROR_BAD_X_VAL));

    // Valid X argument, invalid Y argument
    let robot = "1 a".parse::<Robot>();
    assert_eq!(robot.err(), Some(PARSE_ERROR_BAD_Y_VAL));

    // Valid X argument, but missing Y argument and heading
    let robot = "1".parse::<Robot>();
    assert_eq!(robot.err(), Some(PARSE_ERROR_MISSING_Y_VAL));

    // Valid location arguments, but missing heading
    let robot = "1 1".parse::<Robot>();
    assert_eq!(robot.err(), Some(PARSE_ERROR_MISSING_HDNG));

    // Valid location arguments, but invalid heading
    let robot = "1 1 q".parse::<Robot>();
    assert_eq!(robot.err(), Some(PARSE_ERROR_INVALID_HEADING));

    // Correct, but invalid location arguments
    let robot = "0 0 e".parse::<Robot>();
    assert_eq!(robot.err(), Some(ERROR_OUTSIDE_WORLD_BOUNDS));

    // Correct, but invalid location arguments
    let robot = "51 51 e".parse::<Robot>();
    assert_eq!(robot.err(), Some(ERROR_OUTSIDE_WORLD_BOUNDS));
  }  
}