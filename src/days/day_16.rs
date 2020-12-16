use crate::days::Day;

pub struct Day16 {}

impl Day for Day16 {
  fn day_num(&self) -> usize {
    16
  }

  fn puzzle_1(&self, input: &Vec<String>) -> String {
    let taf = parse_tickets_and_fields(input);

    let mut error_rate = 0;

    for ticket in &taf.nearby_tickets {
      for value in ticket {
        if !&taf.fits(*value) {
          error_rate += value;
        }
      }
    }

    error_rate.to_string()
  }

  fn puzzle_2(&self, input: &Vec<String>) -> String {
    let mut taf = parse_tickets_and_fields(input);
    taf.eliminate_invalid();

    let mut field_columns = Vec::new();
    for field in &taf.fields {
      field_columns.push(taf.field_column(field));
    }

    for _ in 0..field_columns.len() {
      field_columns = reduce_possible(&field_columns);
    }

    let mut departure_totals = 1;
    for i in 0..field_columns.len() {
      if taf.fields[i].name.starts_with("departure") {
        departure_totals *= taf.my_ticket[field_columns[i][0]];
      }
    }

    departure_totals.to_string()
  }
}

struct TicketsAndFields {
  fields: Vec<Field>,
  my_ticket: Vec<usize>,
  nearby_tickets: Vec<Vec<usize>>,
}

impl TicketsAndFields {
  fn fits(&self, num: usize) -> bool {
    for field in &self.fields {
      if field.fits(num) {
        return true;
      }
    }

    false
  }

  fn eliminate_invalid(&mut self) {
    let mut new_nearby = Vec::new();

    for ticket in &self.nearby_tickets {
      let mut matches = true;
      for value in ticket {
        if !&self.fits(*value) {
          matches = false;
          break;
        }
      }

      if matches {
        new_nearby.push(ticket.clone());
      }
    }

    self.nearby_tickets = new_nearby;
  }

  fn field_column(&self, field: &Field) -> Vec<usize> {
    let mut columns = Vec::new();

    for i in 0..self.my_ticket.len() {
      let mut valid = true;
      for ticket in &self.nearby_tickets {
        if !field.fits(ticket[i]) {
          valid = false;
        }
      }

      if valid {
        columns.push(i);
      } 
    }

    columns
  }
}

#[derive(Debug, PartialEq)]
struct Field {
  name: String,
  first_lower: usize,
  first_upper: usize,
  second_lower: usize,
  second_upper: usize,
}

impl Field {
  fn fits(&self, num: usize) -> bool {
    (num >= self.first_lower && num <= self.first_upper) || (num >= self.second_lower && num <= self.second_upper)
  }
}

fn parse_tickets_and_fields(input: &Vec<String>) -> TicketsAndFields {
  let mut taf = TicketsAndFields {
    fields: Vec::new(),
    my_ticket: Vec::new(),
    nearby_tickets: Vec::new(),
  };

  let mut part = 0;
  for line in input {
    if line.is_empty() {
      part += 1;
    }

    match part {
      0 => taf.fields.push(parse_field(line)),
      1 => if line.contains(",") {taf.my_ticket = parse_ticket(line);}, 
      2 => if line.contains(",") {taf.nearby_tickets.push(parse_ticket(line));}, 
      _ => println!("Not sure what we are processing right now.")
    }
  }

  taf
}

fn parse_field(line: &str) -> Field {
  let mut parts = line.split(": ");
  let name = String::from(parts.next().unwrap());
  let mut words = parts.next().unwrap().split(" ");
  let mut first = words.next().unwrap().split("-");
  let first_lower = first.next().unwrap().parse::<usize>().unwrap();
  let first_upper = first.next().unwrap().parse::<usize>().unwrap();
  words.next();
  let mut second = words.next().unwrap().split("-");
  let second_lower = second.next().unwrap().parse::<usize>().unwrap();
  let second_upper = second.next().unwrap().parse::<usize>().unwrap();

  Field {
    name: name,
    first_lower: first_lower,
    first_upper: first_upper,
    second_lower: second_lower,
    second_upper: second_upper,
  }
}

fn parse_ticket(line: &str) -> Vec<usize> {
  line.split(",")
    .map(|n| n.parse::<usize>().unwrap())
    .collect()
}

fn reduce_possible(fields: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
  let mut to_remove = Vec::new();
  for field in fields {
    if field.len() == 1 {
      to_remove.push(field[0]);
    }
  }

  let mut new_fields = Vec::new();
  for field in fields {
    if field.len() == 1 {
      new_fields.push(vec![field[0]]);
    } else {
      new_fields.push(remove(field, &to_remove));
    }
  }

  new_fields
}

