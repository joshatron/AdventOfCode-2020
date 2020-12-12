use crate::days::Day;

pub struct Day12 {}

impl Day for Day12 {
  fn day_num(&self) -> usize {
    12
  }

  fn puzzle_1(&self, input: &Vec<String>) -> String {
    String::from("")
  }

  fn puzzle_2(&self, input: &Vec<String>) -> String {
    String::from("")
  }
}

#[derive(Debug, PartialEq)]
struct Ship {
  x: isize,
  y: isize,
  facing: Direction
}

#[derive(Debug, PartialEq)]
enum Direction {
  North,
  South,
  East,
  West
}

fn create_ship() -> Ship {
  Ship{
    x: 0,
    y: 0,
    facing: Direction::East
  }
}

#[derive(Debug, PartialEq)]
enum Instruction {
  North(usize),
  South(usize),
  East(usize),
  West(usize),
  Left(usize),
  Right(usize),
  Forward(usize),
  None
}

fn parse_instruction(line: &str) -> Instruction {
  let num = line[1..].parse::<usize>().unwrap();
  match &line[0..1] {
    "N" => Instruction::North(num),
    "S" => Instruction::South(num),
    "E" => Instruction::East(num),
    "W" => Instruction::West(num),
    "L" => Instruction::Left(num / 90),
    "R" => Instruction::Right(num / 90),
    "F" => Instruction::Forward(num),
    _ => Instruction::None,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_instruction() {
    assert_eq!(parse_instruction("N3"), Instruction::North(3));
    assert_eq!(parse_instruction("S12"), Instruction::South(12));
    assert_eq!(parse_instruction("E1"), Instruction::East(1));
    assert_eq!(parse_instruction("W197"), Instruction::West(197));
    assert_eq!(parse_instruction("L90"), Instruction::Left(1));
    assert_eq!(parse_instruction("R270"), Instruction::Right(3));
    assert_eq!(parse_instruction("F8"), Instruction::Forward(8));
  }

  #[test]
  fn test_create_ship() {
    let ship = create_ship();
    assert_eq!(ship.x, 0);
    assert_eq!(ship.y, 0);
    assert_eq!(ship.facing, Direction::East);
  }

}
