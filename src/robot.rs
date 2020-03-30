use crate::world::World;
use crate::trace::Trace;

const LIB_NAME     : &str = &"robot";
const TRACE_ACTIVE : &bool = &false;

const HEADINGS_LEFT :[char; 4] = ['N', 'W', 'S', 'E'];
const HEADINGS_RIGHT:[char; 4] = ['N', 'E', 'S', 'W'];

// *********************************************************************************************************************
// Robot definition
// *********************************************************************************************************************
#[derive(Debug)]
pub struct Robot {
  pub id      : i32
, pub x       : i32
, pub y       : i32
, pub heading : char
, pub is_lost : bool
}

// *********************************************************************************************************************
// Robot implementation
// *********************************************************************************************************************
impl Robot {
  pub fn turn_right(&mut self) {
    self.heading = *turn(&HEADINGS_RIGHT, &self.heading);
    Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME, &"turn_right")(&format!("New heading = {}", &self.heading));
  }

  pub fn turn_left(&mut self) {
    self.heading = *turn(&HEADINGS_LEFT, &self.heading);
    Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME, &"turn_left")(&format!("New heading = {}", &self.heading));
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
        'N' => (self.x,     self.y + 1),
        'E' => (self.x + 1, self.y),
        'S' => (self.x,     self.y - 1),
        'W' => (self.x - 1, self.y),
        _   => (self.x,     self.y)
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
  pub fn new(id : i32, x : i32, y : i32, heading : char) -> Robot {
    Robot{
      id
    , x
    , y
    , heading
    , is_lost : false
    }
  }
}

fn turn<'a>(headings : &'a [char; 4], hdg : &char) -> &'a char {
  let idx = headings.iter().position(|&h| &h == hdg).unwrap();
  &headings[(idx + 1) % 4]
}
