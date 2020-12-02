use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn get_day_input(day: usize) -> Vec<String> {
  let file_name = String::from("resources/day_") + &day.to_string() + ".txt";

  let file = File::open(file_name).expect("Day input doesn't exist");
  let buf = BufReader::new(file);

  buf.lines()
     .map(|l| l.expect("Could not parse line."))
     .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn day_1_input() {
    let lines = get_day_input(1);

    assert_eq!(lines.len(), 200);

    for l in &lines {
      l.parse::<i64>().unwrap();
    }
  }
}