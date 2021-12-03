use std::collections::HashMap;
use std::fs;
use std::process;

use clap::{App, Arg};

use crate::exercises::{self, Exercise};

/// Get desired exercises and input files via a CLI
/// The user gives a filename for the input for each exercise to be run
/// A map of day number to tuple(exercise, input data) is returned, where the input data is
/// read from the file specified in the command line arguments
pub fn cli_get_exercises() -> HashMap<u32, (Box<dyn Exercise>, String)> {
  let mut app = App::new("Advent of Code 2021")
    .version("1.0")
    .author("Merlin Webster <mjftwebster@gmail.com>")
    .about("Solves exercises from the Advent of Code challenge 2021");

  // Bug: day_args not sorted
  let day_args: HashMap<u32, String> =
    exercises::exercises()
      .iter()
      .fold(HashMap::new(), |mut hm, (day, _)| {
        hm.insert(*day, format!("day{}", *day));
        hm
      });

  app = day_args.values().fold(app, |app, arg| {
    app.arg(
      Arg::with_name(arg)
        .long(arg)
        .value_name("INPUT_FILENAME")
        .takes_value(true),
    )
  });

  let matches = app.clone().get_matches();

  if matches.args.is_empty() {
    let _ = app.print_help();
    process::exit(1);
  }

  let mut day_inputs = HashMap::new();

  for (day, exercise) in exercises::exercises() {
    //Bug: day_args.get() assumes day_args are sorted
    if let Some(filename) = matches.value_of(day_args.get(&day).unwrap()) {
      match fs::read_to_string(filename) {
        Ok(input) => day_inputs.insert(day, (exercise, input)),
        Err(err) => panic!("Failed to read file {}: {}", filename, err),
      };
    }
  }

  day_inputs
}
