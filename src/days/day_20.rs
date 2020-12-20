use crate::days::Day;

pub struct Day20 {}

impl Day20 {
  pub fn new() -> Day20 {
    Day20{}
  }
}

impl Day for Day20 {
  fn day_num(&self) -> usize {
    20
  }

  fn puzzle_1(&self, input: &Vec<String>) -> String {
    let image = Image::parse(input);
    let corners = image.find_corners();
    
    (corners[0].id * corners[1].id * corners[2].id * corners[3].id).to_string()
  }

  fn puzzle_2(&self, input: &Vec<String>) -> String {
    let image = Image::parse(input);
    let raw_image = image.get_raw_image();
    let monsters = count_monsters(&raw_image);

    (count_choppy(&raw_image) - monsters * 15).to_string()
  }
}

struct Image {
  fragments: Vec<ImageFragment>,
}

impl Image {
  fn parse(input: &Vec<String>) -> Image {
    let mut image = Image {
      fragments: Vec::new(),
    };

    let mut next_fragment = Vec::new();
    for line in input {
      if line.is_empty() {
        image.fragments.push(ImageFragment::parse(&next_fragment));
        next_fragment = Vec::new();
      } else {
        next_fragment.push(line);
      }
    }
    image.fragments.push(ImageFragment::parse(&next_fragment));

    image
  }

  fn find_corners(&self) -> Vec<&ImageFragment> {
    let mut corners = Vec::new();

    for fragment in &self.fragments {
      let mut paired = 0;
      if self.find_other(fragment.id, &fragment.left_border).is_some() {
        paired += 1;
      }
      if self.find_other(fragment.id, &fragment.right_border).is_some() {
        paired += 1;
      }
      if self.find_other(fragment.id, &fragment.top_border).is_some() {
        paired += 1;
      }
      if self.find_other(fragment.id, &fragment.bottom_border).is_some() {
        paired += 1;
      }

      if paired == 2 {
        corners.push(fragment);
      }
    }

    corners
  }

  fn find_other(&self, current_fragment: usize, border: &str) -> Option<&ImageFragment> {
    let reverse_border: String = border.chars().rev().collect();

    for fragment in &self.fragments {
      if fragment.id != current_fragment &&
         (border == fragment.left_border || border == fragment.right_border ||
         border == fragment.top_border || border == fragment.bottom_border ||
         reverse_border == fragment.left_border || reverse_border == fragment.right_border ||
         reverse_border == fragment.top_border || reverse_border == fragment.bottom_border) {
        return Some(fragment);
      }
    }

    None
  }

  fn get_raw_image(&self) -> Vec<Vec<bool>> {
    let side_length = (self.fragments.len() as f64).sqrt() as usize;
    let corners = self.find_corners();

    let mut fragments: Vec<Vec<ImageFragment>> = Vec::new();
    let mut current_row: Vec<ImageFragment> = Vec::new();

    let mut top_left = corners[0].clone();
    while self.find_other(top_left.id, &top_left.right_border).is_none() {
      top_left = top_left.rotate();
    }
    if self.find_other(top_left.id, &top_left.bottom_border).is_none() {
      top_left = top_left.rotate();
    }
    current_row.push(top_left);
    for _ in 1..side_length {
      let to_match = &current_row[current_row.len() - 1];
      let mut other = self.find_other(to_match.id, &to_match.right_border).unwrap().clone();
      while &other.left_border != &to_match.right_border &&
            &other.left_border.chars().rev().collect::<String>() != &to_match.right_border {
        other = other.rotate();
      }
      if &other.left_border == &to_match.right_border {
        other = other.flip().rotate().rotate();
      }
      current_row.push(other);
    }
    fragments.push(current_row);


    for i in 1..side_length {
      current_row = Vec::new();
      for j in 0..side_length {
        let to_match = &fragments[i - 1][j];
        let mut other = self.find_other(to_match.id, &to_match.bottom_border).unwrap().clone();
        while &other.top_border != &to_match.bottom_border &&
              &other.top_border.chars().rev().collect::<String>() != &to_match.bottom_border {
          other = other.rotate();
        }
        if &other.top_border == &to_match.bottom_border {
          other = other.flip();
        }
        current_row.push(other);
      }

      fragments.push(current_row);
    }

    let mut image: Vec<Vec<bool>> = Vec::new();

    for i in 0..side_length {
      for j in 0..8 {
        let mut row = Vec::new();
        for k in 0..side_length {
          for l in 0..8 {
            row.push(fragments[i][k].image[j][l]);
          }
        }
        image.push(row);
      }
    }

    image
  }
}

