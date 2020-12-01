pub fn puzzle_1(input: &Vec<i64>) -> i64 {
  for (i, &num) in input.iter().enumerate() {
    let rest = &input[(i + 1)..];

    for other in rest {
      if num + other == 2020 {
        return num * other;
      }
    }
  }

  0
}

pub fn puzzle_2(input: &Vec<i64>) -> i64 {
  for (i, &first) in input.iter().enumerate() {
    let second_set = &input[(i + 1)..];

    for (j, &second) in second_set.iter().enumerate() {
      let third_set = &second_set[(j + 1)..];

      for third in third_set {
        if first + second + third == 2020 {
          return first * second * third;
        }
      }
    } 
  }

  0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn puzzle_1_sample_input() {
    let input = vec![1721, 979, 366, 299, 675, 1456];

    assert_eq!(puzzle_1(&input), 514579);
  }

  #[test]
  fn puzzle_2_sample_input() {
    let input = vec![1721, 979, 366, 299, 675, 1456];

    assert_eq!(puzzle_2(&input), 241861950);
  }
}