use crate::days::Day;

pub struct Day1 {}

impl Day1 {
  pub fn new() -> Day1 {
    Day1{}
  }
}

impl Day for Day1 {
  fn day_num(&self) -> usize {
    1
  }

  fn puzzle_1(&self, input: &Vec<String>) -> String {
    String::from("")
  }

  fn puzzle_2(&self, input: &Vec<String>) -> String {
    String::from("")
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn sample_input() -> Vec<String> {
    vec![
      String::from("")
    ]
  }

  #[test]
  fn test_puzzle_1() {
    assert_eq!(Day1::new().puzzle_1(&sample_input()), "");
  }

  #[test]
  fn test_puzzle_2() {
    assert_eq!(Day1::new().puzzle_2(&sample_input()), "");
  }
}
