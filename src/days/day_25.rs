use crate::days;
use crate::days::Day;

pub struct Day25 {}

impl Day25 {
  pub fn new() -> Day25 {
    Day25 {}
  }
}

impl Day for Day25 {
  fn day_num(&self) -> usize {
    25
  }

  fn puzzle_1(&self, input: &Vec<String>) -> String {
    let keys = days::input_to_ints(input);
    let loops_for_first = find_loop_size(keys[0] as usize, 1, 7, 20201227);

    let mut current = 1;
    for _ in 0..loops_for_first {
      current = do_loop(current, keys[1] as usize, 20201227);
    }

    current.to_string()
  }

  fn puzzle_2(&self, _input: &Vec<String>) -> String {
    String::from("Merry Christmas!")
  }
}

fn find_loop_size(public_key: usize, start: usize, subject_number: usize, max: usize) -> usize {
  let mut current = start;
  let mut loops = 0;
  while current != public_key {
    current = do_loop(current, subject_number, max);
    loops += 1
  }

  loops
}

fn do_loop(current: usize, subject_number: usize, max: usize) -> usize {
  (current * subject_number) % max
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_find_loop_size() {
    assert_eq!(find_loop_size(5764801, 1, 7, 20201227), 8);
    assert_eq!(find_loop_size(17807724, 1, 7, 20201227), 11);
  }

  #[test]
  fn test_do_loop() {
    assert_eq!(do_loop(1, 7, 20201227), 7);
  }

  fn sample_input() -> Vec<String> {
    vec![String::from("5764801"), String::from("17807724")]
  }

  #[test]
  fn test_puzzle_1() {
    assert_eq!(Day25::new().puzzle_1(&sample_input()), "14897079");
  }

  #[test]
  fn test_puzzle_2() {
    assert_eq!(Day25::new().puzzle_2(&sample_input()), "Merry Christmas!");
  }
}
