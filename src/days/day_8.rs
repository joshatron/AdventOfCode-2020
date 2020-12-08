use crate::days::Day;

pub struct Day8 {}

impl Day for Day8 {
  fn day_num(&self) -> usize {
    8
  }

  fn puzzle_1(&self, input: &Vec<String>) -> String {
    let mut program = input_program(input);

    String::from(program.run_till_terminate().expect_err("Should have errored out").to_string())
  }

  fn puzzle_2(&self, input: &Vec<String>) -> String {
    let mut var_input = input.clone();

    for i in 0..var_input.len() {
      if &var_input[i][0..3] == "nop" {
        let mut new_line = String::from("jmp");
        new_line.push_str(&var_input[i][3..]);
        var_input[i] = new_line;
      } else if &var_input[i][0..3] == "jmp" {
        var_input[i] = String::from("nop +0");
      }

      let mut program = input_program(&var_input);

      match program.run_till_terminate() {
        Ok(n) => return n.to_string(),
        Err(_) => var_input[i] = String::from(&input[i]),
      }
    }

    String::from("No command change fixed the program")
  }
}

struct Program {
  lines: Vec<ProgramLine>,
  acc: i64,
  current_line: usize,
}

impl Program {
  fn run_till_terminate(&mut self) -> Result<i64, i64> {
    while self.current_line < self.lines.len() &&
          !self.lines[self.current_line].visited {
      self.run_next_line();
    }

    if self.current_line == self.lines.len() {
      Ok(self.acc)
    } else {
      Err(self.acc)
    }
  }

  fn run_next_line(&mut self) {
    self.lines[self.current_line].visited = true;

    match self.lines[self.current_line].instruction {
      Instruction::Nop => {
        self.current_line = self.current_line + 1;
      },
      Instruction::Acc(n) => {
        self.acc = self.acc + n;
        self.current_line = self.current_line + 1;
      },
      Instruction::Jmp(n, pos) => {
        if pos {
          self.current_line = self.current_line + n;
        } else {
          self.current_line = self.current_line - n;
        }
      },
    }
  }
}


fn input_program(input: &Vec<String>) -> Program {
  Program {
    lines: input.iter().map(|l| line_to_program_line(l)).collect(),
    acc: 0,
    current_line: 0,
  }
}

struct ProgramLine {
  instruction: Instruction,
  visited: bool,
}

fn line_to_program_line(line: &str) -> ProgramLine {
  ProgramLine{
    instruction: to_instruction(line),
    visited: false,
  }
}

#[derive(Debug, PartialEq)]
enum Instruction {
  Nop,
  Acc(i64),
  Jmp(usize, bool),
}

fn to_instruction(line: &str) -> Instruction {
  match &line[0..3] {
    "nop" => Instruction::Nop,
    "acc" => Instruction::Acc(line[4..].parse::<i64>().unwrap()),
    "jmp" => Instruction::Jmp(line[5..].parse::<usize>().unwrap(), &line[4..5] == "+"),
    _ => Instruction::Nop
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_to_instruction_nop() {
    assert_eq!(to_instruction("nop +0"), Instruction::Nop);
  }

  #[test]
  fn test_to_instruction_acc() {
    assert_eq!(to_instruction("acc +1"), Instruction::Acc(1));
    assert_eq!(to_instruction("acc +5"), Instruction::Acc(5));
    assert_eq!(to_instruction("acc -99"), Instruction::Acc(-99));
  }

  #[test]
  fn test_to_instruction_jmp() {
    assert_eq!(to_instruction("jmp +5"), Instruction::Jmp(5, true));
    assert_eq!(to_instruction("jmp -86"), Instruction::Jmp(86, false));
  }

  fn sample_input() -> Vec<String> {
    vec![
      String::from("nop +0"),
      String::from("acc +1"),
      String::from("jmp +4"),
      String::from("acc +3"),
      String::from("jmp -3"),
      String::from("acc -99"),
      String::from("acc +1"),
      String::from("jmp -4"),
      String::from("acc +6"),
    ]
  }

  #[test]
  fn test_input_program() {
    let program = input_program(&sample_input());
    assert_eq!(program.acc, 0);
    assert_eq!(program.current_line, 0);
    assert_eq!(program.lines.len(), 9);
    assert_eq!(program.lines[0].instruction, Instruction::Nop);
    assert_eq!(program.lines[0].visited, false);
    assert_eq!(program.lines[8].instruction, Instruction::Acc(6));
    assert_eq!(program.lines[8].visited, false);
  }

  #[test]
  fn test_run_next_line() {
    let mut program = input_program(&sample_input());
    program.run_next_line();
    assert_eq!(program.acc, 0);
    assert_eq!(program.current_line, 1);
    assert_eq!(program.lines[0].visited, true);
    program.run_next_line();
    assert_eq!(program.acc, 1);
    assert_eq!(program.current_line, 2);
    assert_eq!(program.lines[1].visited, true);
    program.run_next_line();
    assert_eq!(program.acc, 1);
    assert_eq!(program.current_line, 6);
    assert_eq!(program.lines[2].visited, true);
  }

  #[test]
  fn test_puzzle_1() {
    assert_eq!(Day8{}.puzzle_1(&sample_input()), "5");
  }

  #[test]
  fn test_puzzle_3() {
    assert_eq!(Day8{}.puzzle_2(&sample_input()), "8");
  }
}
