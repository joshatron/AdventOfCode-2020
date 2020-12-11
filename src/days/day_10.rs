use crate::days::Day;
use crate::days;

pub struct Day10 {}

impl Day for Day10 {
  fn day_num(&self) -> usize {
    10
  }

  fn puzzle_1(&self, input: &Vec<String>) -> String {
    let mut ones = 0;
    let mut threes = 0;

    let mut jolts = days::input_to_ints(input);
    jolts.push(0);
    jolts.push(jolts.iter().max().unwrap() + 3);
    jolts.sort();

    for i in 0..(jolts.len() - 1) {
      if jolts[i + 1] - jolts[i] == 1 {
        ones += 1;
      } else if jolts[i + 1] - jolts[i] == 3 {
        threes += 1;
      }
    }

    (ones * threes).to_string()
  }

  fn puzzle_2(&self, input: &Vec<String>) -> String {
    let mut adapters = days::input_to_ints(input);
    adapters.sort();

    one_groups(&adapters).iter()
      .map(|g| group_to_permutations(*g))
      .fold(1, |acc, p| acc * p)
      .to_string()
  }
}

fn one_groups(adapters: &Vec<i64>) -> Vec<usize> {
  let mut groups = vec![];
  let mut current = 1;
  let mut previous = 0;

  for i in adapters {
    if i - previous == 1 {
      current += 1;
    } else {
      if current > 1 {
        groups.push(current);
      }
      current = 1;
    }
    previous = *i;
  }

  if current > 1 {
    groups.push(current);
  }


  groups
}

fn group_to_permutations(group_size: usize) -> usize {
  match group_size {
    2 => 1,
    3 => 2,
    4 => 4,
    5 => 7,
    _ => 1
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn sample_input_1() -> Vec<String> {
    vec![
      String::from("16"),
      String::from("10"),
      String::from("15"),
      String::from("5"),
      String::from("1"),
      String::from("11"),
      String::from("7"),
      String::from("19"),
      String::from("6"),
      String::from("12"),
      String::from("4"),
    ]
  }

  fn sample_input_2() -> Vec<String> {
    vec![
      String::from("28"),
      String::from("33"),
      String::from("18"),
      String::from("42"),
      String::from("31"),
      String::from("14"),
      String::from("46"),
      String::from("20"),
      String::from("48"),
      String::from("47"),
      String::from("24"),
      String::from("23"),
      String::from("49"),
      String::from("45"),
      String::from("19"),
      String::from("38"),
      String::from("39"),
      String::from("11"),
      String::from("1"),
      String::from("32"),
      String::from("25"),
      String::from("35"),
      String::from("8"),
      String::from("17"),
      String::from("7"),
      String::from("9"),
      String::from("4"),
      String::from("2"),
      String::from("34"),
      String::from("10"),
      String::from("3"),
    ]
  }

  #[test]
  fn test_puzzle_1() {
    assert_eq!(Day10{}.puzzle_1(&sample_input_1()), "35");
    assert_eq!(Day10{}.puzzle_1(&sample_input_2()), "220");
  }

  #[test]
  fn test_one_groups() {
    let mut first_sample = days::input_to_ints(&sample_input_1());
    first_sample.sort();
    assert_eq!(one_groups(&first_sample), vec![2, 4, 3, 2]);

    let mut second_sample = days::input_to_ints(&sample_input_2());
    second_sample.sort();
    assert_eq!(one_groups(&second_sample), vec![5, 5, 4, 3, 5, 2, 5]);
  }

  #[test]
  fn test_group_to_permutations() {
    assert_eq!(group_to_permutations(2), 1);
    assert_eq!(group_to_permutations(3), 2);
    assert_eq!(group_to_permutations(4), 4);
    assert_eq!(group_to_permutations(5), 7);
  }

  #[test]
  fn test_puzzle_2() {
    assert_eq!(Day10{}.puzzle_2(&sample_input_1()), "8");
    assert_eq!(Day10{}.puzzle_2(&sample_input_2()), "19208");
  }
}