#[derive(Clone, Debug)]
struct ImageFragment {
  id: usize,
  left_border: String,
  right_border: String,
  top_border: String,
  bottom_border: String,
  image: Vec<Vec<bool>>,
}

impl ImageFragment {
  fn parse(input: &Vec<&str>) -> ImageFragment {
    let id = input[0][5..(input[0].len() - 1)].parse::<usize>().unwrap();
    let top_border = input[1].to_string();
    let bottom_border = input[10].chars().rev().collect();

    let mut left_border = input[1][0..1].to_string();
    let mut right_border = input[1][9..].to_string();
    let mut image = Vec::new();
    for i in 2..10 {
      left_border.push_str(&input[i][0..1]);
      right_border.push_str(&input[i][9..]);
      image.push(input[i][1..9].to_string()
        .chars()
        .map(|c| c == '#')
        .collect());
    }
    left_border.push_str(&input[10][0..1]);
    right_border.push_str(&input[10][9..]);

    left_border = left_border.chars().rev().collect();

    ImageFragment {
      id: id,
      left_border: left_border,
      right_border: right_border,
      top_border: top_border,
      bottom_border: bottom_border,
      image: image,
    }
  }

  fn rotate(&self) -> ImageFragment {
    ImageFragment {
      id: self.id,
      left_border: self.bottom_border.clone(),
      right_border: self.top_border.clone(),
      top_border: self.left_border.clone(),
      bottom_border: self.right_border.clone(),
      image: rotate_image(&self.image),
    }
  }

  fn flip(&self) -> ImageFragment {
    ImageFragment {
      id: self.id,
      left_border: self.right_border.chars().rev().collect(),
      right_border: self.left_border.chars().rev().collect(),
      top_border: self.top_border.chars().rev().collect(),
      bottom_border: self.bottom_border.chars().rev().collect(),
      image: flip_image(&self.image),
    }
  }
}

fn rotate_image(image: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
  let mut new_image = Vec::new();

  for i in 0..image[0].len() {
    let mut new_line = Vec::new();
    for j in (0..image.len()).rev() {
      new_line.push(image[j][i]);
    }
    new_image.push(new_line);
  }

  new_image
}

fn flip_image(image: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
  let mut new_image = Vec::new();

  for i in 0..image.len() {
    let mut new_line = Vec::new();
    for j in (0..image[0].len()).rev() {
      new_line.push(image[i][j]);
    }
    new_image.push(new_line);
  }

  new_image
}

fn count_monsters(image: &Vec<Vec<bool>>) -> usize {
  let mut pattern = vec![
    vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false],
    vec![true, false, false, false, false, true, true, false, false, false, false, true, true, false, false, false, false, true, true, true],
    vec![false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, false],
  ];

  for _ in 0..4 {
    let matching = get_matching(image, &pattern);
    if matching > 0 {
      return matching;
    }
    pattern = rotate_image(&pattern);
  }
  pattern = flip_image(&pattern);
  for _ in 0..4 {
    let matching = get_matching(image, &pattern);
    if matching > 0 {
      return matching;
    }
    pattern = rotate_image(&pattern);
  }

  0
}

fn get_matching(image: &Vec<Vec<bool>>, pattern: &Vec<Vec<bool>>) -> usize {
  let mut found = 0;
  for x in 0..(image[0].len() - pattern[0].len()) {
    for y in 0..(image.len() - pattern.len()) {
      if does_match(image, pattern, x, y) {
        found += 1;
      }
    }
  }

  found
}

fn does_match(image: &Vec<Vec<bool>>, pattern: &Vec<Vec<bool>>, start_x: usize, start_y: usize) -> bool {
  for x in 0..pattern[0].len() {
    for y in 0..pattern.len() {
      if pattern[y][x] && !image[y + start_y][x + start_x] {
        return false;
      }
    }
  }

  true
}

