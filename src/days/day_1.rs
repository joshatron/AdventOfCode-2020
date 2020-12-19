use crate::days;
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
    let ints = days::input_to_ints(input);

    for (i, &num) in ints.iter().enumerate() {
      let rest = &ints[(i + 1)..];

      for other in rest {
        if num + other == 2020 {
          return (num * other).to_string();
        }
      }
    }

    String::from("")
  }
  
  fn puzzle_2(&self, input: &Vec<String>) -> String {
    let ints = days::input_to_ints(input);

    for (i, &first) in ints.iter().enumerate() {
      let second_set = &ints[(i + 1)..];

      for (j, &second) in second_set.iter().enumerate() {
        let third_set = &second_set[(j + 1)..];

        for third in third_set {
          if first + second + third == 2020 {
            return (first * second * third).to_string();
          }
        }
      } 
    }
    String::from("")
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn puzzle_1_sample_input() {
    let input = vec![String::from("1721"), String::from("979"), String::from("366"), String::from("299"), String::from("675"), String::from("1456")];

    assert_eq!(Day1{}.puzzle_1(&input), String::from("514579"));
  }

  #[test]
  fn puzzle_2_sample_input() {
    let input = vec![String::from("1721"), String::from("979"), String::from("366"), String::from("299"), String::from("675"), String::from("1456")];

    assert_eq!(Day1{}.puzzle_2(&input), String::from("241861950"));
  }
}