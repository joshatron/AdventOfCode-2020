use crate::days::Day;

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
    String::from("")
  }

  fn puzzle_2(&self, input: &Vec<String>) -> String {
    String::from("")
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

#[derive(Debug, PartialEq)]
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
      _ => (),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_directions() {
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
  fn move_point() {
    let mut point = Point::new();
    assert_eq!(point, Point { x: 0, y: 0 });
    point.move_dir(&Direction::East);
    assert_eq!(point, Point { x: 1, y: 0 });
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
    assert_eq!(Day24::new().puzzle_1(&sample_input()), "");
  }

  #[test]
  fn test_puzzle_2() {
    assert_eq!(Day24::new().puzzle_2(&sample_input()), "");
  }
}
