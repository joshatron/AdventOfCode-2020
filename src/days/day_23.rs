use crate::days::Day;

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
    let mut cups = Cups::parse(input, 10);
    for _ in 0..100 {
      cups.step_one();
    }

    let mut order = String::new();
    let mut spot = cups.cups[1];
    for _ in 0..8 {
      order.push_str(&spot.to_string());
      spot = cups.cups[spot];
    }

    order
  }

  fn puzzle_2(&self, input: &Vec<String>) -> String {
    let mut cups = Cups::parse(input, 1000001);
    for _ in 0..10000000 {
      cups.step_one();
    }

    let first = cups.cups[1];
    let second = cups.cups[first];

    (first * second).to_string()
  }
}

struct Cups {
  cups: Vec<usize>,
  current: usize,
}

impl Cups {
  fn parse(input: &Vec<String>, size: usize) -> Cups {
    let mut cups = Cups {
      cups: vec![0; size],
      current: 0,
    };

    let mut last = 0;
    for c in input[0].chars() {
      let num = c.to_string().parse::<usize>().unwrap();
      if last == 0 {
        cups.current = num;
      } else {
        cups.cups[last] = num;
      }
      last = num;
    }

    for i in 10..size {
      cups.cups[last] = i;
      last = i;
    }
    cups.cups[last] = cups.current;

    cups
  }

  fn step_one(&mut self) {
    let next = self.get_nth_after_current(1);
    let next_three = self.get_nth_after_current(3);
    let next_below = find_next_below_big(
      self.current,
      self.get_nth_after_current(1),
      self.get_nth_after_current(2),
      self.get_nth_after_current(3),
      self.cups.len() - 1,
    );
    let after_next_below = self.cups[next_below];
    let after_current = self.get_nth_after_current(4);
    self.cups[next_below] = next;
    self.cups[next_three] = after_next_below;
    self.cups[self.current] = after_current;
    self.current_to_next();
  }

  fn get_nth_after_current(&self, amount: usize) -> usize {
    let mut temp_current = self.current;
    for _ in 0..amount {
      temp_current = self.cups[temp_current];
    }

    temp_current
  }

  fn current_to_next(&mut self) {
    self.current = self.cups[self.current];
  }
}

fn find_next_below_big(
  start: usize,
  skip_one: usize,
  skip_two: usize,
  skip_three: usize,
  size: usize,
) -> usize {
  let mut below = if start == 1 { size } else { start - 1 };
  while below == skip_one || below == skip_two || below == skip_three {
    if below == 1 {
      below = size;
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
    let cups = Cups::parse(&sample_input(), 10);
    assert_eq!(cups.cups[1], 2);
    assert_eq!(cups.cups[3], 8);
    assert_eq!(cups.cups[7], 3);
    assert_eq!(cups.current, 3);
  }

  #[test]
  fn test_step_one() {
    let mut cups = Cups::parse(&sample_input(), 10);
    cups.step_one();
    assert_eq!(cups.cups, [0, 5, 8, 2, 6, 4, 7, 3, 9, 1]);
    assert_eq!(cups.current, 2);
    cups.step_one();
    assert_eq!(cups.cups, [0, 3, 5, 2, 6, 4, 7, 8, 9, 1]);
    assert_eq!(cups.current, 5);
  }

  #[test]
  fn test_puzzle_1() {
    assert_eq!(Day23::new().puzzle_1(&sample_input()), "67384529");
  }
}
