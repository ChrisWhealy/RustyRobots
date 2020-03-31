use crate::world::{World, Heading};
use crate::trace::Trace;

const LIB_NAME     : &str = &"robot";
const TRACE_ACTIVE : &bool = &false;

const HEADINGS_LEFT :[Heading; 4] = [Heading::North, Heading::West, Heading::South, Heading::East];
const HEADINGS_RIGHT:[Heading; 4] = [Heading::North, Heading::East, Heading::South, Heading::West];

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

    if world.is_it_safe(&self.x, &self.y, &self.heading) {
      trace(&format!("It appears safe to head {} from ({},{})", &self.heading, &self.x, &self.y));

      let (new_x, new_y) = match self.heading {
        Heading::North => (self.x,     self.y + 1),
        Heading::East  => (self.x + 1, self.y),
        Heading::South => (self.x,     self.y - 1),
        Heading::West  => (self.x - 1, self.y),
      };

      // Moving forward always moves the robot out of its current location,
      // irrespective of whether or not this move will kill it
      world.remove_robot_from(&self.x, &self.y);

      // Did we just die?
      if (new_y < 0 || new_y > world.height) ||
         (new_x < 0 || new_x > world.width) {
        trace(&"Bad idea - I died!");
       // Warn other robots not to venture this way
        world.here_be_monsters(&self.x, &self.y, &self.heading);

        // The robot is now lost, but its last known location needs to be printed, so don't update x and y
        self.is_lost = true;
      }
      else {
        // Update the robot's position and update the world grid
        self.x = new_x;
        self.y = new_y;
        world.place_robot_at(&self.id, &self.x, &self.y);
        trace(&format!("Robot {} is now at ({},{}) heading {}", &self.id, &self.x, &self.y, &self.heading));
      }
    }
    else {
      trace(&format!("Ignoring instruction to head {} from ({},{}) - here be monsters!", &self.heading, &self.x, &self.y));
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

fn turn<'a>(headings : &'a [Heading; 4], hdg : &Heading) -> &'a Heading {
  let idx = headings.iter().position(|h| h == hdg).unwrap();
  &headings[(idx + 1) % 4]
}


// *********************************************************************************************************************
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn spin_right() {
    let mut test_bot = Robot::new(1,1,1, Heading::North);

    test_bot.turn_right();  assert_eq!(test_bot.heading, Heading::East);
    test_bot.turn_right();  assert_eq!(test_bot.heading, Heading::South);
    test_bot.turn_right();  assert_eq!(test_bot.heading, Heading::West);
    test_bot.turn_right();  assert_eq!(test_bot.heading, Heading::North);
  }

  #[test]
  fn spin_left() {
    let mut test_bot = Robot::new(1,1,1, Heading::North);

    test_bot.turn_left();  assert_eq!(test_bot.heading, Heading::West);
    test_bot.turn_left();  assert_eq!(test_bot.heading, Heading::South);
    test_bot.turn_left();  assert_eq!(test_bot.heading, Heading::East);
    test_bot.turn_left();  assert_eq!(test_bot.heading, Heading::North);
  }

  #[test]
  fn navigate() {
    let mut test_bot   = Robot::new(1,1,1, Heading::North);
    let mut test_world = World::new(&5, &5);

    test_bot.forward(&mut test_world);
    assert_eq!(test_bot.position(),(&1, &2));

    test_bot.turn_left();
    assert_eq!(test_bot.heading, Heading::West);

    test_bot.forward(&mut test_world);
    test_bot.forward(&mut test_world);
    assert!(test_bot.is_lost);
  }  
}