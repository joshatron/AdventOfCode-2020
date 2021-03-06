use crate::days::Day;
use std::iter::Peekable;
use std::str::Split;
use std::collections::HashMap;

pub struct Day7 {}

impl Day7 {
  pub fn new() -> Day7 {
    Day7{}
  }
}

impl Day for Day7 {
  fn day_num(&self) -> usize {
    7
  }

  fn puzzle_1(&self, input: &Vec<String>) -> String {
    Bags::parse(input).get_ancestors_of_color("shiny gold").len().to_string()
  }

  fn puzzle_2(&self, input: &Vec<String>) -> String {
    let bags = Bags::parse(input);

    let mut current_colors = vec![bags.get_color("shiny gold").unwrap()];
    let mut total_bags = 0;

    while !current_colors.is_empty() {
      let mut new_colors = vec![];
      for color in &current_colors {
        for holding in &color.holds {
          for _i in 0..holding.quantity {
            new_colors.push(bags.get_color(&holding.color).unwrap());
            total_bags = total_bags + 1;
          }
        }
      }

      current_colors = new_colors;
    }

    total_bags.to_string()
  }
}

struct Bags {
  bags: HashMap<String, Bag>,
}

impl Bags {
  fn get_ancestors_of_color(&self, color: &str) -> Vec<&str> {
    let mut ancestors = vec![];

    let mut current_parents = vec![color];
    
    while !current_parents.is_empty() {
      let mut new_parents = vec![];
      for parent in &current_parents {
        new_parents.extend(self.get_parent_colors(parent));
      }

      current_parents = vec![];

      for parent in &new_parents {
        if !ancestors.contains(parent) {
          current_parents.push(parent);
          ancestors.push(parent);
        }
      }
    }

    ancestors
  }

  fn get_parent_colors(&self, color: &str) -> Vec<&str> {
    self.bags.values()
      .filter(|b| can_hold_color(&b, color))
      .map(|b| &*b.color)
      .collect()
  }

  fn get_color(&self, color: &str) -> Option<&Bag> {
    self.bags.get(color)
  }

  fn parse(input: &Vec<String>) -> Bags {
    Bags{
      bags: input.iter()
        .map(|l| Bag::parse(&l))
        .map(|b| (String::from(&b.color), b))
        .collect()
    }
  }
}

fn can_hold_color(bag: &Bag, color: &str) -> bool {
  bag.holds.iter()
    .any(|h| h.color == color)
}

struct Bag {
  color: String,
  holds: Vec<BagHolding>,
}

impl Bag {
  fn parse(input: &str) -> Bag {
    let mut words = input.split(" ").peekable();
    let color = get_color(&mut words);
    words.next();
    words.next();

    match words.peek().unwrap() {
      &"no" => Bag {color: color, holds: vec![]},
      _ => Bag {color: color, holds: get_holds(&mut words)}
    }
  }
}

#[derive(Debug, PartialEq)]
struct BagHolding {
  color: String,
  quantity: usize,
}

fn get_holds(words: &mut Peekable<Split<&str>>) -> Vec<BagHolding> {
  let mut holding = vec![];

  while words.peek() != None {
    holding.push(BagHolding{
      quantity: words.next().unwrap().parse::<usize>().unwrap(),
      color: get_color(words),
    });
    words.next();
  }


  holding
}

fn get_color(words: &mut Peekable<Split<&str>>) -> String {
  let mut color = String::from(words.next().unwrap());
  color.push_str(" ");
  color.push_str(words.next().unwrap());

  color
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_create_bag_not_containing_other() {
    let bag = Bag::parse("faded blue bags contain no other bags.");
    assert_eq!(bag.color, "faded blue");
  }

  #[test]
  fn test_create_bag_containing_one() {
    let bag = Bag::parse("bright white bags contain 1 shiny gold bag.");
    assert_eq!(bag.color, "bright white");
    assert_eq!(bag.holds.len(), 1);
    assert_eq!(bag.holds[0], BagHolding{color: String::from("shiny gold"), quantity: 1});
  }

  #[test]
  fn test_create_bag_containing_multiple() {
    let bag = Bag::parse("muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.");
    assert_eq!(bag.color, "muted yellow");
    assert_eq!(bag.holds.len(), 2);
    assert_eq!(bag.holds[0], BagHolding{color: String::from("shiny gold"), quantity: 2});
    assert_eq!(bag.holds[1], BagHolding{color: String::from("faded blue"), quantity: 9});
  }

  fn sample_input() -> Vec<String> {
    vec![
      String::from("light red bags contain 1 bright white bag, 2 muted yellow bags."),
      String::from("dark orange bags contain 3 bright white bags, 4 muted yellow bags."),
      String::from("bright white bags contain 1 shiny gold bag."),
      String::from("muted yellow bags contain 2 shiny gold bags, 9 faded blue bags."),
      String::from("shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags."),
      String::from("dark olive bags contain 3 faded blue bags, 4 dotted black bags."),
      String::from("vibrant plum bags contain 5 faded blue bags, 6 dotted black bags."),
      String::from("faded blue bags contain no other bags."),
      String::from("dotted black bags contain no other bags."),
    ]
  }

  #[test]
  fn test_bags_parse() {
    let bags = Bags::parse(&sample_input());
    assert_eq!(bags.bags.len(), 9);
    assert_eq!(bags.get_color("light red").unwrap().color, "light red");
    assert_eq!(bags.get_color("dotted black").unwrap().color, "dotted black");
  }

  #[test]
  fn test_get_parent_colors() {
    let bags = Bags::parse(&sample_input());
    assert_eq!(bags.get_parent_colors("shiny gold").len(), 2);
    assert_eq!(bags.get_parent_colors("shiny gold").contains(&"bright white"), true);
    assert_eq!(bags.get_parent_colors("shiny gold").contains(&"muted yellow"), true);
  }

  #[test]
  fn test_get_all_parents_of_color() {
    let bags = Bags::parse(&sample_input());
    let ancestors = bags.get_ancestors_of_color("shiny gold");
    assert_eq!(ancestors.len(), 4);
    assert_eq!(ancestors.contains(&"bright white"), true);
    assert_eq!(ancestors.contains(&"muted yellow"), true);
    assert_eq!(ancestors.contains(&"dark orange"), true);
    assert_eq!(ancestors.contains(&"light red"), true);
  }

  #[test]
  fn test_puzzle_1() {
    assert_eq!(Day7{}.puzzle_1(&sample_input()), "4");
  }

  #[test]
  fn test_get_color() {
    let bags = Bags::parse(&sample_input());
    assert_eq!(bags.get_color("shiny gold").unwrap().color, "shiny gold");
    assert_eq!(bags.get_color("bright white").unwrap().color, "bright white");
    assert_eq!(bags.get_color("light red").unwrap().color, "light red");
  }

  #[test]
  fn test_puzzle_2() {
    assert_eq!(Day7{}.puzzle_2(&sample_input()), "32");
  }
}
