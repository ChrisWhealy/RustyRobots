// *********************************************************************************************************************
// Trace Utility
// 
// (c) Chris Whealy 2019
// *********************************************************************************************************************

const ENTRY_ARROW  : &str = &"--->";
const EXIT_ARROW   : &str = &"<---";
const IN_OUT_ARROW : &str = &"<-->";


pub struct Trace {}

impl Trace {
  // *******************************************************************************************************************
  // Trace execution flow at function boundaries
  // *******************************************************************************************************************
  pub fn make_boundary_trace_fn<'a>(
    is_active : &'a bool
  , lib_name  : &'a str
  , fn_name   : &'a str
  ) -> impl Fn(&'a Option<bool>)
  {
    move |is_entry| {
      if *is_active {
        let ptr = match is_entry {
          Some(b) => if *b { ENTRY_ARROW } else { EXIT_ARROW }
        , None    => IN_OUT_ARROW
        };

        &println!("{} {}.{}()", ptr, lib_name, fn_name);
      }
      else {
        ()
      }
    }
  }

  // *******************************************************************************************************************
  // Trace data during execution flow
  // *******************************************************************************************************************
  pub fn make_trace_fn<'a>(
    is_active : &'a bool
  , lib_name  : &'a str
  , fn_name   : &'a str
  ) -> impl Fn(&str) + 'a 
  {
    move |info| {
      if *is_active {
        &println!("     {}.{}() {}", lib_name, fn_name, info);
      }
      else {
        ()
      }
    }
  }
}

