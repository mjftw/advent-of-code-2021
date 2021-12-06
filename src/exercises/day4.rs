use super::Exercise;

///[Advent of Code - Day 4](https://adventofcode.com/2021/day/4)
pub struct Day4;

type Tile = (u8, bool);

/// Board tiles have the following indices:
///  0  1  2  3  4
///  5  6  7  8  9
/// 10 11 12 13 14
/// 15 16 17 18 19
/// 20 21 22 23 24
/// The type at each tile is (number, is_marked?)
#[derive(Debug)]
struct Board([Tile; 25]);

impl Board {
  fn new(input: &str) -> Board {
    let nums: Vec<u8> = input
      .split_whitespace()
      .map(|num_char| num_char.parse().expect("Failed to parse board"))
      .collect();

    if nums.len() != 25 {
      panic!("Board input not 25 numbers!");
    }

    let mut tiles = [(0, false); 25];
    for (i, n) in nums.iter().enumerate() {
      tiles[i] = (*n, false);
    }

    Board(tiles)
  }

  fn mark_num(&mut self, num: u8) {
    for i in 0..25 {
      if self.0[i].0 == num {
        self.0[i] = (self.0[i].0, true);
        break;
      }
    }
  }

  fn is_winning(&self) -> bool {
    for i in 0..5 {
      // Check if row i all marked
      if (i..i + 5).all(|idx| self.0[idx].1) {
        return true;
      }

      // Check if col i all marked
      if (i..25).step_by(5).all(|idx| self.0[idx].1) {
        return true;
      }
    }

    false
  }

  fn score(&self, multiplier: u32) -> u32 {
    multiplier
      * self
        .0
        .iter()
        .filter(|(_, is_marked)| !is_marked)
        .fold(0, |sum, (num, _)| sum + *num as u32)
  }
}

/// Returns (vec of marked numbers, vec of boards)
fn parse_input(raw_input: &str) -> (Vec<u8>, Vec<Board>) {
  let mut sections = raw_input.split("\n\n");

  let marked_nums = sections
    .next()
    .expect("Input file is empty")
    .split(",")
    .map(|num_char| num_char.parse().expect("Failed to parse marked numbers"))
    .collect();

  let boards = sections
    .map(|board_chars| Board::new(board_chars))
    .collect();

  (marked_nums, boards)
}

impl Exercise for Day4 {
  fn solve(&self, raw_input: &str) -> String {
    let (marked_nums, mut boards) = parse_input(raw_input);

    for to_mark in marked_nums {
      for board in &mut boards {
        board.mark_num(to_mark);
        if board.is_winning() {
          return format!("{}", board.score(to_mark as u32));
        }
      }
    }

    "No winner!".to_string()
  }
}
