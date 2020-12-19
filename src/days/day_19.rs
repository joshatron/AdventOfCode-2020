use crate::days::Day;
use std::collections::HashSet;
use std::collections::HashMap;

pub struct Day19 {}

impl Day19 {
  pub fn new() -> Day19 {
    Day19{}
  }
}

impl Day for Day19 {
  fn day_num(&self) -> usize {
    1
  }

  fn puzzle_1(&self, input: &Vec<String>) -> String {
    String::from("")
  }

  fn puzzle_2(&self, input: &Vec<String>) -> String {
    String::from("")
  }
}

struct Rules {
  rules: HashMap<usize, RuleType>,
}

impl Rules {
  fn parse(input: &Vec<String>) -> Rules {
    let mut rules = Rules {
      rules: HashMap::new(),
    };

    for line in input {
      if line.is_empty() {
        break;
      }

      rules.rules.insert(parse_rule_num(line), parse_rule(line));
    }

    rules
  }

  fn get_rule(&self, index: usize) -> &RuleType {
    &self.rules.get(&index).unwrap()
  }

  fn reduce(&mut self) {
    let mut done = false;
    while !done {
      done = true;
      for (rule_num, rule) in &self.rules {
        match rule {
          RuleType::Order(rules) => println!("Order"),
          RuleType::Other(rules) => println!("Other"),
          RuleType::Or(first, second) => println!("Or"),
          RuleType::Set(rules) => println!("Or"),
          RuleType::None => println!("Nothing to do"),
        }
      }
    }
  }

  fn replace_set(&mut self, set: RuleType, number: usize) {
    let mut new_rules = HashMap::new();

    for (rule_num, rule) in &self.rules {
      match rule {
        RuleType::Order(rules) => println!("Order"),
        RuleType::Other(rules) => {
          if rules == &number {
            new_rules.insert(*rule_num, set.clone());
          } else {
            new_rules.insert(*rule_num, *rule);
          }
        },
        RuleType::Or(first, second) => println!("Or"),
        RuleType::Set(rules) => println!("Or"),
        RuleType::None => println!("Nothing to do"),
      }
    }

    self.rules = new_rules;
  }
}

#[derive(Debug, PartialEq, Clone)]
enum RuleType {
  Order(Vec<RuleType>),
  Other(usize),
  Or(Box<RuleType>, Box<RuleType>),
  Set(HashSet<String>),
  None,
}

fn parse_rule_num(line: &str) -> usize {
  line.split(":")
    .next()
    .unwrap()
    .parse::<usize>()
    .unwrap()
}

fn parse_rule(line: &str) -> RuleType {
  let mut parts = line.split(": ");
  parts.next();
  let rules = parts.next().unwrap();

  if rules.contains("|") {
    let mut or_parts = rules.split(" | ");

    RuleType::Or(Box::new(parse_order(or_parts.next().unwrap())), Box::new(parse_order(or_parts.next().unwrap())))
  } else if rules.contains("\"") {
    let mut set = HashSet::new();
    set.insert(rules[1..(rules.len() - 1)].to_string());
    
    RuleType::Set(set)
  } else {
    parse_order(rules)
  }
}

fn parse_order(rules: &str) -> RuleType {
  let mut others = Vec::new();
  for other in rules.split(" ") {
    others.push(RuleType::Other(other.parse::<usize>().unwrap()));
  }

  RuleType::Order(others)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn sample_input() -> Vec<String> {
    vec![
      String::from("0: 4 1 5"),
      String::from("1: 2 3 | 3 2"),
      String::from("2: 4 4 | 5 5"),
      String::from("3: 4 5 | 5 4"),
      String::from("4: \"a\""),
      String::from("5: \"b\""),
      String::from(""),
      String::from("ababbb"),
      String::from("bababa"),
      String::from("abbbab"),
      String::from("aaabbb"),
      String::from("aaaabbb"),
    ]
  }

  #[test]
  fn test_parse_rules() {
    let rules = Rules::parse(&sample_input());
    assert_eq!(rules.get_rule(0), &RuleType::Order(vec![RuleType::Other(4), RuleType::Other(1), RuleType::Other(5)]));
    assert_eq!(rules.get_rule(1), &RuleType::Or(Box::new(RuleType::Order(vec![RuleType::Other(2), RuleType::Other(3)])),
                                                Box::new(RuleType::Order(vec![RuleType::Other(3), RuleType::Other(2)]))));
    assert_eq!(rules.get_rule(2), &RuleType::Or(Box::new(RuleType::Order(vec![RuleType::Other(4), RuleType::Other(4)])),
                                                Box::new(RuleType::Order(vec![RuleType::Other(5), RuleType::Other(5)]))));
    assert_eq!(rules.get_rule(3), &RuleType::Or(Box::new(RuleType::Order(vec![RuleType::Other(4), RuleType::Other(5)])),
                                                Box::new(RuleType::Order(vec![RuleType::Other(5), RuleType::Other(4)]))));
    let mut first_set = HashSet::new();
    first_set.insert(String::from("a"));
    assert_eq!(rules.get_rule(4), &RuleType::Set(first_set));
    let mut second_set = HashSet::new();
    second_set.insert(String::from("b"));
    assert_eq!(rules.get_rule(5), &RuleType::Set(second_set));
  }

  #[test]
  fn test_reduce_rules() {
    let mut rules = Rules::parse(&sample_input());
    rules.reduce();

    let mut first_set = HashSet::new();
    first_set.insert(String::from("b"));
    assert_eq!(rules.get_rule(5), &RuleType::Set(first_set));
    let mut second_set = HashSet::new();
    second_set.insert(String::from("a"));
    assert_eq!(rules.get_rule(4), &RuleType::Set(second_set));
    let mut third_set = HashSet::new();
    third_set.insert(String::from("ab"));
    third_set.insert(String::from("ba"));
    assert_eq!(rules.get_rule(3), &RuleType::Set(third_set));
  }

  #[test]
  fn test_puzzle_1() {
    assert_eq!(Day19::new().puzzle_1(&sample_input()), "");
  }

  #[test]
  fn test_puzzle_2() {
    assert_eq!(Day19::new().puzzle_2(&sample_input()), "");
  }
}
