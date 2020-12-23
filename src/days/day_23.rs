use crate::days::Day;
use std::collections::HashMap;

pub struct Day23 {}

impl Day23 {
  pub fn new() -> Day23 {
    Day23 {}
  }
}

impl Day for Day23 {
  fn day_num(&self) -> usize {
    23
  }

  fn puzzle_1(&self, input: &Vec<String>) -> String {
    let mut cups = Cups::parse(input);
    for _ in 0..100 {
      cups.step_one();
    }

    let mut order = String::new();
    let mut spot = up_one_wrap(cups.find_cup(1));
    for _ in 0..8 {
      order.push_str(&cups.get_cup_at_loc(spot).to_string());
      spot = up_one_wrap(spot);
    }

    order
  }

  fn puzzle_2(&self, input: &Vec<String>) -> String {
    let mut cups = BigCups::parse(input);
    for _ in 0..10000000 {
      cups.step_one();
    }

    let first = *cups.cups.get(&1).unwrap();
    let second = *cups.cups.get(&first).unwrap();

    (first * second).to_string()
  }
}

struct Cups {
  cups: [usize; 9],
  current: usize,
}

impl Cups {
  fn parse(input: &Vec<String>) -> Cups {
    let mut cups = Cups {
      cups: [0; 9],
      current: 0,
    };

    for (i, c) in input[0].chars().enumerate() {
      cups.cups[i] = c.to_string().parse::<usize>().unwrap();
    }

    cups
  }

  fn get_cup_at_loc(&self, loc: usize) -> usize {
    self.cups[loc % 9]
  }

  fn current_cup(&self) -> usize {
    self.get_cup_at_loc(self.current)
  }

  fn current_cup_loc(&self) -> usize {
    self.current
  }

  fn find_cup(&self, cup: usize) -> usize {
    for (i, c) in self.cups.iter().enumerate() {
      if &cup == c {
        return i;
      }
    }

    0
  }

  fn step_one(&mut self) {
    let next_one = self.get_cup_at_loc(up_n_wrap(self.current, 1));
    let next_two = self.get_cup_at_loc(up_n_wrap(self.current, 2));
    let next_three = self.get_cup_at_loc(up_n_wrap(self.current, 3));
    let next_below = find_next_below(self.current_cup(), next_one, next_two, next_three);

    let mut loc = (self.current + 4) % 9;
    let mut done = false;
    while !done {
      if self.get_cup_at_loc(loc) == next_below {
        done = true;
      }
      self.shift_left_n(loc, 3);
      loc = up_one_wrap(loc);
    }

    self.cups[down_n_wrap(loc, 3)] = next_one;
    self.cups[down_n_wrap(loc, 2)] = next_two;
    self.cups[down_n_wrap(loc, 1)] = next_three;
    self.current = up_one_wrap(self.current);
  }

  fn shift_left_n(&mut self, loc: usize, amount: usize) {
    let mut current_loc = loc;
    for _ in 0..amount {
      self.shift_left_one(current_loc);
      current_loc = down_one_wrap(current_loc);
    }
  }

  fn shift_left_one(&mut self, loc: usize) {
    self.cups[down_one_wrap(loc)] = self.get_cup_at_loc(loc);
  }
}

fn find_next_below(start: usize, skip_one: usize, skip_two: usize, skip_three: usize) -> usize {
  let mut below = if start == 1 { 9 } else { start - 1 };
  while below == skip_one || below == skip_two || below == skip_three {
    if below == 1 {
      below = 9;
    } else {
      below = below - 1;
    }
  }

  below
}

fn down_n_wrap(start: usize, amount: usize) -> usize {
  let mut current = start;
  for _ in 0..amount {
    current = down_one_wrap(current);
  }

  current
}

fn down_one_wrap(start: usize) -> usize {
  if start == 0 {
    8
  } else {
    start - 1
  }
}

fn up_n_wrap(start: usize, amount: usize) -> usize {
  let mut current = start;
  for _ in 0..amount {
    current = up_one_wrap(current);
  }

  current
}

fn up_one_wrap(start: usize) -> usize {
  (start + 1) % 9
}

struct BigCups {
  cups: HashMap<usize, usize>,
  current: usize,
}

impl BigCups {
  fn parse(input: &Vec<String>) -> BigCups {
    let mut cups = BigCups {
      cups: HashMap::new(),
      current: 0,
    };

    let mut last = 0;
    for c in input[0].chars() {
      let num = c.to_string().parse::<usize>().unwrap();
      if last == 0 {
        cups.current = num;
      } else {
        cups.cups.insert(last, num);
      }
      last = num;
    }

    for i in 10..1000001 {
      cups.cups.insert(last, i);
      last = i;
    }
    cups.cups.insert(last, cups.current);

    cups
  }

  fn step_one(&mut self) {
    let next_below = find_next_below_big(
      self.current,
      self.get_nth_after_current(1),
      self.get_nth_after_current(2),
      self.get_nth_after_current(3),
    );
    let after_next_below = *self.cups.get(&next_below).unwrap();
    let after_current = self.get_nth_after_current(4);
    self.cups.insert(next_below, self.get_nth_after_current(1));
    self
      .cups
      .insert(self.get_nth_after_current(3), after_next_below);
    self.cups.insert(self.current, after_current);
    self.current_to_next();
  }

  fn get_nth_after_current(&self, amount: usize) -> usize {
    let mut temp_current = self.current;
    for _ in 0..amount {
      temp_current = *self.cups.get(&temp_current).unwrap();
    }

    temp_current
  }

  fn current_to_next(&mut self) {
    self.current = *self.cups.get(&self.current).unwrap();
  }
}

fn find_next_below_big(start: usize, skip_one: usize, skip_two: usize, skip_three: usize) -> usize {
  let mut below = if start == 1 { 1000000 } else { start - 1 };
  while below == skip_one || below == skip_two || below == skip_three {
    if below == 1 {
      below = 1000000;
    } else {
      below = below - 1;
    }
  }

  below
}

#[cfg(test)]
mod tests {
  use super::*;

  fn sample_input() -> Vec<String> {
    vec![String::from("389125467")]
  }

  #[test]
  fn test_parse_cups() {
    let cups = Cups::parse(&sample_input());
    assert_eq!(cups.get_cup_at_loc(0), 3);
    assert_eq!(cups.get_cup_at_loc(8), 7);
    assert_eq!(cups.get_cup_at_loc(9), 3);
  }

  #[test]
  fn test_current_cup() {
    let cups = Cups::parse(&sample_input());
    assert_eq!(cups.current_cup(), 3);
    assert_eq!(cups.current_cup_loc(), 0);
  }

  #[test]
  fn test_step_one() {
    let mut cups = Cups::parse(&sample_input());
    cups.step_one();
    assert_eq!(cups.cups, [3, 2, 8, 9, 1, 5, 4, 6, 7]);
    assert_eq!(cups.current_cup(), 2);
    cups.step_one();
    assert_eq!(cups.cups, [3, 2, 5, 4, 6, 7, 8, 9, 1]);
    assert_eq!(cups.current_cup(), 5);
  }

  #[test]
  fn test_puzzle_1() {
    assert_eq!(Day23::new().puzzle_1(&sample_input()), "67384529");
  }

  #[test]
  fn test_puzzle_2() {
    assert_eq!(Day23::new().puzzle_2(&sample_input()), "149245887792");
  }
}
