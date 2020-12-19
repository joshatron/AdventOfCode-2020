use crate::days::Day;

pub struct Day6 {}

impl Day6 {
  pub fn new() -> Day6 {
    Day6{}
  }
}

impl Day for Day6 {
  fn day_num(&self) -> usize {
    6
  }

  fn puzzle_1(&self, input: &Vec<String>) -> String {
    into_groups(input).iter()
      .fold(0, |acc, b| acc + anyone(&b))
      .to_string()
  }

  fn puzzle_2(&self, input: &Vec<String>) -> String {
    into_groups(input).iter()
      .fold(0, |acc, b| acc + everyone(&b))
      .to_string()
  }
}

fn into_groups(input: &Vec<String>) -> Vec<Vec<String>> {
  let mut groups = vec![];

  let mut current_group = vec![];
  for line in input {
    if line.is_empty() {
      groups.push(current_group);
      current_group = vec![];
    } else {
      current_group.push(String::from(line));
    }
  }

  groups.push(current_group);

  groups
}

fn anyone(group: &Vec<String>) -> usize {
  let mut chars: Vec<char> = group.concat().chars().collect();
  chars.sort();
  chars.dedup();

  chars.len()
}

fn everyone(group: &Vec<String>) -> usize {
  let mut all = 0;
  for c in group[0].chars() {
    if group.iter().all(|g| g.contains(&c.to_string())) {
      all = all + 1
    }
  }

  all
}

#[cfg(test)]
mod tests {
  use super::*;

  fn sample_data() -> Vec<String> {
    vec![
      String::from("abc"),
      String::from(""),
      String::from("a"),
      String::from("b"),
      String::from("c"),
      String::from(""),
      String::from("ab"),
      String::from("ac"),
      String::from(""),
      String::from("a"),
      String::from("a"),
      String::from("a"),
      String::from("a"),
      String::from(""),
      String::from("b"),
    ]
  }

  #[test]
  fn test_into_groups() {
    let groups = into_groups(&sample_data());

    assert_eq!(groups[0], vec!["abc"]);
    assert_eq!(groups[1], vec!["a", "b", "c"]);
    assert_eq!(groups[2], vec!["ab", "ac"]);
    assert_eq!(groups[3], vec!["a", "a", "a", "a"]);
    assert_eq!(groups[4], vec!["b"]);
  }

  #[test]
  fn test_anyone() {
    assert_eq!(anyone(&vec![String::from("abc")]), 3);
    assert_eq!(anyone(&vec![String::from("a"), String::from("b"), String::from("c")]), 3);
    assert_eq!(anyone(&vec![String::from("ab"), String::from("ac")]), 3);
    assert_eq!(anyone(&vec![String::from("a"), String::from("a"), String::from("a")]), 1);
  }

  #[test]
  fn test_everyone() {
    assert_eq!(everyone(&vec![String::from("abc")]), 3);
    assert_eq!(everyone(&vec![String::from("a"), String::from("b"), String::from("c")]), 0);
    assert_eq!(everyone(&vec![String::from("ab"), String::from("ac")]), 1);
    assert_eq!(everyone(&vec![String::from("a"), String::from("a"), String::from("a")]), 1);
  }
}