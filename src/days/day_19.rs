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
    19
  }

  fn puzzle_1(&self, input: &Vec<String>) -> String {
    let mut rules = Rules::parse(input);
    rules.reduce();

    let mut matching = 0;
    let strs = rules.get_set(0);
    for line in second_part(input) {
      if strs.contains(&line) {
        matching += 1;
      }
    }

    matching.to_string()
  }

  fn puzzle_2(&self, input: &Vec<String>) -> String {
    let mut rules = Rules::parse(input);
    rules.reduce();

    let fourty_two = rules.get_set(42);
    let thirty_one = rules.get_set(31);

    let mut matching = 0;
    for line in &mut second_part(input) {
      if matches_loop(line, fourty_two, thirty_one) {
        matching += 1;
      }
    }

    matching.to_string()
  }
}

fn matches_loop(line: &mut String, first: &HashSet<String>, second: &HashSet<String>) -> bool {
  let chunks = line.chars()
    .collect::<Vec<char>>()
    .chunks(8)
    .map(|c| c.iter().collect::<String>())
    .collect::<Vec<String>>();

  let mut first_part = true;
  let mut first_parts = 0;
  let mut second_parts = 0;

  for chunk in chunks {
    if first_part {
      if first.contains(&chunk) {
        first_parts += 1;
      } else {
        first_part = false;
        if second.contains(&chunk) {
          second_parts += 1;
        } else {
          return false;
        }
      }
    } else {
      if second.contains(&chunk) {
        second_parts += 1;
      } else {
        return false;
      }
    }
  }

  first_parts > second_parts && second_parts != 0
}

fn second_part(input: &Vec<String>) -> Vec<String> {
  let mut to_return = Vec::new();
  let mut started = false;
  for line in input {
    if started {
      to_return.push(line.clone());
    } else if line.is_empty() {
      started = true;
    }
  }

  to_return
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

  fn get_set(&self, index: usize) -> &HashSet<String> {
    if let RuleType::Set(strs) = self.rules.get(&index).unwrap() {
      &strs
    } else {
      panic!("Not a set!");
    }
  }

  fn reduce(&mut self) {
    let mut done = false;
    while !done {
      let mut sets = HashMap::new();

      for (rule_num, rule) in &self.rules {
        match rule {
          RuleType::Set(strs) => {
            sets.insert(rule_num.clone(), strs.clone());
          },
          _ => (),
        }
      }

      for (set_num, set) in sets {
        let mut new_rules = HashMap::new();
        for (rule_num, rule) in &self.rules {
          new_rules.insert(*rule_num, sub_set(rule.clone(), &set, set_num));
        }

        self.rules = new_rules;
      }

      let mut new_rules = HashMap::new();
      for (rule_num, rule) in &self.rules {
        new_rules.insert(*rule_num, simplfy(rule.clone()));
      }
      self.rules = new_rules;

      if let RuleType::Set(_) = self.get_rule(0) {
        done = true;
      }
    }
  }
}

fn simplfy(rule: RuleType) -> RuleType {
  match rule {
    RuleType::Order(rules) => {
      let mut all_sets = true;
      for r in &rules {
        if let RuleType::Set(_) = r {
        } else {
          all_sets = false;
        }
      }

      if all_sets {
        squash_order(&rules)
      } else {
        RuleType::Order(rules)
      }
    },
    RuleType::Other(_) => rule,
    RuleType::Or(first, second) => {
      let both = vec![*first.clone(), *second.clone()];

      let mut all_sets = true;
      for r in &both {
        if let RuleType::Set(_) = r {

        } else {
          all_sets = false;
        }
      }

      if all_sets {
        squash_or(&both)
      } else {
        RuleType::Or(Box::new(simplfy(*first)), Box::new(simplfy(*second)))
      }

    },
    RuleType::Set(strs) => RuleType::Set(strs),
  }
}

fn squash_or(sets: &Vec<RuleType>) -> RuleType {
  let mut new_set = HashSet::new();

  for set in sets {
    if let RuleType::Set(strs) = set {
      for part in strs {
        new_set.insert(part.clone());
      }
    }
  }

  RuleType::Set(new_set)
}

fn squash_order(sets: &Vec<RuleType>) -> RuleType {
  let mut new_set = HashSet::new();

  for set in sets {
    if let RuleType::Set(strs) = set {
      if new_set.is_empty() {
        new_set = strs.clone();
      } else {
        let mut temp_set = HashSet::new();
        for s1 in new_set {
          for s2 in strs {
            let mut both = s1.clone();
            both.push_str(s2);
            temp_set.insert(both);
          }
        }

        new_set = temp_set;
      }
    }
  }


  RuleType::Set(new_set)
}

fn sub_set(rule: RuleType, set: &HashSet<String>, set_number: usize) -> RuleType {
  match rule {
    RuleType::Order(rules) => {
      let mut new_rules = Vec::new();
      for rule in rules {
        new_rules.push(sub_set(rule, set, set_number))
      }

      RuleType::Order(new_rules)
    },
    RuleType::Other(num) => {
      if num == set_number {
        RuleType::Set(set.clone())
      } else {
        RuleType::Other(num)
      }
    },
    RuleType::Or(first, second) => {
      RuleType::Or(Box::new(sub_set(*first, set, set_number)), Box::new(sub_set(*second, set, set_number)))
    },
    RuleType::Set(strs) => RuleType::Set(strs),
  }
}

#[derive(Debug, PartialEq, Clone)]
enum RuleType {
  Order(Vec<RuleType>),
  Other(usize),
  Or(Box<RuleType>, Box<RuleType>),
  Set(HashSet<String>),
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
    let mut fourth_set = HashSet::new();
    fourth_set.insert(String::from("aa"));
    fourth_set.insert(String::from("bb"));
    assert_eq!(rules.get_rule(2), &RuleType::Set(fourth_set));
    let mut fifth_set = HashSet::new();
    fifth_set.insert(String::from("aaab"));
    fifth_set.insert(String::from("aaba"));
    fifth_set.insert(String::from("bbab"));
    fifth_set.insert(String::from("bbba"));
    fifth_set.insert(String::from("abaa"));
    fifth_set.insert(String::from("abbb"));
    fifth_set.insert(String::from("baaa"));
    fifth_set.insert(String::from("babb"));
    assert_eq!(rules.get_rule(1), &RuleType::Set(fifth_set));
  }

  #[test]
  fn test_puzzle_1() {
    assert_eq!(Day19::new().puzzle_1(&sample_input()), "2");
  }

  #[test]
  fn test_puzzle_2() {
    assert_eq!(Day19::new().puzzle_2(&sample_input()), "");
  }
}
