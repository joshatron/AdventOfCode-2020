use crate::days::Day;
use std::convert::TryInto;
use std::collections::HashMap;
use rustc_hash::FxHashMap;

pub struct Day15 {}

impl Day for Day15 {
  fn day_num(&self) -> usize {
    15
  }

  fn puzzle_1(&self, input: &Vec<String>) -> String {
    let mut initial = parse_to_sequence(&input[0]);

    while initial.turn < 2020 {
      initial.play_one_round();
    }

    initial.last.to_string()

  }

  fn puzzle_2(&self, input: &Vec<String>) -> String {
    let mut initial = parse_to_sequence(&input[0]);

    while initial.turn < 30000000 {
      initial.play_one_round();
    }

    initial.last.to_string()
  }
}

fn parse_starting_sequence(line: &str) -> Vec<usize> {
  line.split(",")
    .map(|e| e.parse::<usize>().unwrap())
    .collect()
}

fn num_since_last_occurance(nums: &Vec<usize>) -> usize {
  let num_to_check = &nums[nums.len() - 1];

  for (i, num) in nums.iter().rev().enumerate() {
    if i > 0 && num == num_to_check {
      return i;
    }
  }

  0
}

struct Sequence {
  turn: u64,
  last: u64,
  done: FxHashMap<u64, u64>,
}

impl Sequence {
  fn play_one_round(&mut self) {
    match self.done.get(&self.last) {
      Some(turn) => {
        let new_last = self.turn - turn;
        self.done.insert(self.last, self.turn);
        self.last = new_last;
      },
      None => {
        self.done.insert(self.last, self.turn);
        self.last = 0;
      }
    }
    self.turn += 1;
  }
}

fn parse_to_sequence(line: &str) -> Sequence {
  let mut sequence = Sequence {
    turn: 0,
    last: 0, 
    done: FxHashMap::default(),
  };

  let mut s = parse_starting_sequence(line);
  let last = s.pop().unwrap();

  for n in s {
    sequence.turn += 1;
    sequence.done.insert(n.try_into().unwrap(), sequence.turn);
  }

  sequence.turn += 1;
  sequence.last = last.try_into().unwrap();

  sequence
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_starting_sequence() {
    assert_eq!(parse_starting_sequence("0,3,6"), vec![0, 3, 6]);
    assert_eq!(parse_starting_sequence("0,37,645"), vec![0, 37, 645]);
  }

  #[test]
  fn test_num_since_last_occurance() {
    assert_eq!(num_since_last_occurance(&vec![0, 3, 6]), 0);
    assert_eq!(num_since_last_occurance(&vec![0, 3, 6, 0]), 3);
    assert_eq!(num_since_last_occurance(&vec![0, 3, 6, 0, 3]), 3);
    assert_eq!(num_since_last_occurance(&vec![0, 3, 6, 0, 3, 3]), 1);
    assert_eq!(num_since_last_occurance(&vec![0, 3, 6, 0, 3, 3, 1]), 0);
    assert_eq!(num_since_last_occurance(&vec![0, 3, 6, 0, 3, 3, 1, 0]), 4);
    assert_eq!(num_since_last_occurance(&vec![0, 3, 6, 0, 3, 3, 1, 0, 4]), 0);
  }

  #[test]
  fn test_puzzle_1() {
    assert_eq!(Day15{}.puzzle_1(&vec![String::from("0,3,6")]), "436");
  }

  #[test]
  fn test_parse_to_sequence() {
    let sequence = parse_to_sequence("0,3,6");
    assert_eq!(sequence.turn, 3);
    assert_eq!(sequence.last, 6);
    assert_eq!(sequence.done.len(), 2);
    assert_eq!(sequence.done.get(&0).unwrap(), &1);
    assert_eq!(sequence.done.get(&3).unwrap(), &2);
  }

  #[test]
  fn test_play_one_round() {
    let mut sequence = parse_to_sequence("0,3,6");
    sequence.play_one_round();
    assert_eq!(sequence.turn, 4);
    assert_eq!(sequence.last, 0);
    assert_eq!(sequence.done.len(), 3);
    assert_eq!(sequence.done.get(&6).unwrap(), &3);
    sequence.play_one_round();
    assert_eq!(sequence.turn, 5);
    assert_eq!(sequence.last, 3);
    sequence.play_one_round();
    assert_eq!(sequence.turn, 6);
    assert_eq!(sequence.last, 3);
    sequence.play_one_round();
    assert_eq!(sequence.turn, 7);
    assert_eq!(sequence.last, 1);
  }

  #[test]
  fn test_puzzle_2() {
    assert_eq!(Day15{}.puzzle_2(&vec![String::from("0,3,6")]), "175594");
  }
}
