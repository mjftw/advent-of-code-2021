use std::collections::HashMap;
use std::fs;

use clap::{App, Arg};

pub fn cli_get_day_inputs() -> HashMap<u32, String> {
  let mut app = App::new("Advent of Code 2021")
    .version("1.0")
    .author("Merlin Webster <mjftwebster@gmail.com>")
    .about("Solves exercises from the Advent of Code challenge 2021");

  let args: Vec<String> = (1..=25).map(|day| format!("day{}", day)).collect();

  app = (0..25).fold(app, |app, day| {
    app.arg(
      Arg::with_name(args.get(day).unwrap())
        .long(args.get(day).unwrap())
        .value_name("INPUT_FILENAME")
        .takes_value(true),
    )
  });

  let matches = app.get_matches();

  let mut day_inputs = HashMap::new();

  for day in 1..=25 {
    if let Some(filename) = matches.value_of(&format!("day{}", day)) {
      match fs::read_to_string(filename) {
        Ok(input) => day_inputs.insert(day as u32, input),
        Err(err) => panic!("Failed to read file {}: {}", filename, err),
      };
    }
  }

  day_inputs
}
