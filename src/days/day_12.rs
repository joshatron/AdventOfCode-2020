use crate::days::Day;

pub struct Day12 {}

impl Day for Day12 {
  fn day_num(&self) -> usize {
    12
  }

  fn puzzle_1(&self, input: &Vec<String>) -> String {
    let mut ship = Ship::new();

    for instruction in input.iter().map(|l| parse_instruction(l)) {
      ship.run_instruction(&instruction);
    }

    (ship.x.abs() + ship.y.abs()).to_string()
  }

  fn puzzle_2(&self, input: &Vec<String>) -> String {
    let mut ship = ShipWithWaypoint::new();

    for instruction in input.iter().map(|l| parse_instruction(l)) {
      ship.run_instruction(&instruction);
    }

    (ship.x.abs() + ship.y.abs()).to_string()
  }
}

#[derive(Debug, PartialEq)]
struct Ship {
  x: isize,
  y: isize,
  facing: Direction
}

impl Ship {
  fn run_instruction(&mut self, instruction: &Instruction) {
    match instruction {
      Instruction::Move(dir, dist) => self.move_in_dir(&dir, *dist),
      Instruction::Turn(amount) => self.turn(*amount),
      Instruction::Forward(dist) => self.move_in_dir(&self.facing.clone(), *dist),
      Instruction::None => println!("Shouldn't have gotten here.")
    }
  }

  fn move_in_dir(&mut self, dir: &Direction, dist: isize) {
    match dir {
      Direction::North => self.y += dist,
      Direction::South => self.y -= dist,
      Direction::East => self.x += dist,
      Direction::West => self.x -= dist,
    }
  }

  fn turn(&mut self, amount: isize) {
    let right = amount > 0;

    for _ in 0..amount.abs() {
      if right {
        self.turn_right_one()
      } else {
        self.turn_left_one()
      }
    }
  }

  fn turn_left_one(&mut self) {
    match &self.facing {
      Direction::North => self.facing = Direction::West,
      Direction::South => self.facing = Direction::East,
      Direction::East => self.facing = Direction::North,
      Direction::West => self.facing = Direction::South,
    }
  }

  fn turn_right_one(&mut self) {
    match &self.facing {
      Direction::North => self.facing = Direction::East,
      Direction::South => self.facing = Direction::West,
      Direction::East => self.facing = Direction::South,
      Direction::West => self.facing = Direction::North,
    }
  }

  fn new() -> Ship {
    Ship{
      x: 0,
      y: 0,
      facing: Direction::East
    }
  }
}

struct ShipWithWaypoint {
  x: isize,
  y: isize,
  waypoint_x: isize,
  waypoint_y: isize,
}

impl ShipWithWaypoint {
  fn run_instruction(&mut self, instruction: &Instruction) {
    match instruction {
      Instruction::Move(dir, dist) => self.move_waypoint_in_dir(&dir, *dist),
      Instruction::Turn(amount) => self.turn_waypoint(*amount),
      Instruction::Forward(dist) => self.move_to_waypoint(*dist),
      Instruction::None => println!("Shouldn't have gotten here.")
    }
  }

  fn move_waypoint_in_dir(&mut self, dir: &Direction, dist: isize) {
    match dir {
      Direction::North => self.waypoint_y += dist,
      Direction::South => self.waypoint_y -= dist,
      Direction::East => self.waypoint_x += dist,
      Direction::West => self.waypoint_x -= dist,
    }
  }

  fn move_to_waypoint(&mut self, times: isize) {
    for _ in 0..times.abs() {
      self.move_to_waypoint_once();
    }
  }

  fn move_to_waypoint_once(&mut self) {
    self.x += self.waypoint_x;
    self.y += self.waypoint_y;
  }

  fn turn_waypoint(&mut self, amount: isize) {
    let right = amount > 0;

    for _ in 0..amount.abs() {
      if right {
        self.turn_waypoint_right_one()
      } else {
        self.turn_waypoint_left_one()
      }
    }
  }

