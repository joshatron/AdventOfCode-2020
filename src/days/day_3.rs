use crate::days::Day;

pub struct Day3 {}

impl Day for Day3 {
  fn day_num(&self) -> usize {
    3
  }

  fn puzzle_1(&self, input: &Vec<String>) -> String {
    let forest = generate_forest(input);

    get_trees_in_path(&forest, 3, 1).to_string()
  }

  fn puzzle_2(&self, input: &Vec<String>) -> String {
    let forest = generate_forest(input);


    (get_trees_in_path(&forest, 1, 1) * get_trees_in_path(&forest, 3, 1) * get_trees_in_path(&forest, 5, 1) *
     get_trees_in_path(&forest, 7, 1) * get_trees_in_path(&forest, 1, 2)).to_string()
  }
}

fn get_trees_in_path(forest: &Forest, x_slope: usize, y_slope: usize) -> usize {
  let mut x = x_slope;
  let mut y = y_slope;
  let mut trees = 0;

  while y < forest.height() {
    if forest.is_tree_at_loc(x, y) {
      trees = trees + 1;
    }
    x = x + x_slope;
    y = y + y_slope;
  }

  trees
}

struct Forest {
  trees: Vec<Vec<bool>>,
}

impl Forest {
  fn is_tree_at_loc(&self, x: usize, y: usize) -> bool {
    self.trees[y][x % self.base_width()]
  }

  fn height(&self) -> usize {
    self.trees.len()
  }

  fn base_width(&self) -> usize {
    self.trees[0].len()
  }
}

fn generate_forest(input: &Vec<String>) -> Forest {
  Forest{
    trees: input_to_bools(input),
  }
}

fn input_to_bools(input: &Vec<String>) -> Vec<Vec<bool>> {
  input.iter()
    .map(|l| string_to_bools(&l))
    .collect()
}

fn string_to_bools(line: &String) -> Vec<bool> {
  line.chars()
    .map(|c| match c {
      '#' => true,
      _ => false
    })
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  fn sample_input() -> Vec<String> {
    vec![
      String::from("..##......."),
      String::from("#...#...#.."),
      String::from(".#....#..#."),
      String::from("..#.#...#.#"),
      String::from(".#...##..#."),
      String::from("..#.##....."),
      String::from(".#.#.#....#"),
      String::from(".#........#"),
      String::from("#.##...#..."),
      String::from("#...##....#"),
      String::from(".#..#...#.#"),
    ]
  }

  #[test]
  fn test_generate_forest() {
    let forest = generate_forest(&sample_input());
    assert_eq!(forest.trees[0][1], false);
    assert_eq!(forest.trees[0][2], true);
    assert_eq!(forest.trees[10][8], true);
    assert_eq!(forest.trees[10][9], false);
  }

  #[test]
  fn test_is_tree_at_loc() {
    let forest = generate_forest(&sample_input());
    assert_eq!(forest.is_tree_at_loc(1, 0), false);
    assert_eq!(forest.is_tree_at_loc(2, 0), true);
    assert_eq!(forest.is_tree_at_loc(8, 10), true);
    assert_eq!(forest.is_tree_at_loc(9, 10), false);
    assert_eq!(forest.is_tree_at_loc(12, 0), false);
    assert_eq!(forest.is_tree_at_loc(13, 0), true);
  }

  #[test]
  fn test_puzzle_1_sample_input() {
    assert_eq!(Day3{}.puzzle_1(&sample_input()), String::from("7"));
  }

  #[test]
  fn test_puzzle_2_sample_input() {
    assert_eq!(Day3{}.puzzle_2(&sample_input()), String::from("336"));
  }
}
