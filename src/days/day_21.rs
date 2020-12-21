use crate::days::Day;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct Day21 {}

impl Day21 {
  pub fn new() -> Day21 {
    Day21 {}
  }
}

impl Day for Day21 {
  fn day_num(&self) -> usize {
    21
  }

  fn puzzle_1(&self, input: &Vec<String>) -> String {
    let food_items = AllFood::parse(input);

    food_items
      .safe_ingredients()
      .iter()
      .map(|i| food_items.count_occurances(i))
      .sum::<usize>()
      .to_string()
  }

  fn puzzle_2(&self, input: &Vec<String>) -> String {
    let food_items = AllFood::parse(input);
    let allergens = food_items.identify_allergens();
    let mut allergen_keys = allergens.keys().map(|k| *k).collect::<Vec<&String>>();
    allergen_keys.sort();

    let mut dangerous_ingredients = String::new();
    for allergen in &allergen_keys {
      dangerous_ingredients.push_str(allergens.get(allergen).unwrap());
      if allergen != &allergen_keys[allergen_keys.len() - 1] {
        dangerous_ingredients.push_str(",");
      }
    }

    dangerous_ingredients
  }
}

struct AllFood {
  items: Vec<Food>,
}

impl AllFood {
  fn parse(input: &Vec<String>) -> AllFood {
    AllFood {
      items: input.iter().map(|l| Food::parse(l)).collect(),
    }
  }

  fn all_allergens(&self) -> HashSet<&String> {
    let mut all_allergens = HashSet::new();

    for food in &self.items {
      for allergen in &food.allergens {
        all_allergens.insert(allergen);
      }
    }

    all_allergens
  }

  fn all_ingredients(&self) -> HashSet<&String> {
    let mut all_ingredients = HashSet::new();

    for food in &self.items {
      for ingredient in &food.ingredients {
        all_ingredients.insert(ingredient);
      }
    }

    all_ingredients
  }

  fn with_allergen(&self, allergen: &str) -> Vec<&Food> {
    let mut foods = Vec::new();

    for food in &self.items {
      if food.allergens.contains(allergen) {
        foods.push(food);
      }
    }

    foods
  }

  fn find_ingredients_with_allergen(&self, allergen: &str) -> HashSet<&String> {
    let foods = self.with_allergen(allergen);
    let mut final_ingredients = HashSet::new();

    for ingredient in &foods[0].ingredients {
      final_ingredients.insert(ingredient);
    }

    for food in foods {
      let mut new_final_ingredients = HashSet::new();
      for ingredient in final_ingredients {
        if food.ingredients.contains(ingredient) {
          new_final_ingredients.insert(ingredient);
        }
      }

      final_ingredients = new_final_ingredients;
    }

    final_ingredients
  }

  fn identify_allergens(&self) -> HashMap<&String, &String> {
    let mut allergen_ingredients = HashMap::new();
    let mut allergens = HashMap::new();

    for allergen in self.all_allergens() {
      allergen_ingredients.insert(allergen, self.find_ingredients_with_allergen(allergen));
    }

    while !allergen_ingredients.is_empty() {
      let mut new_allergen_ingredients = HashMap::new();

      for (allergen, ingredients) in allergen_ingredients {
        if ingredients.len() == 1 {
          allergens.insert(allergen, *ingredients.iter().next().unwrap());
        } else {
          let mut new_ingredients = HashSet::new();
          for ingredient in ingredients {
            if !allergens.values().any(|v| v == &ingredient) {
              new_ingredients.insert(ingredient);
            }
          }
          new_allergen_ingredients.insert(allergen, new_ingredients);
        }
      }

      allergen_ingredients = new_allergen_ingredients;
    }

    allergens
  }

  fn safe_ingredients(&self) -> HashSet<&String> {
    let mut safe_ingredients = HashSet::new();
    let allergens: Vec<&String> = self.identify_allergens().values().map(|v| *v).collect();

    for ingredient in self.all_ingredients() {
      if !allergens.contains(&ingredient) {
        safe_ingredients.insert(ingredient);
      }
    }

    safe_ingredients
  }