  fn turn_waypoint_left_one(&mut self) {
    let temp_x = self.waypoint_x;
    self.waypoint_x = self.waypoint_y * -1;
    self.waypoint_y = temp_x;
  }

  fn turn_waypoint_right_one(&mut self) {
    let temp_x = self.waypoint_x;
    self.waypoint_x = self.waypoint_y;
    self.waypoint_y = temp_x * -1;
  }

  fn new() -> ShipWithWaypoint {
    ShipWithWaypoint {
      x: 0,
      y: 0,
      waypoint_x: 10,
      waypoint_y: 1,
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
  North,
  South,
  East,
  West
}

#[derive(Debug, PartialEq)]
enum Instruction {
  Move(Direction, isize),
  Turn(isize),
  Forward(isize),
  None
}

fn parse_instruction(line: &str) -> Instruction {
  let num = line[1..].parse::<isize>().unwrap();
  match &line[0..1] {
    "N" => Instruction::Move(Direction::North, num),
    "S" => Instruction::Move(Direction::South, num),
    "E" => Instruction::Move(Direction::East, num),
    "W" => Instruction::Move(Direction::West, num),
    "L" => Instruction::Turn((num / 90) * -1),
    "R" => Instruction::Turn(num / 90),
    "F" => Instruction::Forward(num),
    _ => Instruction::None,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_instruction() {
    assert_eq!(parse_instruction("N3"), Instruction::Move(Direction::North, 3));
    assert_eq!(parse_instruction("S12"), Instruction::Move(Direction::South, 12));
    assert_eq!(parse_instruction("E1"), Instruction::Move(Direction::East, 1));
    assert_eq!(parse_instruction("W197"), Instruction::Move(Direction::West, 197));
    assert_eq!(parse_instruction("L90"), Instruction::Turn(-1));
    assert_eq!(parse_instruction("R270"), Instruction::Turn(3));
    assert_eq!(parse_instruction("F8"), Instruction::Forward(8));
  }

  #[test]
  fn test_create_ship() {
    let ship = Ship::new();
    assert_eq!(ship.x, 0);
    assert_eq!(ship.y, 0);
    assert_eq!(ship.facing, Direction::East);
  }

  #[test]
  fn test_run_direction() {
    let mut ship = Ship::new();
    ship.run_instruction(&Instruction::Move(Direction::North, 3));
    ship.run_instruction(&Instruction::Move(Direction::South, 4));
    ship.run_instruction(&Instruction::Move(Direction::East, 5));
    ship.run_instruction(&Instruction::Move(Direction::West, 2));
    ship.run_instruction(&Instruction::Turn(4));
    ship.run_instruction(&Instruction::Turn(-6));
    ship.run_instruction(&Instruction::Forward(5));

    assert_eq!(ship.x, -2);
    assert_eq!(ship.y, -1);
    assert_eq!(ship.facing, Direction::West);
  }

  fn sample_input() -> Vec<String> {
    vec![
      String::from("F10"),
      String::from("N3"),
      String::from("F7"),
      String::from("R90"),
      String::from("F11"),
    ]
  }

  #[test]
  fn test_puzzle_1() {
    assert_eq!(Day12{}.puzzle_1(&sample_input()), "25");
  }

  #[test]
  fn test_ship_with_waypoint_run_instruction() {
    let mut ship = ShipWithWaypoint::new();

    ship.run_instruction(&Instruction::Move(Direction::North, 3));
    ship.run_instruction(&Instruction::Move(Direction::South, 4));
    ship.run_instruction(&Instruction::Move(Direction::East, 5));
    ship.run_instruction(&Instruction::Move(Direction::West, 2));
    ship.run_instruction(&Instruction::Turn(4));
    ship.run_instruction(&Instruction::Turn(-6));
    ship.run_instruction(&Instruction::Forward(5));

    assert_eq!(ship.x, -65);
    assert_eq!(ship.y, 0);
    assert_eq!(ship.waypoint_x, -13);
    assert_eq!(ship.waypoint_y, 0);
  }

  #[test]
  fn test_puzzle_2() {
    assert_eq!(Day12{}.puzzle_2(&sample_input()), "286");
  }
}
