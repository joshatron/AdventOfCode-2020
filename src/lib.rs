pub mod input;
pub mod days;

use days::Day;

pub fn run() {
  println!("Advent of Code 2020 Results");
  println!("===========================\n");

  let day_1_input = input::get_day_input(1);
  println!("Day 1");
  println!("-----");
  println!("  Puzzle 1: {}", days::day_1::Day1{}.puzzle_1(&day_1_input));
  println!("  Puzzle 2: {}", days::day_1::Day1{}.puzzle_2(&day_1_input));
  println!("");
}