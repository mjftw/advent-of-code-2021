use std::array::IntoIter;
use std::collections::HashMap;
use std::hash::Hash;

mod day1;

pub trait Exercise {
  fn solve(&self, raw_input: &str) -> String;
}

fn to_hashmap<K: Eq + Hash, V>(tuples: Vec<(K, V)>) -> HashMap<K, V> {
  tuples.into_iter().collect()
}

pub fn exercises() -> HashMap<u32, Box<dyn Exercise>> {
  to_hashmap(vec![(1, Box::new(day1::Day1))])
}
