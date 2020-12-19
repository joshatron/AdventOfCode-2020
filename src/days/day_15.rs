use crate::days::Day;

pub struct Day15 {}

impl Day15 {
  pub fn new() -> Day15 {
    Day15{}
  }
}

impl Day for Day15 {
  fn day_num(&self) -> usize {
    15
  }

  fn puzzle_1(&self, input: &Vec<String>) -> String {
    let mut initial = Sequence::parse(&input[0]);

    while initial.turn < 2020 {
      initial.play_one_round();
    }

    initial.last.to_string()

  }

  fn puzzle_2(&self, input: &Vec<String>) -> String {
    let mut initial = Sequence::parse(&input[0]);

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

struct Sequence {
  turn: usize,
  last: usize,
  done: Vec<usize>,
}

impl Sequence {
  fn play_one_round(&mut self) {
    if self.done[self.last] != 0 {
        let new_last = self.turn - self.done[self.last];
        self.done[self.last] = self.turn;
        self.last = new_last;
    } else {
        self.done[self.last] = self.turn;
        self.last = 0;
    }

    self.turn += 1;
  }

  fn parse(line: &str) -> Sequence {
    let mut sequence = Sequence {
      turn: 0,
      last: 0, 
      done: vec![0; 30000000],
    };

    let mut s = parse_starting_sequence(line);
    let last = s.pop().unwrap();

    for n in s {
      sequence.turn += 1;
      sequence.done[n] = sequence.turn;
    }

    sequence.turn += 1;
    sequence.last = last;

    sequence
  }
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
  fn test_parse_to_sequence() {
    let sequence = Sequence::parse("0,3,6");
    assert_eq!(sequence.turn, 3);
    assert_eq!(sequence.last, 6);
  }

  #[test]
  fn test_play_one_round() {
    let mut sequence = Sequence::parse("0,3,6");
    sequence.play_one_round();
    assert_eq!(sequence.turn, 4);
    assert_eq!(sequence.last, 0);
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
  fn test_puzzle_1() {
    assert_eq!(Day15{}.puzzle_1(&vec![String::from("0,3,6")]), "436");
  }
}
