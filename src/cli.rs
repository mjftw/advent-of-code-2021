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

  let exs = exercises::exercises();
  let exercise_days = exs.keys();

  let args: Vec<String> = exercise_days
    .clone()
    .map(|day| format!("day{}", day))
    .collect();

  app = exercise_days.clone().fold(app, |app, day| {
    let arg_name = args.get((*day - 1) as usize).unwrap();
    app.arg(
      Arg::with_name(arg_name)
        .long(arg_name)
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
  let mut exercises = exercises::exercises();

  for day in exercise_days {
    if let Some(filename) = matches.value_of(args.get((*day - 1) as usize).unwrap()) {
      match fs::read_to_string(filename) {
        Ok(input) => day_inputs.insert(*day, (exercises.remove(day).unwrap(), input)),
        Err(err) => panic!("Failed to read file {}: {}", filename, err),
      };
    }
  }

  day_inputs
}
