use super::Exercise;

///[Advent of Code - Day 1](https://adventofcode.com/2021/day/1)
pub struct Day1;

fn parse_input(raw_input: &str) -> Vec<u32> {
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
fn solve_part1(input: &Vec<u32>) -> u32 {
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

fn solve_part2(input: &Vec<u32>) -> u32 {
  let mut increases = 0;
  for i in 3..input.len() {
    let last_window_sum: u32 = input[(i - 3)..=(i - 1)].iter().sum();
    let window_sum: u32 = input[i - 2..=i].iter().sum();

    if window_sum > last_window_sum {
      increases += 1;
    }
  }

  increases
}

impl Exercise for Day1 {
  fn solve(&self, raw_input: &str) -> String {
    let input = parse_input(raw_input);

    format!(
      "Part1 = {}, Part2 = {}",
      solve_part1(&input),
      solve_part2(&input)
    )
  }
}