  fn count_occurances(&self, ingredient: &str) -> usize {
    let mut found = 0;
    for item in &self.items {
      if item.ingredients.contains(ingredient) {
        found += 1;
      }
    }

    found
  }
}

struct Food {
  ingredients: HashSet<String>,
  allergens: HashSet<String>,
}

impl Food {
  fn parse(line: &str) -> Food {
    let mut parts = line.split(" (contains ");

    let ingredient_strings = parts.next().unwrap().split(" ");
    let mut ingredients = HashSet::new();
    for ingredient in ingredient_strings {
      ingredients.insert(ingredient.to_string());
    }

    let allergen_strings = parts.next().unwrap().split(" ");
    let mut allergens = HashSet::new();
    for allergen in allergen_strings {
      allergens.insert(allergen.to_string().replace(",", "").replace(")", ""));
    }

    Food {
      ingredients: ingredients,
      allergens: allergens,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn sample_input() -> Vec<String> {
    vec![
      String::from("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)"),
      String::from("trh fvjkl sbzzf mxmxvkd (contains dairy)"),
      String::from("sqjhc fvjkl (contains soy)"),
      String::from("sqjhc mxmxvkd sbzzf (contains fish)"),
    ]
  }

  #[test]
  fn test_parse_food() {
    let food_items = AllFood::parse(&sample_input());
    assert_eq!(food_items.items[0].ingredients.contains("mxmxvkd"), true);
    assert_eq!(food_items.items[0].ingredients.contains("trh"), false);
    assert_eq!(food_items.items[0].allergens.contains("dairy"), true);
    assert_eq!(food_items.items[3].allergens.contains("fish"), true);
    assert_eq!(food_items.items[3].allergens.contains("soy"), false);
  }

  #[test]
  fn test_all_allergens() {
    let food_items = AllFood::parse(&sample_input());
    let all_alergens = food_items.all_allergens();
    assert_eq!(all_alergens.len(), 3);
    assert_eq!(all_alergens.contains(&String::from("dairy")), true);
    assert_eq!(all_alergens.contains(&String::from("fish")), true);
    assert_eq!(all_alergens.contains(&String::from("soy")), true);
  }

  #[test]
  fn test_with_allergen() {
    let food_items = AllFood::parse(&sample_input());
    let with_allergens = food_items.with_allergen("dairy");
    assert_eq!(with_allergens.len(), 2);
    assert_eq!(with_allergens[0].ingredients.contains("nhms"), true);
    assert_eq!(with_allergens[1].ingredients.contains("trh"), true);
  }

  #[test]
  fn test_all_ingredients() {
    let food_items = AllFood::parse(&sample_input());
    let all_ingredients = food_items.all_ingredients();
    assert_eq!(all_ingredients.len(), 7);
  }

  #[test]
  fn test_safe_ingredients() {
    let food_items = AllFood::parse(&sample_input());
    let safe_ingredients = food_items.safe_ingredients();
    assert_eq!(safe_ingredients.len(), 4);
    assert_eq!(safe_ingredients.contains(&String::from("kfcds")), true);
    assert_eq!(safe_ingredients.contains(&String::from("nhms")), true);
    assert_eq!(safe_ingredients.contains(&String::from("sbzzf")), true);
    assert_eq!(safe_ingredients.contains(&String::from("trh")), true);
  }

  #[test]
  fn test_count_occurances() {
    let food_items = AllFood::parse(&sample_input());
    assert_eq!(food_items.count_occurances("sbzzf"), 2);
    assert_eq!(food_items.count_occurances("trh"), 1);
  }

  #[test]
  fn test_puzzle_1() {
    assert_eq!(Day21::new().puzzle_1(&sample_input()), "5");
  }

  #[test]
  fn test_puzzle_2() {
    assert_eq!(
      Day21::new().puzzle_2(&sample_input()),
      "mxmxvkd,sqjhc,fvjkl"
    );
  }
}
