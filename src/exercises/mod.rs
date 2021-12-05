use std::collections::HashMap;
use std::hash::Hash;

mod day1;
mod day2;
mod day3;

pub trait Exercise {
  fn solve(&self, raw_input: &str) -> String;
}

fn to_hashmap<K: Eq + Hash, V>(tuples: Vec<(K, V)>) -> HashMap<K, V> {
  tuples.into_iter().collect()
}

/// Get a map of all available exercises, with the key being the day number
/// and value being the exercise.
pub fn exercises() -> HashMap<u32, Box<dyn Exercise>> {
  // Extend this vector to add new exercises
  to_hashmap(vec![
    (1, Box::new(day1::Day1)),
    (2, Box::new(day2::Day2)),
    (3, Box::new(day3::Day3)),
  ])
}
