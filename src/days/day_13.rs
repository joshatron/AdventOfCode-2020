use std::convert::TryInto;
use crate::days::Day;

pub struct Day13 {}

impl Day13 {
  pub fn new() -> Day13 {
    Day13{}
  }
}

impl Day for Day13 {
  fn day_num(&self) -> usize {
    13
  }

  fn puzzle_1(&self, input: &Vec<String>) -> String {
    let arrival = input[0].parse::<u64>().unwrap();
    let buses = get_buses(&input[1]);

    let mut min_time = 999999;
    let mut min_id = 0;
    for bus in buses {
      let time = get_wait_time(arrival, bus);
      if time < min_time {
        min_time = time;
        min_id = bus;
      }
    }

    (min_time * min_id).to_string()
  }

  fn puzzle_2(&self, input: &Vec<String>) -> String {
    let buses = get_all_buses(&input[1]);

    let mut current = 1;
    let mut adder = 1;

    for (i, &bus) in buses.iter().enumerate() {
      if bus != 1 {
        let mut new_i: u64 = i.try_into().unwrap();
        if new_i >= bus {
          new_i = new_i % bus;
        }

        while get_wait_time(current, bus) != new_i {
          current += adder;
        }

        adder *= bus;
      }
    }

    current.to_string()
  }
} 

fn get_buses(line: &str) -> Vec<u64> {
  line.split(",")
    .map(|b| b.parse::<u64>())
    .filter(|r| r.is_ok())
    .map(|r| r.unwrap())
    .collect()
}

fn get_wait_time(start: u64, bus: u64) -> u64 {
  if start % bus != 0 {
    ((start / bus + 1) * bus) - start
  } else {
    0
  }
}

fn get_all_buses(line: &str) -> Vec<u64> {
  line.split(",")
    .map(|b| b.parse::<u64>().unwrap_or(1))
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_buses() {
    assert_eq!(get_buses("7,13,x,x,59,x,31,19"), vec![7, 13, 59, 31, 19]);
  }

  #[test]
  fn test_get_wait_time() {
    assert_eq!(get_wait_time(939, 59), 5);
    assert_eq!(get_wait_time(939, 7), 6);
  }

  fn sample_input() -> Vec<String> {
    vec![
      String::from("939"),
      String::from("7,13,x,x,59,x,31,19")
    ]
  }

  #[test]
  fn test_puzzle_1() {
    assert_eq!(Day13{}.puzzle_1(&sample_input()), "295");
  }

  #[test]
  fn test_get_all_buses() {
    assert_eq!(get_all_buses("7,13,x,x,59,x,31,19"), vec![7, 13, 1, 1, 59, 1, 31, 19]);
  }

  #[test]
  fn test_puzzle_2() {
    assert_eq!(Day13{}.puzzle_2(&vec![String::from(""), String::from("17,x,13,19")]), "3417");
    assert_eq!(Day13{}.puzzle_2(&vec![String::from(""), String::from("67,7,59,61")]), "754018");
    assert_eq!(Day13{}.puzzle_2(&vec![String::from(""), String::from("67,x,7,59,61")]), "779210");
    assert_eq!(Day13{}.puzzle_2(&vec![String::from(""), String::from("67,7,x,59,61")]), "1261476");
    assert_eq!(Day13{}.puzzle_2(&vec![String::from(""), String::from("1789,37,47,1889")]), "1202161486");
    assert_eq!(Day13{}.puzzle_2(&sample_input()), "1068781");
  }
}
