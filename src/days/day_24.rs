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

    let mut to_check: HashSet<Point> = HashSet::new();
    for t in &self.flipped {
      to_check.insert(Point { x: t.x, y: t.y });
      for p in t.surrounding() {
        to_check.insert(p);
      }
    }

    for p in to_check {
      let surrounding = self.num_surrounding(&p, 2);

      if (self.flipped.contains(&p) && (surrounding == 1 || surrounding == 2))
        || (!self.flipped.contains(&p) && surrounding == 2)
      {
        new_tiles.insert(p);
      }
    }

    self.flipped = new_tiles;
  }

  fn num_surrounding(&self, point: &Point, max: usize) -> usize {
    let mut surrounding = 0;

    for p in point.surrounding() {
      if self.flipped.contains(&p) {
        surrounding += 1;
        if surrounding == max + 1 {
          return surrounding;
        }
      }
    }

    surrounding
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
