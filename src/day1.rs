//! [Advent of Code - Day 1](https://adventofcode.com/2021/day/1)

pub fn parse_input(raw_input: &str) -> Vec<u32> {
  let mut parsed = Vec::new();

  for line in raw_input.trim().split('\n') {
    parsed.push(
      line
        .parse()
        .expect(&format!("Failed to parse input line: {}", line)),
    );
  }

  parsed
}

pub fn solve(input: &[u32]) -> u32 {
  // Would have been easier to use a loop, but solved FP style for fun
  let (_, increases): (u32, u32) =
    input[1..]
      .iter()
      .fold((input[0], 0), |(last_depth, increases), depth| {
        if *depth > last_depth {
          (*depth, increases + 1)
        } else {
          (*depth, increases)
        }
      });

  increases
}
