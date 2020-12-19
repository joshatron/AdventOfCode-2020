use crate::days::Day;

pub struct Day18 {}

impl Day18 {
  pub fn new() -> Day18 {
    Day18{}
  }
}

impl Day for Day18 {
  fn day_num(&self) -> usize {
    18
  }

  fn puzzle_1(&self, input: &Vec<String>) -> String {
    let mut expressions: Vec<Expression> = input.iter()
      .map(|l| Expression::parse(l))
      .collect();

    let mut sum = 0;
    for e in &mut expressions {
      sum += e.reduce();
    }

    sum.to_string()
  }

  fn puzzle_2(&self, input: &Vec<String>) -> String {
    let mut expressions: Vec<Expression> = input.iter()
      .map(|l| Expression::parse(l))
      .collect();

    let mut sum = 0;
    for e in &mut expressions {
      sum += e.reduce_with_precedence();
    }

    sum.to_string()
  }
}

struct Expression {
  elements: Vec<ExpressionElement>,
}

impl Expression {
  fn parse(line: &str) -> Expression {
    let mut expression = Expression {
      elements: Vec::new(),
    };

    let mut num_buffer = String::new();
    for c in line.chars() {
      if c.is_numeric() {
        num_buffer.push(c);
      } else if !num_buffer.is_empty() {
        expression.elements.push(ExpressionElement::Number(num_buffer.parse::<i64>().unwrap()));
        num_buffer = String::new();
      }

      if c == '+' {
        expression.elements.push(ExpressionElement::Add);
      } else if c == '*' {
        expression.elements.push(ExpressionElement::Multiply);
      } else if c == '(' {
        expression.elements.push(ExpressionElement::OpenParenthesis);
      } else if c == ')' {
        expression.elements.push(ExpressionElement::CloseParenthesis);
      }
    }

    if !num_buffer.is_empty() {
      expression.elements.push(ExpressionElement::Number(num_buffer.parse::<i64>().unwrap()));
    }

    expression
  }

  fn reduce(&mut self) -> i64 {
    while self.elements.len() > 1 {
      self.reduce_one_pass();
    }

    match self.elements[0] {
      ExpressionElement::Number(a) => a,
      _ => 0
    }
  }

  fn reduce_one_pass(&mut self) {
    let mut i = 0;
    while &(i + 2) < &self.elements.len() {
      match (&self.elements[i], &self.elements[i + 1], &self.elements[i + 2]) {
        (ExpressionElement::Number(a), ExpressionElement::Add, ExpressionElement::Number(b)) => {
          let new_number = ExpressionElement::Number(a + b);
          self.elements.insert(i, new_number);
          self.elements.remove(i + 3);
          self.elements.remove(i + 2);
          self.elements.remove(i + 1);
          break;
        },
        (ExpressionElement::Number(a), ExpressionElement::Multiply, ExpressionElement::Number(b)) => {
          let new_number = ExpressionElement::Number(a * b);
          self.elements.insert(i, new_number);
          self.elements.remove(i + 3);
          self.elements.remove(i + 2);
          self.elements.remove(i + 1);
          break;
        },
        (ExpressionElement::OpenParenthesis, ExpressionElement::Number(_), ExpressionElement::CloseParenthesis) => {
          self.elements.remove(i + 2);
          self.elements.remove(i);
          break;
        }
        _ => {
          i += 1;
        }
      }
    }
  }

  fn reduce_with_precedence(&mut self) -> i64 {
    while self.elements.len() > 1 {
      self.reduce_innermost_parenthesis();
    }

    match self.elements[0] {
      ExpressionElement::Number(a) => a,
      _ => 0
    }
  }

  fn reduce_innermost_parenthesis(&mut self) {
    let mut start = 0;

    for i in 0..self.elements.len() {
      if self.elements[i] == ExpressionElement::OpenParenthesis {
        start = i;
      } else if self.elements[i] == ExpressionElement::CloseParenthesis {
        break;
      }
    }

    self.reduce_adds_till_next_parenthesis(start);
    self.reduce_multiplies_till_next_parenthesis(start);
    if self.elements[start] == ExpressionElement::OpenParenthesis {
      self.elements.remove(start + 2);
      self.elements.remove(start);
    }
  }

  fn reduce_adds_till_next_parenthesis(&mut self, start: usize) {
    let mut i = start;
    while &(i + 2) < &self.elements.len() && self.elements[i + 1] != ExpressionElement::CloseParenthesis &&
                                             self.elements[i + 2] != ExpressionElement::CloseParenthesis {
      match (&self.elements[i], &self.elements[i + 1], &self.elements[i + 2]) {
        (ExpressionElement::Number(a), ExpressionElement::Add, ExpressionElement::Number(b)) => {
          let new_number = ExpressionElement::Number(a + b);
          self.elements.insert(i, new_number);
          self.elements.remove(i + 3);
          self.elements.remove(i + 2);
          self.elements.remove(i + 1);
        },
        _ => {
          i += 1;
        }
      }
    }
  }

