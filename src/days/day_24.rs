use crate::days::Day;
use std::collections::HashSet;

pub struct Day24 {}

impl Day24 {
  pub fn new() -> Day24 {
    Day24 {}
  }
}

impl Day for Day24 {
  fn day_num(&self) -> usize {
    24
  }

  fn puzzle_1(&self, input: &Vec<String>) -> String {
    let tiles = Tiles::parse(input);

    tiles.black_tiles().to_string()
  }

  fn puzzle_2(&self, input: &Vec<String>) -> String {
    let mut tiles = Tiles::parse(input);

    for _ in 0..100 {
      tiles.next_day();
    }

    tiles.black_tiles().to_string()
  }
}

struct Directions {
  directions: Vec<Direction>,
}

impl Directions {
  fn parse(line: &str) -> Directions {
    let mut directions = Directions {
      directions: Vec::new(),
    };

    let converted_line = line
      .replace("ne", "1")
      .replace("nw", "2")
      .replace("se", "3")
      .replace("sw", "4")
      .replace("e", "5")
      .replace("w", "6");

    for c in converted_line.chars() {
      directions.directions.push(match c {
        '1' => Direction::NorthEast,
        '2' => Direction::NorthWest,
        '3' => Direction::SouthEast,
        '4' => Direction::SouthWest,
        '5' => Direction::East,
        '6' => Direction::West,
        _ => Direction::NorthEast,
      });
    }

    directions
  }

  fn get_loc(&self) -> Point {
    let mut point = Point::new();
    for dir in &self.directions {
      point.move_dir(dir);
    }

    point
  }
}

#[derive(Debug, PartialEq)]
enum Direction {
  NorthEast,
  East,
  SouthEast,
  SouthWest,
  West,
  NorthWest,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
  x: isize,
  y: isize,
}

impl Point {
  fn new() -> Point {
    Point { x: 0, y: 0 }
  }

  fn move_dir(&mut self, direction: &Direction) {
    match direction {
      Direction::East => self.x += 1,
      Direction::NorthEast => self.y += 1,
      Direction::NorthWest => {
        self.x -= 1;
        self.y += 1;
      }
      Direction::West => self.x -= 1,
      Direction::SouthWest => self.y -= 1,
      Direction::SouthEast => {
        self.x += 1;
        self.y -= 1;
      }
    }
  }

  fn surrounding(&self) -> Vec<Point> {
    let mut surrounding = Vec::new();

    surrounding.push(Point {
      x: self.x + 1,
      y: self.y,
    });
    surrounding.push(Point {
      x: self.x,
      y: self.y + 1,
    });
    surrounding.push(Point {
      x: self.x - 1,
      y: self.y + 1,
    });
    surrounding.push(Point {
      x: self.x - 1,
      y: self.y,
    });
    surrounding.push(Point {
      x: self.x,
      y: self.y - 1,
    });
    surrounding.push(Point {
      x: self.x + 1,
      y: self.y - 1,
    });

    surrounding
  }
}

struct Tiles {
  flipped: HashSet<Point>,
}

impl Tiles {
  fn parse(input: &Vec<String>) -> Tiles {
    let mut tiles = Tiles::new();
    for line in input {
      let directions = Directions::parse(line);
      tiles.flip(directions.get_loc());
    }

    tiles
  }

  fn new() -> Tiles {
    Tiles {
      flipped: HashSet::new(),
    }
  }

  fn black_tiles(&self) -> usize {
    self.flipped.len()
  }

  fn flip(&mut self, point: Point) {
    if self.flipped.contains(&point) {
      self.flipped.remove(&point);
    } else {
      self.flipped.insert(point);
    }
  }

  fn next_day(&mut self) {
    let mut new_tiles = HashSet::new();

    for x in self.min_x()..self.max_x() {
      for y in self.min_y()..self.max_y() {
        let point = Point { x: x, y: y };
        let surrounding = self.num_surrounding(&point);

        if (self.flipped.contains(&point) && surrounding == 1 || surrounding == 2)
          || (!self.flipped.contains(&point) && surrounding == 2)
        {
          new_tiles.insert(point);
        }
      }
    }

    self.flipped = new_tiles;
  }

  fn num_surrounding(&self, point: &Point) -> usize {
    let mut surrounding = 0;

    for p in point.surrounding() {
      if self.flipped.contains(&p) {
        surrounding += 1;
      }
    }

    surrounding
  }

  fn min_x(&self) -> isize {
    let mut min_x = isize::MAX;

    for point in &self.flipped {
      if point.x < min_x {
        min_x = point.x
      }
    }

    min_x - 3
  }

  fn min_y(&self) -> isize {
    let mut min_y = isize::MAX;

    for point in &self.flipped {
      if point.y < min_y {
        min_y = point.y
      }
    }

    min_y - 2
  }

