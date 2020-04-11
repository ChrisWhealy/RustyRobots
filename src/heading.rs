use std::{fmt, str};

pub const PARSE_ERROR_INVALID_HEADING : &str = &"Invalid heading";

// *********************************************************************************************************************
// Heading definition
// *********************************************************************************************************************
#[derive(Debug, PartialEq, Clone)]
pub enum Heading {
  North
, South
, East
, West
}

// *********************************************************************************************************************
// Heading implementation
// *********************************************************************************************************************
impl fmt::Display for Heading {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let txt = match self {
      Heading::North => "N"
    , Heading::South => "S"
    , Heading::East  => "E"
    , Heading::West  => "W"
    };
    write!(f, "{}", txt)
  }
}

impl str::FromStr for Heading {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let hdg = match s {
      "N" => Heading::North
    , "n" => Heading::North
    , "S" => Heading::South
    , "s" => Heading::South
    , "E" => Heading::East
    , "e" => Heading::East
    , "W" => Heading::West
    , "w" => Heading::West
    , _   => return Err(PARSE_ERROR_INVALID_HEADING)
    };

    Ok(hdg)
  }
}

// *********************************************************************************************************************
// Suppose we'd better test it...
// *********************************************************************************************************************
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_test_valid() {
    let p01 = "n".parse::<Heading>();
    let p02 = "N".parse::<Heading>();
    let p03 = "e".parse::<Heading>();
    let p04 = "E".parse::<Heading>();
    let p05 = "s".parse::<Heading>();
    let p06 = "S".parse::<Heading>();
    let p07 = "w".parse::<Heading>();
    let p08 = "W".parse::<Heading>();

    assert_eq!(p01.unwrap(), Heading::North);
    assert_eq!(p02.unwrap(), Heading::North);
    assert_eq!(p03.unwrap(), Heading::East);
    assert_eq!(p04.unwrap(), Heading::East);
    assert_eq!(p05.unwrap(), Heading::South);
    assert_eq!(p06.unwrap(), Heading::South);
    assert_eq!(p07.unwrap(), Heading::West);
    assert_eq!(p08.unwrap(), Heading::West);
  }  

  #[test]
  fn parse_test_invalid() {
    let p01 = "a".parse::<Heading>();
    let p02 = "4".parse::<Heading>();
    let p03 = "=".parse::<Heading>();
    let p04 = "z".parse::<Heading>();
    let p05 = "!".parse::<Heading>();
    let p06 = "$".parse::<Heading>();
    let p07 = "\t".parse::<Heading>();
    let p08 = " ".parse::<Heading>();

    assert_eq!(p01.err(), Some(PARSE_ERROR_INVALID_HEADING));
    assert_eq!(p02.err(), Some(PARSE_ERROR_INVALID_HEADING));
    assert_eq!(p03.err(), Some(PARSE_ERROR_INVALID_HEADING));
    assert_eq!(p04.err(), Some(PARSE_ERROR_INVALID_HEADING));
    assert_eq!(p05.err(), Some(PARSE_ERROR_INVALID_HEADING));
    assert_eq!(p06.err(), Some(PARSE_ERROR_INVALID_HEADING));
    assert_eq!(p07.err(), Some(PARSE_ERROR_INVALID_HEADING));
    assert_eq!(p08.err(), Some(PARSE_ERROR_INVALID_HEADING));
  }  
}