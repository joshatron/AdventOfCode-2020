use crate::days::Day;

pub struct Day4 {}

impl Day for Day4 {
  fn day_num(&self) -> usize {
    4
  }

  fn puzzle_1(&self, input: &Vec<String>) -> String {
    let passports = create_passports(input);

    let mut valid = 0;

    for passport in passports {
      if is_not_empty(&passport) {
        valid = valid + 1;
      }
    }

    valid.to_string()
  }

  fn puzzle_2(&self, input: &Vec<String>) -> String {
    let passports = create_passports(input);

    let mut valid = 0;

    for passport in passports {
      if is_valid(&passport) {
        valid = valid + 1;
      }
    }

    valid.to_string()
  }
}

fn is_not_empty(passport: &Passport) -> bool {
  !passport.birth_year.is_empty() && !passport.issue_year.is_empty() && !passport.expiration_year.is_empty() && !passport.height.is_empty() &&
  !passport.hair_color.is_empty() && !passport.eye_color.is_empty() && !passport.passport_id.is_empty()
}

fn is_valid(passport: &Passport) -> bool {
  is_num_valid(&passport.birth_year, 1920, 2002) && is_num_valid(&passport.issue_year, 2010, 2020) &&
  is_num_valid(&passport.expiration_year, 2020, 2030) && is_height_valid(&passport.height) &&
  is_hair_color_valid(&passport.hair_color) && is_eye_color_valid(&passport.eye_color) &&
  is_passport_id_valid(&passport.passport_id)
}

fn is_num_valid(num: &str, start: usize, end: usize) -> bool {
  match num.parse::<usize>() {
    Ok(y) => {
      if y < start || y > end {
        return false;
      }
    },
    Err(_) => return false,
  }

  true
}

fn is_height_valid(height: &str) -> bool {
  if height.ends_with("in") {
    is_num_valid(&height[..(height.len() - 2)], 59, 76)
  } else if height.ends_with("cm") {
    is_num_valid(&height[..(height.len() - 2)], 150, 193)
  } else {
    false
  }
}

fn is_hair_color_valid(hair_color: &str) -> bool {
  if hair_color.len() != 7 {
    return false;
  }

  let mut first = true;
  for c in hair_color.chars() {
    if first {
      first = false;
      if c != '#' {
        return false;
      }
    } else {
      if !c.is_digit(16) {
        return false;
      }
    }
  }

  true
}

fn is_eye_color_valid(eye_color: &str) -> bool {
  eye_color == "amb" || eye_color == "blu" || eye_color == "brn" || eye_color == "gry" ||
  eye_color == "grn" || eye_color == "hzl" || eye_color == "oth"
}

fn is_passport_id_valid(passport_id: &str) -> bool {
  if passport_id.len() != 9 {
    return false;
  }

  for c in passport_id.chars() {
    if !c.is_digit(10) {
      return false;
    }
  }

  true
}

struct Passport {
  birth_year: String,
  issue_year: String,
  expiration_year: String,
  height: String,
  hair_color: String,
  eye_color: String,
  passport_id: String,
  country_id: String
}

fn create_passports(input: &Vec<String>) -> Vec<Passport> {
  let mut passports = vec![];

  let mut current_passport = String::from("");
  for line in input {
    if line == "" {
      passports.push(get_passport(&current_passport));
      current_passport = String::from("");
    } else {
      current_passport.push_str(line);
      current_passport.push_str(" ");
    }
  }

  if current_passport != "" {
    passports.push(get_passport(&current_passport));
  }

  passports
}

fn get_passport(line: &str) -> Passport {
  let mut passport = empty_passport();

  for field in separate_passport_fields(line) {
    add_field_to_passport(&mut passport, field);
  }

  passport
}

fn empty_passport() -> Passport {
  Passport{
    birth_year: String::from(""),
    issue_year: String::from(""),
    expiration_year: String::from(""),
    height: String::from(""),
    hair_color: String::from(""),
    eye_color: String::from(""),
    passport_id: String::from(""),
    country_id: String::from(""),
  }
}