  fn max_x(&self) -> isize {
    let mut max_x = isize::MIN;

    for point in &self.flipped {
      if point.x > max_x {
        max_x = point.x
      }
    }

    max_x + 3
  }

  fn max_y(&self) -> isize {
    let mut max_y = isize::MIN;

    for point in &self.flipped {
      if point.y > max_y {
        max_y = point.y
      }
    }

    max_y + 2
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_directions() {
    let directions_1 = Directions::parse("esew");
    assert_eq!(
      directions_1.directions,
      vec![Direction::East, Direction::SouthEast, Direction::West]
    );
    let directions_2 = Directions::parse("swneeesewnww");
    assert_eq!(
      directions_2.directions,
      vec![
        Direction::SouthWest,
        Direction::NorthEast,
        Direction::East,
        Direction::East,
        Direction::SouthEast,
        Direction::West,
        Direction::NorthWest,
        Direction::West
      ]
    );
  }

  #[test]
  fn test_move_point() {
    let mut point = Point::new();
    assert_eq!(point, Point { x: 0, y: 0 });
    point.move_dir(&Direction::East);
    assert_eq!(point, Point { x: 1, y: 0 });
    point.move_dir(&Direction::NorthEast);
    assert_eq!(point, Point { x: 1, y: 1 });
    point.move_dir(&Direction::NorthWest);
    assert_eq!(point, Point { x: 0, y: 2 });
    point.move_dir(&Direction::West);
    assert_eq!(point, Point { x: -1, y: 2 });
    point.move_dir(&Direction::SouthWest);
    assert_eq!(point, Point { x: -1, y: 1 });
    point.move_dir(&Direction::SouthEast);
    assert_eq!(point, Point { x: 0, y: 0 });
  }

  #[test]
  fn test_directions_location() {
    let directions_1 = Directions::parse("esew");
    assert_eq!(directions_1.get_loc(), Point { x: 1, y: -1 });
    let directions_2 = Directions::parse("nwwswee");
    assert_eq!(directions_2.get_loc(), Point { x: 0, y: 0 });
  }

  #[test]
  fn test_flip_tile() {
    let mut tiles = Tiles::new();
    assert_eq!(tiles.black_tiles(), 0);
    tiles.flip(Point { x: 1, y: -1 });
    assert_eq!(tiles.black_tiles(), 1);
    tiles.flip(Point { x: 0, y: 0 });
    assert_eq!(tiles.black_tiles(), 2);
    tiles.flip(Point { x: 1, y: -1 });
    assert_eq!(tiles.black_tiles(), 1);
  }

  fn sample_input() -> Vec<String> {
    vec![
      String::from("sesenwnenenewseeswwswswwnenewsewsw"),
      String::from("neeenesenwnwwswnenewnwwsewnenwseswesw"),
      String::from("seswneswswsenwwnwse"),
      String::from("nwnwneseeswswnenewneswwnewseswneseene"),
      String::from("swweswneswnenwsewnwneneseenw"),
      String::from("eesenwseswswnenwswnwnwsewwnwsene"),
      String::from("sewnenenenesenwsewnenwwwse"),
      String::from("wenwwweseeeweswwwnwwe"),
      String::from("wsweesenenewnwwnwsenewsenwwsesesenwne"),
      String::from("neeswseenwwswnwswswnw"),
      String::from("nenwswwsewswnenenewsenwsenwnesesenew"),
      String::from("enewnwewneswsewnwswenweswnenwsenwsw"),
      String::from("sweneswneswneneenwnewenewwneswswnese"),
      String::from("swwesenesewenwneswnwwneseswwne"),
      String::from("enesenwswwswneneswsenwnewswseenwsese"),
      String::from("wnwnesenesenenwwnenwsewesewsesesew"),
      String::from("nenewswnwewswnenesenwnesewesw"),
      String::from("eneswnwswnwsenenwnwnwwseeswneewsenese"),
      String::from("neswnwewnwnwseenwseesewsenwsweewe"),
      String::from("wseweeenwnesenwwwswnew"),
    ]
  }

  #[test]
  fn test_puzzle_1() {
    assert_eq!(Day24::new().puzzle_1(&sample_input()), "10");
  }

  #[test]
  fn test_next_day() {
    let mut tiles = Tiles::parse(&sample_input());
    tiles.next_day();
    assert_eq!(tiles.black_tiles(), 15);
    tiles.next_day();
    assert_eq!(tiles.black_tiles(), 12);
    tiles.next_day();
    assert_eq!(tiles.black_tiles(), 25);
  }

  #[test]
  fn test_puzzle_2() {
    assert_eq!(Day24::new().puzzle_2(&sample_input()), "2208");
  }
}
