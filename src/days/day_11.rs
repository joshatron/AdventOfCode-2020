use crate::days::Day;
use std::convert::TryInto;

pub struct Day11 {}

impl Day11 {
  pub fn new() -> Day11 {
    Day11 {}
  }
}

impl Day for Day11 {
  fn day_num(&self) -> usize {
    11
  }

  fn puzzle_1(&self, input: &Vec<String>) -> String {
    let final_arrangment = get_final_arrangement(input, ArrangementMethod::One);

    final_arrangment.count_total(&Spot::Filled).to_string()
  }

  fn puzzle_2(&self, input: &Vec<String>) -> String {
    let final_arrangment = get_final_arrangement(input, ArrangementMethod::Two);

    final_arrangment.count_total(&Spot::Filled).to_string()
  }
}

#[derive(Debug, PartialEq)]
struct WaitingArea {
  spots: Vec<Vec<Spot>>,
}

impl WaitingArea {
  fn width(&self) -> usize {
    self.spots[0].len()
  }

  fn height(&self) -> usize {
    self.spots.len()
  }

  fn at_spot(&self, x: usize, y: usize) -> &Spot {
    &self.spots[y][x]
  }

  fn at_spot_safe(&self, x: isize, y: isize) -> &Spot {
    if !self.in_bounds(x, y) {
      &Spot::Floor
    } else {
      self.at_spot(x.try_into().unwrap(), y.try_into().unwrap())
    }
  }

  fn in_bounds(&self, x: isize, y: isize) -> bool {
    x >= 0
      && x < self.width().try_into().unwrap()
      && y >= 0
      && y < self.height().try_into().unwrap()
  }

  fn surrounding(&self, x: usize, y: usize) -> usize {
    let isize_x: isize = x.try_into().unwrap();
    let isize_y: isize = y.try_into().unwrap();

    let mut occupied = 0;

    for nx in (isize_x - 1)..(isize_x + 2) {
      for ny in (isize_y - 1)..(isize_y + 2) {
        if (nx != isize_x || ny != isize_y) && self.at_spot_safe(nx, ny) == &Spot::Filled {
          occupied += 1;
        }
      }
    }

    occupied
  }

  fn can_see_less_than(&self, x: usize, y: usize, less_than: usize) -> bool {
    let mut see = 0;
    for dir in vec![
      Direction::North,
      Direction::NorthEast,
      Direction::East,
      Direction::SouthEast,
      Direction::South,
      Direction::SouthWest,
      Direction::West,
      Direction::NorthWest,
    ] {
      if self.can_see_in_dir(x, y, dir) {
        see += 1;
        if see >= less_than {
          return false;
        }
      }
    }

    true
  }

  fn can_see_in_dir(&self, x: usize, y: usize, dir: Direction) -> bool {
    let mut current_point = Point {
      x: x.try_into().unwrap(),
      y: y.try_into().unwrap(),
    };

    current_point.move_1(&dir);

    while self.in_bounds(current_point.x, current_point.y) {
      if self.at_spot_safe(current_point.x, current_point.y) == &Spot::Filled {
        return true;
      } else if self.at_spot_safe(current_point.x, current_point.y) == &Spot::Empty {
        return false;
      }

      current_point.move_1(&dir);
    }

    false
  }

  fn count_total(&self, spot_type: &Spot) -> usize {
    let mut total = 0;
    for row in &self.spots {
      for spot in row {
        if spot_type == spot {
          total += 1;
        }
      }
    }

    total
  }

  fn parse(input: &Vec<String>) -> WaitingArea {
    let mut spots = Vec::new();

    for line in input {
      let mut row = Vec::new();
      for c in line.chars() {
        match c {
          'L' => row.push(Spot::Empty),
          '.' => row.push(Spot::Floor),
          '#' => row.push(Spot::Filled),
          _ => row.push(Spot::Floor),
        }
      }

      spots.push(row);
    }

    WaitingArea { spots: spots }
  }
}

enum Direction {
  North,
  NorthEast,
  East,
  SouthEast,
  South,
  SouthWest,
  West,
  NorthWest,
}

struct Point {
  x: isize,
  y: isize,
}

impl Point {
  fn move_1(&mut self, dir: &Direction) {
    match dir {
      Direction::North => {
        self.y -= 1;
      }
      Direction::NorthEast => {
        self.y -= 1;
        self.x += 1;
      }
      Direction::East => {
        self.x += 1;
      }
      Direction::SouthEast => {
        self.y += 1;
        self.x += 1;
      }
      Direction::South => {
        self.y += 1;
      }
      Direction::SouthWest => {
        self.y += 1;
        self.x -= 1;
      }
      Direction::West => {
        self.x -= 1;
      }
      Direction::NorthWest => {
        self.y -= 1;
        self.x -= 1;
      }
    }
  }
}

#[derive(Debug, PartialEq)]
enum Spot {
  Floor,
  Empty,
  Filled,
}

enum ArrangementMethod {
  One,
  Two,
}

