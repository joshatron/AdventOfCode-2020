use std::time::Instant;

pub mod input;
pub mod days;

pub fn run() {
  println!("Advent of Code 2020 Results");
  println!("===========================");

  for day in days::get_days() {
    let day_input = input::get_day_input(day.day_num());
    println!("\nDay {}", day.day_num());
    let puzzle_1_start = Instant::now();
    let puzzle_1 = day.puzzle_1(&day_input);
    let puzzle_1_time = puzzle_1_start.elapsed().as_micros() as f64 / 1000.;
    println!("  Puzzle 1 ({}ms): {}", puzzle_1_time, puzzle_1);
    let puzzle_2_start = Instant::now();
    let puzzle_2 = day.puzzle_2(&day_input);
    let puzzle_2_time = puzzle_2_start.elapsed().as_micros() as f64 / 1000.;
    println!("  Puzzle 2 ({}ms): {}", puzzle_2_time, puzzle_2);
  }
}