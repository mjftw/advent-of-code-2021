use super::Exercise;

///[Advent of Code - Day 2](https://adventofcode.com/2021/day/2)
pub struct Day2;

enum Command {
  Up(u32),
  Down(u32),
  Forward(u32),
}

struct Location {
  horizontal: u32,
  depth: u32,
}

impl Location {
  pub fn new() -> Self {
    Location {
      horizontal: 0,
      depth: 0,
    }
  }

  pub fn translate(self, command: &Command) -> Self {
    match command {
      Command::Up(distance) => Location {
        depth: self.depth - distance,
        ..self
      },
      Command::Down(distance) => Location {
        depth: self.depth + distance,
        ..self
      },
      Command::Forward(distance) => Location {
        horizontal: self.horizontal + distance,
        ..self
      },
    }
  }

  pub fn summary(&self) -> u32 {
    self.horizontal * self.depth
  }
}

fn count_digits(num: usize) -> usize {
  num.to_string().len()
}

#[test]
fn test_count_digits() {
  assert_eq!(count_digits(1), 1);
  assert_eq!(count_digits(12), 2);
  assert_eq!(count_digits(123), 3);
  assert_eq!(count_digits(1234), 4);
}

fn line_parse_error(line: &str, line_num: usize, err_pos: usize, reason: &str) -> String {
  format!(
    "\n\
     {0}: {1}\n\
     {2}^\n\
     {2}Error: {3}\n",
    line_num,
    line,
    " ".repeat(err_pos + count_digits(line_num) + 2),
    reason
  )
}

fn parse_input(raw_input: &str) -> Vec<Command> {
  let mut commands = Vec::new();

  for (idx, line) in raw_input.trim().split('\n').enumerate() {
    let raw_command: Vec<&str> = line.trim().split(' ').collect();
    let line_num = idx + 1;

    let parse_int = |direction: &str, distance: &str| {
      distance.parse().expect(&line_parse_error(
        line,
        line_num,
        direction.len() + 1,
        "Expected an integer here!",
      ))
    };

    match raw_command[..] {
      [direction @ "up", distance] => commands.push(Command::Up(parse_int(direction, distance))),
      [direction @ "down", distance] => {
        commands.push(Command::Down(parse_int(direction, distance)))
      }
      [direction @ "forward", distance] => {
        commands.push(Command::Forward(parse_int(direction, distance)))
      }
      _ => panic!(
        "{}",
        &line_parse_error(
          line,
          line_num,
          0,
          r#"Expected line to have format "direction distance""#
        )
      ),
    }
  }

  commands
}

impl Exercise for Day2 {
  fn solve(&self, raw_input: &str) -> String {
    let commands = parse_input(raw_input);

    let location = commands.iter().fold(Location::new(), |location, command| {
      location.translate(command)
    });

    format!("{}", location.summary())
  }
}
