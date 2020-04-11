use std::io::prelude::*;
use std::io::BufReader;
use std::str;

pub mod trace;
pub mod robot;
pub mod location;
pub mod world;
pub mod heading;

use crate::trace::Trace;
use crate::heading::Heading;
use crate::world::{
  create_world
, World
, EOF_ENCOUNTERED
};

use crate::robot::{
  create_robot
, Robot
};

const LIB_NAME     : &str  = &"main";
const TRACE_ACTIVE : &bool = &false;

// *********************************************************************************************************************
fn main() -> std::io::Result<()> {
  const FN_NAME : &str = &"main";

  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);
  let trace          = Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);

  trace_boundary(&Some(true));

  // Dummy initial robot
  let mut robot    : Robot = Robot::new(0,1,1,Heading::North);
  let mut robot_id : i32 = 0;

  let mut stdin                = BufReader::new(std::io::stdin());
  let mut stdin_data : Vec<u8> = Vec::new();
  let mut line_count : i32     = 1;

  // Read the first line from stdin expecting the world dimensions
  let mut world : World;

  match create_world() {
    Ok(w)    => world = w
  , Err(err) =>
      match err {
        EOF_ENCOUNTERED => std::process::exit(0)
      , _               => panic!(format!("Unexpected error : {}", err))
      }
  }

  trace(&format!("Created a {}x{} world", world.width, world.height));
  
  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // I can haz data?
  loop {
    trace(&format!("Line {}: expecting {}"
                  , line_count
                  , if line_count % 2 == 0 { "move/turn instruction set" } else { "new robot definition" }
                  )
         );
    // Bail out as soon as EOF is encountered
    if stdin.read_until(b'\n', &mut stdin_data)? == 0 { break };
    let line = str::from_utf8(&stdin_data).unwrap().trim();
    
    // Even numbered lines should contain a move/turn instruction set
    if line_count % 2 == 0 {
      // Obey move/turn instruction set then print robot status
      robot.turn_and_move(line, &mut world);
      println!("{}", &robot);
    }
    // Odd numbered lines should contain a new robot definition
    else {
      // Create a new robot
      match create_robot(line, &mut world, &robot_id) {
        Ok(r)    => {
          robot = r;
          robot_id += 1;
        }
      , Err(err) =>
          match err {
            EOF_ENCOUNTERED => break
          , _               => panic!(format!("Unexpected error : {}", err))
          }
      }
    }

    line_count += 1;
    stdin_data.clear();
  }

  trace_boundary(&Some(false));
  Ok(())
}


