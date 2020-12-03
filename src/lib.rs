pub mod input;
pub mod days;

use days::Day;

pub fn run() {
  println!("Advent of Code 2020 Results");
  println!("===========================");

  let mut all_days: Vec<Box<dyn Day>> = Vec::new();
  all_days.push(Box::new(days::day_1::Day1{}));
  all_days.push(Box::new(days::day_2::Day2{}));
  all_days.push(Box::new(days::day_3::Day3{}));

  for day in all_days {
    let day_input = input::get_day_input(day.day_num());
    println!("\nDay {}", day.day_num());
    println!("  Puzzle 1: {}", day.puzzle_1(&day_input));
    println!("  Puzzle 2: {}", day.puzzle_2(&day_input));
  }
}