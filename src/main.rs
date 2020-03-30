use std::io::prelude::*;
use std::io::BufReader;
use std::str;

pub mod robot;
pub mod location;
pub mod world;
pub mod trace;

use crate::robot::Robot;
use crate::world::World;
use crate::trace::Trace;

const LIB_NAME     : &str  = &"main";
const TRACE_ACTIVE : &bool = &false;

// *********************************************************************************************************************
fn main() -> std::io::Result<()> {
  const FN_NAME : &str = &"main";

  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);
  let trace          = Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);

  trace_boundary(&Some(true));

  let mut stdin = BufReader::new(std::io::stdin());
  let mut raw_input : Vec<u8> = Vec::new();
  
  // Dummy initial robot
  let mut robot    : Robot = Robot::new(0,0,0,'N');
  let mut robot_id : i32 = 0;
  
  // Read first line and create the world
  stdin.read_until(b'\n', &mut raw_input)?;
  
  let mut line      = str::from_utf8(&raw_input).unwrap().trim();
  let mut line_iter = line.split_ascii_whitespace();
  let     width     = line_iter.next().unwrap().parse::<i32>().unwrap();
  let     height    = line_iter.next().unwrap().parse::<i32>().unwrap();
  let mut world     = World::new(&width, &height);

  let mut line_count : i32 = 1;

  trace(&format!("World dimensions = {}x{}", width, height));
  raw_input.clear();

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // I can haz data?
  loop {
    // Bail out as soon as EOF is encountered
    if stdin.read_until(b'\n', &mut raw_input)? == 0 { break };

    line      = str::from_utf8(&raw_input).unwrap().trim();
    line_iter = line.split_ascii_whitespace();

    // Even numbered lines contain move instruction set
    if line_count % 2 == 0 {
      for c in line_iter.next().unwrap().to_ascii_uppercase().chars() {
        // If I died as a result of following a previous instruction, then bail out
        if robot.is_lost {
          break;
        }
        else {
          // Pass commands to robot
          match c {
            'R' => robot.turn_right(),
            'L' => robot.turn_left(),
            'F' => robot.forward(&mut world),
            _   => break
          }
        }
      }

      // Print robot status after instruction set has been applied
      println!("{} {} {}{}", &robot.x, &robot.y, &robot.heading, if robot.is_lost { " LOST" } else { "" })
    }
    // Odd numbered lines contain new robot definition
    else {
      let x = line_iter.next().unwrap().parse::<i32>().unwrap();
      let y = line_iter.next().unwrap().parse::<i32>().unwrap();
      let heading = line_iter.next().unwrap().chars().next().unwrap().to_ascii_uppercase();

      // Check that new location is within the world's boundaries
      if x <= width as i32  && y <= height as i32 {
        // Check if the world location already contains a robot
        if world.is_location_occupied(x, y) {
          println!("ERROR: Cannot create robot at location ({},{}) - already occupied", x, y);
        }
        else {
          robot = Robot::new(robot_id, x, y, heading);
          world.place_robot_at(&robot_id, &x, &y);
          trace(&format!("New robot created at ({},{}) heading {}", x, y, heading));
          robot_id += 1;
        }
      }
      else {
        println!("ERROR: Cannot create robot at location ({},{}) - outside world boundaries", x, y);
      }
    }


    raw_input.clear();
    line_count += 1;
  }

  trace_boundary(&Some(false));
  Ok(())
}

