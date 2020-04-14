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
, PROMPT_NEW_ROBOT
, PROMPT_MOVE_TURN
};

const LIB_NAME     : &str  = module_path!();
const TRACE_ACTIVE : &bool = &true;

// *********************************************************************************************************************
fn main() -> std::io::Result<()> {
  const FN_NAME : &str = &"main";

  let trace_boundary = Trace::make_boundary_trace_fn(TRACE_ACTIVE, LIB_NAME, file!());
  let trace          = Trace::make_trace_fn(TRACE_ACTIVE, LIB_NAME, FN_NAME);

  trace_boundary(&Some(true));

  // Dummy initial robot
  let mut robot      : Robot = Robot::new(0,1,1,Heading::North);
  let mut robot_id   : i32 = 0;
  let mut line_count : i32 = 1;

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Read the first line from stdin expecting the world dimensions
  let mut world : World = match create_world() {
    Ok(w)    => w
  , Err(err) =>
      match err {
        EOF_ENCOUNTERED => std::process::exit(0)
      , _               => panic!(format!("Unexpected error : {}", err))
      }
  };

  trace(&format!("Created a {}x{} world", world.width, world.height));
  
  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // I can haz data?
  prompt(PROMPT_NEW_ROBOT);

  for stdin_data in BufReader::new(std::io::stdin()).lines() {
    trace(&format!("Line {}: Expecting {}"
                  , line_count
                  , if line_count % 2 == 0 { "move/turn instruction set" } else { "new robot definition" }
                  )
         );

    // Even numbered lines should contain a move/turn instruction set
    if line_count % 2 == 0 {
      // Obey move/turn instruction set then print robot status
      robot.turn_and_move(stdin_data.unwrap().trim(), &mut world);
      trace(&world.to_string());
      prompt(PROMPT_NEW_ROBOT);
    }
    // Odd numbered lines should contain a new robot definition
    else {
      // Try to create a new robot
      match create_robot(stdin_data.unwrap().trim(), &mut world, &robot_id) {
        Ok(r)    => {
          robot = r;
          robot_id += 1;
          trace(&world.to_string());
          prompt(PROMPT_MOVE_TURN);
        }
      , Err(err) =>
          match err {
            EOF_ENCOUNTERED => break
          , _               => panic!(format!("Unexpected error : {}", err))
          }
      }
    }

    line_count += 1;
  }

  trace_boundary(&Some(false));
  Ok(())
}


// *********************************************************************************************************************
// Private API
// *********************************************************************************************************************
fn prompt(prompt_msg : &str) {
  print!("{} : ", prompt_msg);
  let _ = std::io::stdout().flush();
}

