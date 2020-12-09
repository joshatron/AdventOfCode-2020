use crate::days::Day;
use crate::days;
use std::cmp;

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
    let nums = days::input_to_ints(input);

    let not_matching = find_first_not_matching(&nums, 25);
    let adding_to = find_contiguous_adding_to_num(&nums, not_matching).unwrap();

    (adding_to.iter().min().unwrap() + adding_to.iter().max().unwrap()).to_string()
  }
}

fn find_contiguous_adding_to_num<'a>(nums: &'a Vec<i64>, num: i64) -> Result<&'a [i64], String> {
  for i in 0..nums.len() {
    for j in (i+1)..nums.len() {
      let slice = &nums[i..j];
      let added = add_slice(slice);

      if added > num {
        break;
      } else if added == num {
        return Ok(slice);
      }
    }
  }

  Err(String::from("Could not find contiguous numbers adding to value"))
}

fn add_slice(slice: &[i64]) -> i64 {
  slice.iter()
    .fold(0, |acc, n| acc + n)
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

  #[test]
  fn test_find_contiguous_adding_to_num() {
    let expected_result = vec![15, 25, 47, 40];
    assert_eq!(find_contiguous_adding_to_num(&days::input_to_ints(&sample_input()), 127), Ok(&expected_result[0..4]));
  }
}
