extern crate regex;

use regex::Regex;

static s : int = 1000;
static m : int = s * 60;
static h : int = m * 60;
static d : int = h * 24;
static y : int = d * 365;

fn main() {

}

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
    _ => 0
  }
  
}