//! [Advent of Code - Day 1](https://adventofcode.com/2021/day/1)

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
