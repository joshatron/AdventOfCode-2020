use crate::days::Day;
use std::convert::TryInto;
use std::collections::HashMap;

pub struct Day14 {}

impl Day14 {
  pub fn new() -> Day14 {
    Day14{}
  }
}

impl Day for Day14 {
  fn day_num(&self) -> usize {
    14
  }

  fn puzzle_1(&self, input: &Vec<String>) -> String {
    let mut computer = Computer::new();
    for command in input.iter().map(|l| parse_command(l)) {
      computer.run_command(command);
    }

    computer.memory.iter().fold(0, |acc, m| acc + m.1).to_string()
  }

  fn puzzle_2(&self, input: &Vec<String>) -> String {
    let mut computer = Computer2::new();
    for command in input.iter().map(|l| parse_command(l)) {
      computer.run_command(command);
    }

    computer.memory.iter().fold(0, |acc, m| acc + m.1).to_string()
  }
}

struct Computer {
  memory: HashMap<u64, u64>,
  bitmask_ones: u64,
  bitmask_zeros: u64,
}

impl Computer {
  fn run_command(&mut self, command: Command) {
    match command {
      Command::SetMask(mask) => self.set_mask(mask),
      Command::SetMemory(address, value) => self.set_memory(address, value),
      Command::None => println!("No action for None command")
    }
  }

  fn set_mask(&mut self, new_mask: &str) {
    self.bitmask_ones = u64::from_str_radix(&new_mask.replace("X", "0"), 2).unwrap();
    let mut zeros_str = String::from("1111111111111111111111111111");
    zeros_str.push_str(&new_mask.replace("X", "1"));
    self.bitmask_zeros = u64::from_str_radix(&zeros_str, 2).unwrap();
  }

  fn set_memory(&mut self, address: u64, value: u64) {
    self.memory.insert(address, self.get_bitmasked_number(value));
  }

  fn get_bitmasked_number(&self, value: u64) -> u64 {
    (value | self.bitmask_ones) & self.bitmask_zeros
  }


  fn new() -> Computer {
    Computer {
      memory: HashMap::new(),
      bitmask_ones: 0b0,
      bitmask_zeros: !0b0,
    }
  }
}

struct Computer2 {
  memory: HashMap<u64, u64>,
  bitmask_ones: u64,
  x_bits: Vec<usize>
}

impl Computer2 {
  fn run_command(&mut self, command: Command) {
    match command {
      Command::SetMask(mask) => self.set_mask(mask),
      Command::SetMemory(address, value) => self.set_memory(address, value),
      Command::None => println!("No action for None command")
    }
  }

  fn set_mask(&mut self, new_mask: &str) {
    self.bitmask_ones = u64::from_str_radix(&new_mask.replace("X", "0"), 2).unwrap();

    self.x_bits = vec![];
    for (i, c) in new_mask.chars().rev().enumerate() {
      if c == 'X' {
        self.x_bits.push(i);
      }
    }
  }

  fn set_memory(&mut self, address: u64, value: u64) {
    let base_address = address | self.bitmask_ones;

    for i in 0..usize::pow(2, self.x_bits.len().try_into().unwrap()) {
      let mut new_address = base_address;
      for (j, x_bit) in self.x_bits.iter().enumerate() {
        let mut indicator = String::from("1");
        let zeros: String = vec!['0'; j].into_iter().collect();
        indicator.push_str(&zeros);

        if usize::from_str_radix(&indicator, 2).unwrap() & i != 0 {
          new_address = get_memory_address_one(new_address, *x_bit);
        } else {
          new_address = get_memory_address_zero(new_address, *x_bit);
        }
      }

      self.memory.insert(new_address, value);
    }
  }

  fn new() -> Computer2 {
    Computer2 {
      memory: HashMap::new(),
      bitmask_ones: 0b0,
      x_bits: vec![],
    }
  }
}

fn get_memory_address_one(address: u64, x_bit: usize) -> u64 {
  let mut bitmask = String::from("1");
  let zeros: String = vec!['0'; x_bit].into_iter().collect();
  bitmask.push_str(&zeros);

  address | u64::from_str_radix(&bitmask, 2).unwrap()
}