fn get_final_arrangement(input: &Vec<String>, arrangement: ArrangementMethod) -> WaitingArea {
  let mut current = WaitingArea::parse(input);
  let mut next = get_next_arrangment(&current, &arrangement);

  while current != next {
    current = next;
    next = get_next_arrangment(&current, &arrangement);
  }

  current
}

fn get_next_arrangment(current: &WaitingArea, arrangement: &ArrangementMethod) -> WaitingArea {
  let mut spots = Vec::new();
  for y in 0..current.height() {
    let mut row = Vec::new();
    for x in 0..current.width() {
      match arrangement {
        ArrangementMethod::One => row.push(next_spot_1(&current, x, y)),
        ArrangementMethod::Two => row.push(next_spot_2(&current, x, y)),
      }
    }
    spots.push(row);
  }

  WaitingArea { spots: spots }
}

fn next_spot_1(current: &WaitingArea, x: usize, y: usize) -> Spot {
  match current.at_spot(x, y) {
    Spot::Floor => Spot::Floor,
    Spot::Empty => {
      if current.surrounding(x, y) == 0 {
        Spot::Filled
      } else {
        Spot::Empty
      }
    }
    Spot::Filled => {
      if current.surrounding(x, y) < 4 {
        Spot::Filled
      } else {
        Spot::Empty
      }
    }
  }
}

fn next_spot_2(current: &WaitingArea, x: usize, y: usize) -> Spot {
  match current.at_spot(x, y) {
    Spot::Floor => Spot::Floor,
    Spot::Empty => {
      if current.can_see_less_than(x, y, 1) {
        Spot::Filled
      } else {
        Spot::Empty
      }
    }
    Spot::Filled => {
      if current.can_see_less_than(x, y, 5) {
        Spot::Filled
      } else {
        Spot::Empty
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn sample_input() -> Vec<String> {
    vec![
      String::from("L.LL.LL.LL"),
      String::from("LLLLLLL.LL"),
      String::from("L.L.L..L.."),
      String::from("LLLL.LL.LL"),
      String::from("L.LL.LL.LL"),
      String::from("L.LLLLL.LL"),
      String::from("..L.L....."),
      String::from("LLLLLLLLLL"),
      String::from("L.LLLLLL.L"),
      String::from("L.LLLLL.LL"),
    ]
  }

  #[test]
  fn test_create_waiting_area() {
    let waiting_area = WaitingArea::parse(&sample_input());
    assert_eq!(waiting_area.width(), 10);
    assert_eq!(waiting_area.height(), 10);
    assert_eq!(waiting_area.at_spot(0, 0), &Spot::Empty);
    assert_eq!(waiting_area.at_spot(1, 0), &Spot::Floor);
    assert_eq!(waiting_area.at_spot(9, 9), &Spot::Empty);
    assert_eq!(waiting_area.at_spot(7, 6), &Spot::Floor);
    assert_eq!(waiting_area.at_spot(0, 9), &Spot::Empty);
  }

  #[test]
  fn test_surrounding() {
    let waiting_area = WaitingArea {
      spots: vec![
        vec![Spot::Filled, Spot::Filled, Spot::Filled],
        vec![Spot::Floor, Spot::Empty, Spot::Floor],
        vec![Spot::Filled, Spot::Filled, Spot::Filled],
      ],
    };
    assert_eq!(waiting_area.surrounding(1, 1), 6);
    assert_eq!(waiting_area.surrounding(0, 0), 1);
    assert_eq!(waiting_area.surrounding(2, 2), 1);
  }

  #[test]
  fn test_get_next_arrangement_1() {
    let mut waiting_area = WaitingArea::parse(&sample_input());
    waiting_area = get_next_arrangment(&waiting_area, &ArrangementMethod::One);
    assert_eq!(waiting_area.at_spot(0, 0), &Spot::Filled);
    assert_eq!(waiting_area.at_spot(1, 0), &Spot::Floor);
    assert_eq!(waiting_area.at_spot(9, 9), &Spot::Filled);
    assert_eq!(waiting_area.at_spot(7, 6), &Spot::Floor);
    assert_eq!(waiting_area.at_spot(0, 9), &Spot::Filled);
  }

  #[test]
  fn test_puzzle_1() {
    assert_eq!(Day11 {}.puzzle_1(&sample_input()), "37");
  }

  #[test]
  fn test_get_next_arrangement_2() {
    let mut waiting_area = WaitingArea::parse(&sample_input());
    waiting_area = get_next_arrangment(&waiting_area, &ArrangementMethod::Two);
    waiting_area = get_next_arrangment(&waiting_area, &ArrangementMethod::Two);
    assert_eq!(waiting_area.at_spot(0, 0), &Spot::Filled);
    assert_eq!(waiting_area.at_spot(8, 0), &Spot::Empty);
  }

  #[test]
  fn test_puzzle_2() {
    assert_eq!(Day11 {}.puzzle_2(&sample_input()), "26");
  }
}
