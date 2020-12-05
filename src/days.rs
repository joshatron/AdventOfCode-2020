pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;

pub trait Day {
  fn day_num(&self) -> usize;
  fn puzzle_1(&self, input: &Vec<String>) -> String;
  fn puzzle_2(&self, input: &Vec<String>) -> String;
}

pub fn get_days() -> Vec<Box<dyn Day>> {
  vec![
    Box::new(day_1::Day1{}),
    Box::new(day_2::Day2{}),
    Box::new(day_3::Day3{}),
    Box::new(day_4::Day4{}),
    Box::new(day_5::Day5{}),
  ]
}

pub fn input_to_ints(input: &Vec<String>) -> Vec<i64> {
  input.iter()
    .map(|l| l.parse::<i64>().unwrap())
    .collect()
}