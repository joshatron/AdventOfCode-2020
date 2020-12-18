use crate::days::Day;

pub struct Day2 {}

impl Day for Day2 {
  fn day_num(&self) -> usize {
    2
  }

  fn puzzle_1(&self, input: &Vec<String>) -> String {
    let mut valid = 0;

    for l in input {
      let line = Line::parse(l);
      let occurences = get_number_of_occurences(&line);
      if occurences >= line.lower && occurences <= line.upper {
        valid = valid + 1;
      }
    }

    valid.to_string()
  }

  fn puzzle_2(&self, input: &Vec<String>) -> String {
    let mut valid = 0;

    for l in input {
      let line = Line::parse(l);
        if is_matching_only_once(&line) {
            valid = valid + 1;
        }
    }

    valid.to_string()
  }
}

struct Line {
  lower: usize,
  upper: usize,
  c: char,
  password: String,
}

impl Line {
  fn parse(input: &String) -> Line {
    Line {
      lower: get_lower_range(input),
      upper: get_upper_range(input),
      c: get_char_to_check(input),
      password: get_password(input),
    }
  }
}

fn get_lower_range(line: &String) -> usize {
    let mut split = line.split("-");

    split.next().unwrap().parse::<usize>().unwrap()
}

fn get_upper_range(line: &String) -> usize {
    let mut split = line.split("-");
    split.next();

    let mut second_split = split.next().unwrap().split(" ");

    second_split.next().unwrap().parse::<usize>().unwrap()
}

fn get_char_to_check(line: &String) -> char {
    let mut split = line.split(" ");
    split.next();

    split.next().unwrap().chars().next().unwrap()
}

fn get_password<'a>(line: &String) -> String {
    let mut split = line.split(" ");
    split.next();
    split.next();

    String::from(split.next().unwrap())
}

fn get_number_of_occurences(line: &Line) -> usize {
    let mut occurences = 0;

    for c in line.password.chars() {
        if line.c == c {
            occurences = occurences + 1;
        }
    }

    occurences
}

fn is_matching_only_once(line: &Line) -> bool {
    let mut chars = line.password.chars();

    let first_char = chars.nth(line.lower - 1).unwrap();
    let second_char = chars.nth(line.upper - 1 - line.lower).unwrap();

    if first_char == line.c && second_char != line.c {
        return true
    }

    if first_char != line.c && second_char == line.c {
        return true
    }

    false
}



#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_lower_range() {
      let input = String::from("10-135 c: cccbbbaaac");

        assert_eq!(get_lower_range(&input), 10);
  }

  #[test]
  fn test_get_upper_range() {
      let input = String::from("10-135 c: cccbbbaaac");

        assert_eq!(get_upper_range(&input), 135);
  }

  #[test]
  fn test_get_char_to_check() {
      let input = String::from("10-135 c: cccbbbaaac");

        assert_eq!(get_char_to_check(&input), 'c');
  }

  #[test]
  fn test_get_password() {
      let input = String::from("10-135 c: cccbbbaaac");

      assert_eq!(get_password(&input), "cccbbbaaac");
  }

  #[test]
  fn test_get_number_of_occurences() {
    let line = Line::parse(&String::from("1-2 c: cccbbbaaac"));
      assert_eq!(get_number_of_occurences(&line), 4);
  }

  #[test]
  fn test_is_matching_only_once() {
    let line1 = Line::parse(&String::from("1-3 a: abcde"));
    assert_eq!(is_matching_only_once(&line1), true);
    let line2 = Line::parse(&String::from("1-3 b: cdefg"));
    assert_eq!(is_matching_only_once(&line2), false);
    let line3 = Line::parse(&String::from("2-9 c: ccccccccc"));
    assert_eq!(is_matching_only_once(&line3), false);
  }

  #[test]
  fn puzzle_1_sample_input() {
    let input = vec![String::from("1-3 a: abcde"), String::from("1-3 b: cdefg"), String::from("2-9 c: ccccccccc")];

    assert_eq!(Day2{}.puzzle_1(&input), String::from("2"));
  }

  #[test]
  fn puzzle_2_sample_input() {
    let input = vec![String::from("1-3 a: abcde"), String::from("1-3 b: cdefg"), String::from("2-9 c: ccccccccc")];

    assert_eq!(Day2{}.puzzle_2(&input), String::from("1"));
  }
}