use crate::days::Day;
use crate::days;

pub struct Day9 {}

impl Day for Day9 {
  fn day_num(&self) -> usize {
    9
  }

  fn puzzle_1(&self, input: &Vec<String>) -> String {
    let nums = days::input_to_ints(input);

    find_first_not_matching(&nums, 25).to_string()
  }

  fn puzzle_2(&self, input: &Vec<String>) -> String {
    String::from("")
  }
}

fn find_first_not_matching(nums: &Vec<i64>, previous: usize) -> i64 {
  for i in previous..nums.len() {
    if !previous_add_to_current(&nums, i, previous) {
      return nums[i];
    }
  }

  -1
}

fn previous_add_to_current(nums: &Vec<i64>, current: usize, previous: usize) -> bool {
  let possible = get_previous(nums, current, previous);

  for (i, num1) in possible.iter().enumerate() {
    let rest = &possible[(i + 1)..];
    for num2 in rest {
      if num1 + num2 == nums[current] {
        return true;
      }
    }
  }

  false
}

fn get_previous<'a>(nums: &'a Vec<i64>, current_index: usize, to_grab: usize) -> &'a [i64] {
  let start = current_index - to_grab;
  &nums[start..current_index]
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_previous() {
    assert_eq!(get_previous(&vec![1,2,3,4,5], 4, 3), vec![2,3,4]);
  }

  #[test]
  fn test_previous_adds_to_current() {
    assert_eq!(previous_add_to_current(&vec![1,2,3,4,5], 4, 3), true);
    assert_eq!(previous_add_to_current(&vec![1,2,3,4,5], 4, 2), false);
  }

  fn sample_input() -> Vec<String> {
    vec![
      String::from("35"),
      String::from("20"),
      String::from("15"),
      String::from("25"),
      String::from("47"),
      String::from("40"),
      String::from("62"),
      String::from("55"),
      String::from("65"),
      String::from("95"),
      String::from("102"),
      String::from("117"),
      String::from("150"),
      String::from("182"),
      String::from("127"),
      String::from("219"),
      String::from("299"),
      String::from("277"),
      String::from("309"),
      String::from("576"),
    ]
  }

  #[test]
  fn test_find_first_not_matching() {
    assert_eq!(find_first_not_matching(&days::input_to_ints(&sample_input()), 5), 127);
  }
}