fn remove(field: &Vec<usize>, to_remove: &Vec<usize>) -> Vec<usize> {
  let mut new_field = Vec::new();
  for f in field {
    if !contains(to_remove, f) {
      new_field.push(*f);
    }
  }

  new_field
}

fn contains(vec: &Vec<usize>, num: &usize) -> bool {
  for v in vec {
    if v == num {
      return true;
    }
  }

  false
}

#[cfg(test)]
mod tests {
  use super::*;

  fn sample_input_1() -> Vec<String> {
    vec![
      String::from("departure location: 1-3 or 5-7"),
      String::from("row: 6-11 or 33-44"),
      String::from("seat: 13-40 or 45-50"),
      String::from(""),
      String::from("your ticket:"),
      String::from("7,1,14"),
      String::from(""),
      String::from("nearby tickets:"),
      String::from("7,3,47"),
      String::from("40,4,50"),
      String::from("55,2,20"),
      String::from("38,6,12"),
    ]
  }

  #[test]
  fn test_parse_tickets_and_fields() {
    let taf = parse_tickets_and_fields(&sample_input_1());
    assert_eq!(taf.fields.len(), 3);
    assert_eq!(taf.fields[0], Field{name: String::from("departure location"), first_lower: 1, first_upper: 3, second_lower: 5, second_upper: 7});
    assert_eq!(taf.fields[1], Field{name: String::from("row"), first_lower: 6, first_upper: 11, second_lower: 33, second_upper: 44});
    assert_eq!(taf.fields[2], Field{name: String::from("seat"), first_lower: 13, first_upper: 40, second_lower: 45, second_upper: 50});
    assert_eq!(taf.my_ticket, vec![7, 1, 14]);
    assert_eq!(taf.nearby_tickets.len(), 4);
    assert_eq!(taf.nearby_tickets[0], vec![7, 3, 47]);
    assert_eq!(taf.nearby_tickets[1], vec![40, 4, 50]);
    assert_eq!(taf.nearby_tickets[2], vec![55, 2, 20]);
    assert_eq!(taf.nearby_tickets[3], vec![38, 6, 12]);
  }

  #[test]
  fn test_fits() {
    let taf = parse_tickets_and_fields(&sample_input_1());
    assert_eq!(taf.fits(7), true);
    assert_eq!(taf.fits(47), true);
    assert_eq!(taf.fits(4), false);
    assert_eq!(taf.fits(2), true);
    assert_eq!(taf.fits(55), false); 
  }

  #[test]
  fn test_puzzle_1() {
    assert_eq!(Day16{}.puzzle_1(&sample_input_1()), "71");
  }

  #[test]
  fn test_eliminate_invalid() {
    let mut taf = parse_tickets_and_fields(&sample_input_1());
    taf.eliminate_invalid();
    assert_eq!(taf.nearby_tickets.len(), 1);
    assert_eq!(taf.nearby_tickets[0], vec![7, 3, 47]);
  }

  fn sample_input_2() -> Vec<String> {
    vec![
      String::from("departure location: 0-1 or 4-19"),
      String::from("row: 0-5 or 8-19"),
      String::from("seat: 0-13 or 16-19"),
      String::from(""),
      String::from("your ticket:"),
      String::from("11,12,13"),
      String::from(""),
      String::from("nearby tickets:"),
      String::from("3,9,18"),
      String::from("15,1,5"),
      String::from("5,14,9"),
    ]
  }

  #[test]
  fn test_field_column() {
    let taf = parse_tickets_and_fields(&sample_input_2());
    assert_eq!(taf.field_column(&taf.fields[0]), vec![1, 2]);
    assert_eq!(taf.field_column(&taf.fields[1]), vec![0, 1, 2]);
    assert_eq!(taf.field_column(&taf.fields[2]), vec![2]);
  }

  #[test]
  fn test_reduce_possible() {
    let mut fields = vec![
      vec![1, 2],
      vec![0, 1, 2],
      vec![2],
    ];
    fields = reduce_possible(&fields);
    assert_eq!(fields, vec![vec![1], vec![0, 1], vec![2]]);
    fields = reduce_possible(&fields);
    assert_eq!(fields, vec![vec![1], vec![0], vec![2]]);
    fields = reduce_possible(&fields);
    assert_eq!(fields, vec![vec![1], vec![0], vec![2]]);
  }
}
