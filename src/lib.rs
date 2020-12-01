mod input;
mod days;

pub fn run() {
  println!("Advent of Code 2020 Results:");

  let day_1_input = input::get_day_input_ints(1);
  println!("Day 1");
  println!("  Puzzle 1 result: {}", days::day_1::puzzle_1(&day_1_input));
  println!("  Puzzle 2 result: {}", days::day_1::puzzle_2(&day_1_input));
}