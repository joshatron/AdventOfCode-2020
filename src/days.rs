pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;

pub trait Day {
  fn day_num(&self) -> usize;
  fn puzzle_1(&self, input: &Vec<String>) -> String;
  fn puzzle_2(&self, input: &Vec<String>) -> String;
}

pub fn input_to_ints(input: &Vec<String>) -> Vec<i64> {
  input.iter()
    .map(|l| l.parse::<i64>().unwrap())
    .collect()
}