fn count_choppy(image: &Vec<Vec<bool>>) -> usize {
  let mut total = 0;
  for x in 0..image[0].len() {
    for y in 0..image.len() {
      if image[y][x] {
        total += 1;
      }
    }
  }

  total
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_image_fragments() {
    let image = Image::parse(&sample_input());
    assert_eq!(image.fragments.len(), 9);
    assert_eq!(image.fragments[0].id, 2311);
    assert_eq!(image.fragments[1].top_border, String::from("#.##...##."));
    assert_eq!(image.fragments[3].bottom_border, String::from(".#..#.##.."));
    assert_eq!(image.fragments[5].left_border, String::from(".##...####"));
    assert_eq!(image.fragments[7].right_border, String::from("#..#......"));
    assert_eq!(image.fragments[8].left_border, String::from("...#.##..#"));
    assert_eq!(image.fragments[8].image[3], vec![true, true, true, false, true, false, false, true]);
  }

  #[test]
  fn test_find_corners() {
    let image = Image::parse(&sample_input());
    let corners = image.find_corners();
    assert_eq!(corners.len(), 4);
  }

  #[test]
  fn test_puzzle_1() {
    assert_eq!(Day20::new().puzzle_1(&sample_input()), "20899048083289");
  }

  #[test]
  fn test_rotate() {
    let image = Image::parse(&sample_input());
    let fragment = image.fragments[0].clone().rotate();
    assert_eq!(fragment.image[0], vec![true, false, true, true, true, true, false, true]);
  }

  #[test]
  fn test_construct_raw_image() {
    let image = Image::parse(&sample_input());
    let raw_image = image.get_raw_image();
    assert_eq!(raw_image.len(), 24);
  }

  #[test]
  fn test_count_monsters() {
    let image = Image::parse(&sample_input());
    let raw_image = image.get_raw_image();
    assert_eq!(count_monsters(&raw_image), 2);
  }

  #[test]
  fn test_puzzle_2() {
    assert_eq!(Day20::new().puzzle_2(&sample_input()), "273");
  }

  fn sample_input() -> Vec<String> {
    vec![
      String::from("Tile 2311:"),
      String::from("..##.#..#."),
      String::from("##..#....."),
      String::from("#...##..#."),
      String::from("####.#...#"),
      String::from("##.##.###."),
      String::from("##...#.###"),
      String::from(".#.#.#..##"),
      String::from("..#....#.."),
      String::from("###...#.#."),
      String::from("..###..###"),
      String::from(""),
      String::from("Tile 1951:"),
      String::from("#.##...##."),
      String::from("#.####...#"),
      String::from(".....#..##"),
      String::from("#...######"),
      String::from(".##.#....#"),
      String::from(".###.#####"),
      String::from("###.##.##."),
      String::from(".###....#."),
      String::from("..#.#..#.#"),
      String::from("#...##.#.."),
      String::from(""),
      String::from("Tile 1171:"),
      String::from("####...##."),
      String::from("#..##.#..#"),
      String::from("##.#..#.#."),
      String::from(".###.####."),
      String::from("..###.####"),
      String::from(".##....##."),
      String::from(".#...####."),
      String::from("#.##.####."),
      String::from("####..#..."),
      String::from(".....##..."),
      String::from(""),
      String::from("Tile 1427:"),
      String::from("###.##.#.."),
      String::from(".#..#.##.."),
      String::from(".#.##.#..#"),
      String::from("#.#.#.##.#"),
      String::from("....#...##"),
      String::from("...##..##."),
      String::from("...#.#####"),
      String::from(".#.####.#."),
      String::from("..#..###.#"),
      String::from("..##.#..#."),
      String::from(""),
      String::from("Tile 1489:"),
      String::from("##.#.#...."),
      String::from("..##...#.."),
      String::from(".##..##..."),
      String::from("..#...#..."),
      String::from("#####...#."),
      String::from("#..#.#.#.#"),
      String::from("...#.#.#.."),
      String::from("##.#...##."),
      String::from("..##.##.##"),
      String::from("###.##.#.."),
      String::from(""),
      String::from("Tile 2473:"),
      String::from("#....####."),
      String::from("#..#.##..."),
      String::from("#.##..#..."),
      String::from("######.#.#"),
      String::from(".#...#.#.#"),
      String::from(".#########"),
      String::from(".###.#..#."),
      String::from("########.#"),
      String::from("##...##.#."),
      String::from("..###.#.#."),
      String::from(""),
      String::from("Tile 2971:"),
      String::from("..#.#....#"),
      String::from("#...###..."),
      String::from("#.#.###..."),
      String::from("##.##..#.."),
      String::from(".#####..##"),
      String::from(".#..####.#"),
      String::from("#..#.#..#."),
      String::from("..####.###"),
      String::from("..#.#.###."),
      String::from("...#.#.#.#"),
      String::from(""),
      String::from("Tile 2729:"),
      String::from("...#.#.#.#"),
      String::from("####.#...."),
      String::from("..#.#....."),
      String::from("....#..#.#"),
      String::from(".##..##.#."),
      String::from(".#.####..."),
      String::from("####.#.#.."),
      String::from("##.####..."),
      String::from("##..#.##.."),
      String::from("#.##...##."),
      String::from(""),
      String::from("Tile 3079:"),
      String::from("#.#.#####."),
      String::from(".#..######"),
      String::from("..#......."),
      String::from("######...."),
      String::from("####.#..#."),
      String::from(".#...#.##."),
      String::from("#.#####.##"),
      String::from("..#.###..."),
      String::from("..#......."),
      String::from("..#.###..."),
    ]
  }
}
