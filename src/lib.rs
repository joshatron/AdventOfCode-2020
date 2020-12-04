pub mod input;
pub mod days;

pub fn run() {
  println!("Advent of Code 2020 Results");
  println!("===========================");

  for day in days::get_days() {
    let day_input = input::get_day_input(day.day_num());
    println!("\nDay {}", day.day_num());
    println!("  Puzzle 1: {}", day.puzzle_1(&day_input));
    println!("  Puzzle 2: {}", day.puzzle_2(&day_input));
  }
}