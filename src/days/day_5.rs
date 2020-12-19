use crate::days::Day;

pub struct Day5 {}

impl Day5 {
  pub fn new() -> Day5 {
    Day5{}
  }
}

impl Day for Day5 {
  fn day_num(&self) -> usize {
    5
  }

  fn puzzle_1(&self, input: &Vec<String>) -> String {
    let mut highest = 0;
    for line in input {
      let seat = Seat::parse(line);
      if seat.get_seat_id() > highest {
        highest = seat.get_seat_id();
      }
    }

    highest.to_string()
  }

  fn puzzle_2(&self, input: &Vec<String>) -> String {
    let mut seat_ids: Vec<usize> = input.iter()
      .map(|l| Seat::parse(l))
      .map(|s| s.get_seat_id())
      .collect();
    seat_ids.sort();
    
    let mut ids = seat_ids.iter().peekable();
    while let Some(id) = ids.next() {
      if **ids.peek().unwrap_or(&&9999999) - id == 2 {
        return (id + 1).to_string();
      }
    }

    String::from("Plane full")
  }
}

#[derive(Debug, PartialEq)]
struct Seat{
    row: usize,
    column: usize,
}

impl Seat {
  fn get_seat_id(&self) -> usize {
    self.row * 8 + self.column
  }

  fn parse(instructions: &str) -> Seat {
    Seat {
      row: get_row(&instructions[..7]),
      column: get_column(&instructions[7..]),
    }
  }
}

fn get_row(instructions: &str) -> usize {
  let mut range  = (0, 127);
  for c in instructions.chars() {
    range = next_range(&range, c == 'B');
  }

  range.0
}

fn get_column(instructions: &str) -> usize {
  let mut range  = (0, 7);
  for c in instructions.chars() {
    range = next_range(&range, c == 'R');
  }

  range.0
}

fn next_range(current_range: &(usize, usize), go_high: bool) -> (usize, usize) {
  if go_high {
    (((current_range.1 + current_range.0) / 2) + 1, current_range.1)
  } else {
    (current_range.0, ((current_range.1 + current_range.0) / 2))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_seat() {
    assert_eq!(Seat::parse("FBFBBFFRLR"), Seat{row: 44, column: 5});
    assert_eq!(Seat::parse("BFFFBBFRRR"), Seat{row: 70, column: 7});
    assert_eq!(Seat::parse("FFFBBBFRRR"), Seat{row: 14, column: 7});
    assert_eq!(Seat::parse("BBFFBBFRLL"), Seat{row: 102, column: 4});
  }

  #[test]
  fn test_get_row() {
    assert_eq!(get_row("FBFBBFF"), 44);
    assert_eq!(get_row("BFFFBBF"), 70);
  }

  #[test]
  fn test_get_column() {
    assert_eq!(get_column("RLR"), 5);
    assert_eq!(get_column("RRR"), 7);
  }

  #[test]
  fn test_next_range() {
    assert_eq!(next_range(&(0,127), false), (0,63));
    assert_eq!(next_range(&(0,63), true), (32,63));
    assert_eq!(next_range(&(32,63), false), (32,47));
  }

  #[test]
  fn test_get_seat_id() {
    assert_eq!(Seat{row: 44, column: 5}.get_seat_id(), 357);
    assert_eq!(Seat{row: 70, column: 7}.get_seat_id(), 567);
  }
}
