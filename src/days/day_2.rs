use crate::days::Day;

pub struct Day2 {}

impl Day for Day2 {
  fn puzzle_1(&self, input: &Vec<String>) -> String {
    let mut valid = 0;

    for line in input {
        if valid_password(line) {
            valid = valid + 1;
        }
    }

    valid.to_string()
  }

  fn puzzle_2(&self, input: &Vec<String>) -> String {
    let mut valid = 0;

    for line in input {
        if is_matching_only_once(get_lower_range(line), get_upper_range(line), 
                                 get_char_to_check(line), get_password(line)) {
            valid = valid + 1;
        }
    }

    valid.to_string()
  }
}

fn valid_password(line: &String) -> bool {
    let occurences = get_number_of_occurences(get_char_to_check(line), &get_password(line));

    occurences >= get_lower_range(line) && occurences <= get_upper_range(line)
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

fn get_password(line: &String) -> &str {
    let mut split = line.split(" ");
    split.next();
    split.next();

    split.next().unwrap()
}

fn get_number_of_occurences(c: char, string: &str) -> usize {
    let mut occurences = 0;

    for c2 in string.chars() {
        if c == c2 {
            occurences = occurences + 1;
        }
    }

    occurences
}

fn is_matching_only_once(first: usize, second: usize, c: char, string: &str) -> bool {
    let mut chars = string.chars();

    let first_char = chars.nth(first - 1).unwrap();
    let second_char = chars.nth(second - 1 - first).unwrap();

    if first_char == c && second_char != c {
        return true
    }

    if first_char != c && second_char == c {
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
      assert_eq!(get_number_of_occurences('c', "cccbbbaaac"), 4);
  }

  #[test]
  fn test_is_matching_only_once() {
    assert_eq!(is_matching_only_once(1, 3, 'a', "abcde"), true);
    assert_eq!(is_matching_only_once(1, 3, 'b', "cdefg"), false);
    assert_eq!(is_matching_only_once(2, 9, 'c', "ccccccccc"), false);
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