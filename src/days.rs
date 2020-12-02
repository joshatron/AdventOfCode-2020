pub mod day_1;

pub trait Day {
  fn puzzle_1(&self, input: &Vec<String>) -> String;
  fn puzzle_2(&self, input: &Vec<String>) -> String;
}

pub fn input_to_ints(input: &Vec<String>) -> Vec<i64> {
  input.iter()
    .map(|l| l.parse::<i64>().unwrap())
    .collect()
}