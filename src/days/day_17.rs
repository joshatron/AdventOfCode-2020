use crate::days::Day;
use std::collections::HashSet;

pub struct Day17 {}

impl Day17 {
  pub fn new() -> Day17 {
    Day17{}
  }
}

impl Day for Day17 {
  fn day_num(&self) -> usize {
    17
  }

  fn puzzle_1(&self, input: &Vec<String>) -> String {
    let mut area = Area::parse(input);
    for _ in 0..6 {
      area.do_cycle();
    }

    area.active_points.len().to_string()
  }

  fn puzzle_2(&self, input: &Vec<String>) -> String {
    let mut area = Area4D::parse(input);
    for _ in 0..6 {
      area.do_cycle();
    }

    area.active_points.len().to_string()
  }
}

struct Area {
  active_points: HashSet<Point>,
}

impl Area {
  fn set_active(&mut self, point: Point) {
    self.active_points.insert(point);
  }

  fn is_active(&self, point: &Point) -> bool {
    self.active_points.contains(point)
  }

  fn do_cycle(&mut self) {
    let mut new_active = HashSet::new();

    for z in (self.active_points.iter().map(|p| p.z).min().unwrap() - 1)..(self.active_points.iter().map(|p| p.z).max().unwrap() + 2) {
      for y in (self.active_points.iter().map(|p| p.y).min().unwrap() - 1)..(self.active_points.iter().map(|p| p.y).max().unwrap() + 2) {
        for x in (self.active_points.iter().map(|p| p.x).min().unwrap() - 1)..(self.active_points.iter().map(|p| p.x).max().unwrap() + 2) {
          let point = Point::new(x, y, z);
          let is_active = self.is_active(&point);
          if (is_active && self.surrounded_by(&point, 2, 3)) ||
             (!is_active && self.surrounded_by(&point, 3, 3)) {
            new_active.insert(point);
          } 
        }
      }
    }

    self.active_points = new_active;
  }

  fn surrounded_by(&self, point: &Point, min: usize, max: usize) -> bool {
    let mut surrounding = 0;
    for p in point.get_surrounding() {
      if self.is_active(&p) {
        surrounding += 1;
      }

      if surrounding > max {
        return false;
      }
    }

    surrounding >= min
  }

  fn parse(input: &Vec<String>) -> Area {
    let mut area = Area {
      active_points: HashSet::new(),
    };

    for (y, line) in input.iter().enumerate() {
      for (x, c) in line.chars().enumerate() {
        if c == '#' {
          area.set_active(Point::new(x as isize, y as isize, 0));
        }
      }
    }

    area
  }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
  x: isize,
  y: isize,
  z: isize,
}

impl Point {
  fn get_surrounding(&self) -> Vec<Point> {
    let mut surrounding = Vec::new();

    for z in (self.z-1)..(self.z+2) {
      for y in (self.y-1)..(self.y+2) {
        for x in (self.x-1)..(self.x+2) {
          let point = Point::new(x, y, z);
          if &point != self {
            surrounding.push(point);
          }
        }
      }
    }

    surrounding
  }

  fn new(x: isize, y: isize, z: isize) -> Point {
    Point {
      x: x,
      y: y,
      z: z,
    }
  }
}

struct Area4D {
  active_points: HashSet<Point4D>,
}

impl Area4D {
  fn set_active(&mut self, point: Point4D) {
    self.active_points.insert(point);
  }

  fn is_active(&self, point: &Point4D) -> bool {
    self.active_points.contains(point)
  }

