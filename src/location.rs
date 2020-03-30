// *********************************************************************************************************************
// Location definition
// *********************************************************************************************************************
#[derive(Debug)]
pub struct Location {
  pub id : i32
, pub x  : i32
, pub y  : i32

, pub can_go_north : bool
, pub can_go_south : bool
, pub can_go_east  : bool
, pub can_go_west  : bool
}

// *********************************************************************************************************************
// Location implementation
// *********************************************************************************************************************
impl Location {
  pub fn move_to(&mut self, x : i32, y : i32) {
    self.x = x;
    self.y = y;
  }

  pub fn new(x : i32, y : i32) -> Location {
    Location {
      id : -1               // Location currently unoccupied
    , x
    , y
    , can_go_north : true   // Currently, it's safe to go in this directions
    , can_go_south : true   // Currently, it's safe to go in this directions
    , can_go_east  : true   // Currently, it's safe to go in this directions
    , can_go_west  : true   // Currently, it's safe to go in this directions
    }
  }
}