  fn reduce_multiplies_till_next_parenthesis(&mut self, start: usize) {
    let mut i = start;
    while &(i + 2) < &self.elements.len() && self.elements[i + 1] != ExpressionElement::CloseParenthesis &&
                                             self.elements[i + 2] != ExpressionElement::CloseParenthesis {
      match (&self.elements[i], &self.elements[i + 1], &self.elements[i + 2]) {
        (ExpressionElement::Number(a), ExpressionElement::Multiply, ExpressionElement::Number(b)) => {
          let new_number = ExpressionElement::Number(a * b);
          self.elements.insert(i, new_number);
          self.elements.remove(i + 3);
          self.elements.remove(i + 2);
          self.elements.remove(i + 1);
        },
        _ => {
          i += 1;
        }
      }
    }
  }
}

#[derive(Debug, PartialEq)]
enum ExpressionElement {
  Number(i64),
  Add,
  Multiply,
  OpenParenthesis,
  CloseParenthesis,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_expression() {
    let expression_1 = Expression::parse("1 + 2 * 3 + 4 * 5 + 6");
    assert_eq!(expression_1.elements, vec![ExpressionElement::Number(1), ExpressionElement::Add, ExpressionElement::Number(2),
      ExpressionElement::Multiply, ExpressionElement::Number(3), ExpressionElement::Add, ExpressionElement::Number(4),
      ExpressionElement::Multiply, ExpressionElement::Number(5), ExpressionElement::Add, ExpressionElement::Number(6)]);
    let expression_2 = Expression::parse("12 + 168");
    assert_eq!(expression_2.elements, vec![ExpressionElement::Number(12), ExpressionElement::Add, ExpressionElement::Number(168)]);
    let expression_3 = Expression::parse("12 + (2 * 3) + (4 * (52 + 63))");
    assert_eq!(expression_3.elements, vec![ExpressionElement::Number(12), ExpressionElement::Add, ExpressionElement::OpenParenthesis,
      ExpressionElement::Number(2), ExpressionElement::Multiply, ExpressionElement::Number(3), ExpressionElement::CloseParenthesis,
      ExpressionElement::Add, ExpressionElement::OpenParenthesis, ExpressionElement::Number(4), ExpressionElement::Multiply,
      ExpressionElement::OpenParenthesis, ExpressionElement::Number(52), ExpressionElement::Add, ExpressionElement::Number(63),
      ExpressionElement::CloseParenthesis, ExpressionElement::CloseParenthesis]);
  }

  #[test]
  fn test_reduce() {
    let mut expression_1 = Expression::parse("1 + 2 * 3 + 4 * 5 + 6");
    assert_eq!(expression_1.reduce(), 71);
    let mut expression_2 = Expression::parse("1 + (2 * 3) + (4 * (5 + 6))");
    assert_eq!(expression_2.reduce(), 51);
    let mut expression_2 = Expression::parse("2 * 3 + (4 * 5)");
    assert_eq!(expression_2.reduce(), 26);
    let mut expression_2 = Expression::parse("5 + (8 * 3 + 9 + 3 * 4 * 3)");
    assert_eq!(expression_2.reduce(), 437);
    let mut expression_2 = Expression::parse("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");
    assert_eq!(expression_2.reduce(), 12240);
    let mut expression_2 = Expression::parse("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");
    assert_eq!(expression_2.reduce(), 13632);
  }

  fn sample_input() -> Vec<String> {
    vec![
      String::from("1 + 2 * 3 + 4 * 5 + 6"),
      String::from("1 + (2 * 3) + (4 * (5 + 6))"),
      String::from("2 * 3 + (4 * 5)"),
      String::from("5 + (8 * 3 + 9 + 3 * 4 * 3)"),
      String::from("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
      String::from("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
    ]
  }

  #[test]
  fn test_puzzle_1() {
    assert_eq!(Day18{}.puzzle_1(&sample_input()), "26457");
  }

  #[test]
  fn test_reduce_with_precedence() {
    // let mut expression_1 = Expression::parse("1 + 2 * 3 + 4 * 5 + 6");
    // assert_eq!(expression_1.reduce_with_precedence(), 231);
    // let mut expression_2 = Expression::parse("1 + (2 * 3) + (4 * (5 + 6))");
    // assert_eq!(expression_2.reduce_with_precedence(), 51);
    // let mut expression_2 = Expression::parse("2 * 3 + (4 * 5)");
    // assert_eq!(expression_2.reduce_with_precedence(), 46);
    // let mut expression_2 = Expression::parse("5 + (8 * 3 + 9 + 3 * 4 * 3)");
    // assert_eq!(expression_2.reduce_with_precedence(), 1445);
    // let mut expression_2 = Expression::parse("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");
    // assert_eq!(expression_2.reduce_with_precedence(), 669060);
    let mut expression_2 = Expression::parse("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");
    assert_eq!(expression_2.reduce_with_precedence(), 23340);
  }
}
