use crate::days::Day;

pub struct Day1 {}

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

  #[test]
  fn test() {
    assert_eq!(0, 0);
  }

}
