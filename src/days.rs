pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;
pub mod day_8;
pub mod day_9;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;
pub mod day_16;
pub mod day_17;

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
    Box::new(day_6::Day6{}),
    Box::new(day_7::Day7{}),
    Box::new(day_8::Day8{}),
    Box::new(day_9::Day9{}),
    Box::new(day_10::Day10{}),
    Box::new(day_11::Day11{}),
    Box::new(day_12::Day12{}),
    Box::new(day_13::Day13{}),
    Box::new(day_14::Day14{}),
    Box::new(day_15::Day15{}),
    Box::new(day_16::Day16{}),
    Box::new(day_17::Day17{}),
  ]
}

pub fn input_to_ints(input: &Vec<String>) -> Vec<i64> {
  input.iter()
    .map(|l| l.parse::<i64>().unwrap())
    .collect()
}