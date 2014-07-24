#![crate_name = "ms"]
#![license = "MIT"]
#![feature(globs)]

extern crate regex;

use regex::Regex;

static s : int = 1000;
static m : int = s * 60;
static h : int = m * 60;
static d : int = h * 24;
static y : int = d * 365;

pub fn to_ms(value : &str) -> int {
  println!("{}",value);
  let timeReg = match Regex::new(r"^((?:\d+)?\.?\d+) *(ms|seconds?|s|minutes?|m|hours?|h|days?|d|years?|y)?$") {
    Ok(timeReg) => timeReg,
    Err(err) => fail!("{}", err),
  };

  if !timeReg.is_match(value) {
    //Err("input doesn't match the required format");
  }
  let mut first_piece : int = 0;
  let mut second_piece : &str = "";
  for cap in timeReg.captures_iter(value) {
    first_piece = match from_str(cap.at(1)) {
      Some(piece) => piece,
      None => 0
    };
    second_piece = cap.at(2);
  }

  match second_piece {
    "years" | "y" => first_piece * y,
    "days" | "d" => first_piece * d,
    "hours" | "h" => first_piece * h,
    "minutes" | "m" => first_piece * m,
    "seconds" | "s" => first_piece * s,
    "ms" => first_piece,
    _ => -1
  }
  
}

#[test]
fn test_valid_input() {
  assert_eq!(to_ms("5s"), 5000);
  assert_eq!(to_ms("6m"), 360000);
  assert_eq!(to_ms("7h"), 25200000);
  assert_eq!(to_ms("8d"), 691200000);
  assert_eq!(to_ms("9y"), 283824000000);
}

#[test]
fn test_invalid_input() {
  assert_eq!(to_ms("gibberish"), -1);
}