fn separate_passport_fields(fields: &str) -> Vec<&str> {
  fields.trim().split(' ').collect()
}

fn add_field_to_passport(passport: &mut Passport, field: &str) {
  match &field[..3] {
    "byr" => passport.birth_year = get_value_from_field(field),
    "iyr" => passport.issue_year = get_value_from_field(field),
    "eyr" => passport.expiration_year = get_value_from_field(field),
    "hgt" => passport.height = get_value_from_field(field),
    "hcl" => passport.hair_color = get_value_from_field(field),
    "ecl" => passport.eye_color = get_value_from_field(field),
    "pid" => passport.passport_id = get_value_from_field(field),
    "cid" => passport.country_id = get_value_from_field(field),
    _ => println!("Illegal field: {}", field),
  }
}

fn get_value_from_field(field: &str) -> String {
  String::from(&field[4..])
}

#[cfg(test)]
mod tests {
  use super::*;

  fn sample_input_1() -> Vec<String> {
    vec![
      String::from("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd"),
      String::from("byr:1937 iyr:2017 cid:147 hgt:183cm"),
      String::from(""),
      String::from("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884"),
      String::from("hcl:#cfa07d byr:1929"),
      String::from(""),
      String::from("hcl:#ae17e1 iyr:2013"),
      String::from("eyr:2024"),
      String::from("ecl:brn pid:760753108 byr:1931"),
      String::from("hgt:179cm"),
      String::from(""),
      String::from("hcl:#cfa07d eyr:2025 pid:166559648"),
      String::from("iyr:2011 ecl:brn hgt:59in"),
    ]
  }

  #[test]
  fn test_create_passports() {
    let passports = create_passports(&sample_input_1());
    
    assert_eq!(passports[0].birth_year, "1937");
    assert_eq!(passports[0].issue_year, "2017");
    assert_eq!(passports[0].expiration_year, "2020");
    assert_eq!(passports[0].height, "183cm");
    assert_eq!(passports[0].hair_color, "#fffffd");
    assert_eq!(passports[0].eye_color, "gry");
    assert_eq!(passports[0].passport_id, "860033327");
    assert_eq!(passports[0].country_id, "147");

    assert_eq!(passports[3].birth_year, "");
    assert_eq!(passports[3].issue_year, "2011");
    assert_eq!(passports[3].expiration_year, "2025");
    assert_eq!(passports[3].height, "59in");
    assert_eq!(passports[3].hair_color, "#cfa07d");
    assert_eq!(passports[3].eye_color, "brn");
    assert_eq!(passports[3].passport_id, "166559648");
    assert_eq!(passports[3].country_id, "");
  }

  #[test]
  fn test_puzzle_1() {
    assert_eq!(Day4{}.puzzle_1(&sample_input_1()), "2");
  }

  fn sample_input_2() -> Vec<String> {
    vec![
      String::from("eyr:1972 cid:100"),
      String::from("hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"),
      String::from(""),
      String::from("iyr:2019"),
      String::from("hcl:#602927 eyr:1967 hgt:170cm"),
      String::from("ecl:grn pid:012533040 byr:1946"),
      String::from(""),
      String::from("hcl:dab227 iyr:2012"),
      String::from("ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277"),
      String::from(""),
      String::from("hgt:59cm ecl:zzz"),
      String::from("eyr:2038 hcl:74454a iyr:2023"),
      String::from("pid:3556412378 byr:2007"),
      String::from(""),
      String::from("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980"),
      String::from("hcl:#623a2f"),
      String::from(""),
      String::from("eyr:2029 ecl:blu cid:129 byr:1989"),
      String::from("iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm"),
      String::from(""),
      String::from("hcl:#888785"),
      String::from("hgt:164cm byr:2001 iyr:2015 cid:88"),
      String::from("pid:545766238 ecl:hzl"),
      String::from("eyr:2022"),
      String::from(""),
      String::from("iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"),
    ]
  }

  #[test]
  fn test_puzzle_2() {
    assert_eq!(Day4{}.puzzle_2(&sample_input_2()), String::from("4"));
  }
}