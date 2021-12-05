use super::Exercise;

///[Advent of Code - Day 3](https://adventofcode.com/2021/day/3)
pub struct Day3;

impl Exercise for Day3 {
  fn solve(&self, raw_input: &str) -> String {
    let mut bit_counts = Vec::new();
    let mut num_lines = 0;
    let lines = raw_input.trim().split('\n');

    for line in lines {
      num_lines += 1;

      for (idx, c) in line.chars().enumerate() {
        if c == '1' {
          if bit_counts.len() < idx + 1 {
            bit_counts.push(1);
          } else {
            bit_counts[idx] += 1;
          }
        }
      }
    }
    let word_len = bit_counts.len();

    let mut gamma_rate: u32 = 0;

    bit_counts.reverse();
    for (idx, count) in bit_counts.iter().enumerate() {
      if *count > num_lines / 2 {
        gamma_rate |= 1 << idx;
      }
    }

    let epsilon_rate = !gamma_rate & (0xffffffff >> 32 - word_len);

    let power_consumption = gamma_rate * epsilon_rate;

    format!("Power consumption: {}", power_consumption)
  }
}