fn get_memory_address_zero(address: u64, x_bit: usize) -> u64 {
  let mut bitmask: String = vec!['1'; 63 - x_bit].into_iter().collect();
  bitmask.push_str("0");
  let ones: String = vec!['1'; x_bit].into_iter().collect();
  bitmask.push_str(&ones);

  address & u64::from_str_radix(&bitmask, 2).unwrap()
}

#[derive(Debug, PartialEq)]
enum Command<'a> {
  SetMask(&'a str),
  SetMemory(u64, u64),
  None
}

fn parse_command(line: &str) -> Command {
  match &line[0..4] {
    "mask" => Command::SetMask(&line[7..]),
    "mem[" => Command::SetMemory(get_index(line), get_num(line)),
    _ => Command::None
  }
}

fn get_index(line: &str) -> u64 {
  line.split("]").next().unwrap()[4..].parse::<u64>().unwrap()
}

fn get_num(line: &str) -> u64 {
  let mut parts = line.split(" = ");
  parts.next();
  parts.next().unwrap().parse::<u64>().unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_create_computer() {
    let computer = Computer::new();
    assert_eq!(computer.memory.len(), 0);
    assert_eq!(computer.bitmask_ones, 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
    assert_eq!(computer.bitmask_zeros, 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111);
  }

  #[test]
  fn test_parse_command() {
    assert_eq!(parse_command("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"), 
               Command::SetMask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"));
    assert_eq!(parse_command("mem[8] = 11"), Command::SetMemory(8, 11));
    assert_eq!(parse_command("mem[186] = 1786"), Command::SetMemory(186, 1786));
  }

  #[test]
  fn test_run_command() {
    let mut computer = Computer::new();
    computer.run_command(Command::SetMask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"));
    assert_eq!(computer.bitmask_ones, 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0100_0000);
    assert_eq!(computer.bitmask_zeros, 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1101);
    computer.run_command(Command::SetMemory(8, 11));
    assert_eq!(computer.memory.get(&8).unwrap(), &73);
    computer.run_command(Command::SetMemory(7, 101));
    assert_eq!(computer.memory.get(&7).unwrap(), &101);
    computer.run_command(Command::SetMemory(8, 0));
    assert_eq!(computer.memory.get(&8).unwrap(), &64);
  }

  fn sample_input_1() -> Vec<String> {
    vec![
      String::from("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"),
      String::from("mem[8] = 11"),
      String::from("mem[7] = 101"),
      String::from("mem[8] = 0"),
    ]
  }

  #[test]
  fn test_puzzle_1() {
    assert_eq!(Day14{}.puzzle_1(&sample_input_1()), "165");
  }

  #[test]
  fn test_create_computer_2() {
    let computer = Computer2::new();
    assert_eq!(computer.memory.len(), 0);
    assert_eq!(computer.bitmask_ones, 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
    assert_eq!(computer.x_bits, vec![]);
  }

  #[test]
  fn test_run_command_2() {
    let mut computer = Computer2::new();
    computer.run_command(Command::SetMask("000000000000000000000000000000X1001X"));
    assert_eq!(computer.bitmask_ones, 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001_0010);
    assert_eq!(computer.x_bits, vec![0, 5]);
    computer.run_command(Command::SetMemory(42, 100));
    assert_eq!(computer.memory.get(&26).unwrap(), &100);
    assert_eq!(computer.memory.get(&27).unwrap(), &100);
    assert_eq!(computer.memory.get(&58).unwrap(), &100);
    assert_eq!(computer.memory.get(&59).unwrap(), &100);
  }

  fn sample_input_2() -> Vec<String>{
    vec![
      String::from("mask = 000000000000000000000000000000X1001X"),
      String::from("mem[42] = 100"),
      String::from("mask = 00000000000000000000000000000000X0XX"),
      String::from("mem[26] = 1"),
    ]
  }

  #[test]
  fn test_puzzle_2() {
    assert_eq!(Day14{}.puzzle_2(&sample_input_2()), "208");
  }
}