  fn do_cycle(&mut self) {
    let mut new_active = HashSet::new();
    let max_z = self.active_points.iter().map(|p| p.z).max().unwrap() + 2;
    let min_y = self.active_points.iter().map(|p| p.y).min().unwrap() - 1;
    let max_y = self.active_points.iter().map(|p| p.y).max().unwrap() + 2;
    let min_x = self.active_points.iter().map(|p| p.x).min().unwrap() - 1;
    let max_x = self.active_points.iter().map(|p| p.x).max().unwrap() + 2;
    let max_w = self.active_points.iter().map(|p| p.w).max().unwrap() + 2;

    for z in 0..max_z {
      for y in min_y..max_y {
        for x in min_x..max_x{
          for w in 0..max_w {
            let point = Point4D::new(x, y, z, w);
            let is_active = self.is_active(&point);
            if (is_active && self.surrounded_by(&point, 2, 3)) ||
              (!is_active && self.surrounded_by(&point, 3, 3)) {
              if z != 0 && w != 0 {
                new_active.insert(Point4D::new(x, y, z, w));
                new_active.insert(Point4D::new(x, y, z * -1, w));
                new_active.insert(Point4D::new(x, y, z, w * -1));
                new_active.insert(Point4D::new(x, y, z * -1, w * -1));
              } else if z != 0 {
                new_active.insert(Point4D::new(x, y, z, w));
                new_active.insert(Point4D::new(x, y, z * -1, w));
              } else if w != 0 {
                new_active.insert(Point4D::new(x, y, z, w));
                new_active.insert(Point4D::new(x, y, z, w * -1));
              } else {
                new_active.insert(point);
              }
            } 
          }
        }
      }
    }

    self.active_points = new_active;
  }

  fn surrounded_by(&self, point: &Point4D, min: usize, max: usize) -> bool {
    let mut surrounding = 0;
    for p in point.get_surrounding() {
      if self.is_active(&p) {
        surrounding += 1;
      }

      if surrounding > max {
        return false;
      }
    }

    surrounding >= min
  }

  fn parse(input: &Vec<String>) -> Area4D {
    let mut area = Area4D {
      active_points: HashSet::new(),
    };

    for (y, line) in input.iter().enumerate() {
      for (x, c) in line.chars().enumerate() {
        if c == '#' {
          area.set_active(Point4D::new(x as isize, y as isize, 0, 0));
        }
      }
    }

    area
  }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point4D {
  x: isize,
  y: isize,
  z: isize,
  w: isize,
}

impl Point4D {
  fn get_surrounding(&self) -> Vec<Point4D> {
    let mut surrounding = Vec::new();

    for z in (self.z-1)..(self.z+2) {
      for y in (self.y-1)..(self.y+2) {
        for x in (self.x-1)..(self.x+2) {
          for w in (self.w-1)..(self.w+2) {
            let point = Point4D::new(x, y, z, w);
            if &point != self {
              surrounding.push(point);
            }
          }
        }
      }
    }

    surrounding
  }

  fn new(x: isize, y: isize, z: isize, w: isize) -> Point4D {
    Point4D {
      x: x,
      y: y,
      z: z,
      w: w,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn sample_input() -> Vec<String> {
    vec![
      String::from(".#."),
      String::from("..#"),
      String::from("###"),
    ]
  }

  #[test]
  fn test_init_area() {
    let area = Area::parse(&sample_input());
    assert_eq!(area.is_active(&Point::new(0,0,0)), false);
    assert_eq!(area.is_active(&Point::new(1,0,0)), true);
    assert_eq!(area.is_active(&Point::new(1,1,0)), false);
    assert_eq!(area.is_active(&Point::new(2,1,0)), true);
    assert_eq!(area.is_active(&Point::new(3,4,5)), false);
  }

  #[test]
  fn test_get_surrounding() {
    let point = Point::new(0, 0, 0);
    let surrounding = point.get_surrounding();
    assert_eq!(surrounding.len(), 26);
    assert_eq!(surrounding.contains(&Point::new(0, 1, 0)), true);
    assert_eq!(surrounding.contains(&Point::new(0, 0, 0)), false);
  }

  #[test]
  fn test_surrounded_by() {
    let area = Area::parse(&sample_input());
    assert_eq!(area.surrounded_by(&Point::new(0,1,0), 3, 3), true);
    assert_eq!(area.surrounded_by(&Point::new(2,2,1), 3, 3), true);
  }

  #[test]
  fn test_do_cycle() {
    let mut area = Area::parse(&sample_input());
    area.do_cycle();
    assert_eq!(area.is_active(&Point::new(0,1,0)), true);
    assert_eq!(area.is_active(&Point::new(2,2,1)), true);
  }

  #[test]
  fn test_puzzle_1() {
    assert_eq!(Day17{}.puzzle_1(&sample_input()), "112");
  }
}
