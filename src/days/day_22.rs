use crate::days::Day;
use std::collections::HashSet;

pub struct Day22 {}

impl Day22 {
  pub fn new() -> Day22 {
    Day22 {}
  }
}

impl Day for Day22 {
  fn day_num(&self) -> usize {
    22
  }

  fn puzzle_1(&self, input: &Vec<String>) -> String {
    let mut deck = Deck::parse(input);
    while !deck.person_one_cards[0].is_empty() && !deck.person_two_cards[0].is_empty() {
      deck.play_one_round_of_combat();
    }

    let winning_cards = if deck.person_one_cards[0].is_empty() {
      &deck.person_two_cards[0]
    } else {
      &deck.person_one_cards[0]
    };

    Deck::get_hand_score(winning_cards).to_string()
  }

  fn puzzle_2(&self, input: &Vec<String>) -> String {
    let mut deck = Deck::parse(input);
    while !deck.person_one_cards[0].is_empty() && !deck.person_two_cards[0].is_empty() {
      deck.play_one_round_of_recursive_combat();
    }

    let winning_cards = if deck.person_one_cards[0].is_empty() {
      &deck.person_two_cards[0]
    } else {
      &deck.person_one_cards[0]
    };

    Deck::get_hand_score(winning_cards).to_string()
  }
}

#[derive(Debug)]
struct Deck {
  person_one_cards: Vec<Vec<usize>>,
  person_two_cards: Vec<Vec<usize>>,
  in_play_cards: Vec<(usize, usize)>,
  previously_played: Vec<HashSet<(Vec<usize>, Vec<usize>)>>,
  new_game: bool,
}

impl Deck {
  fn parse(input: &Vec<String>) -> Deck {
    let mut deck = Deck {
      person_one_cards: Vec::new(),
      person_two_cards: Vec::new(),
      in_play_cards: Vec::new(),
      previously_played: Vec::new(),
      new_game: true,
    };
    deck.person_one_cards.push(Vec::new());
    deck.person_two_cards.push(Vec::new());
    deck.previously_played.push(HashSet::new());

    let mut state = 0;
    for line in input {
      match state {
        0 => state = 1,
        1 => {
          if line.is_empty() {
            state = 2;
          } else {
            deck.person_one_cards[0].push(line.parse::<usize>().unwrap());
          }
        }
        2 => state = 3,
        3 => deck.person_two_cards[0].push(line.parse::<usize>().unwrap()),
        _ => (),
      }
    }

    deck
  }

  fn play_one_round_of_combat(&mut self) {
    if !self.person_one_cards[0].is_empty() && !self.person_two_cards[0].is_empty() {
      let person_one_card = self.person_one_cards[0].remove(0);
      let person_two_card = self.person_two_cards[0].remove(0);

      if person_one_card > person_two_card {
        self.person_one_cards[0].push(person_one_card);
        self.person_one_cards[0].push(person_two_card);
      } else {
        self.person_two_cards[0].push(person_two_card);
        self.person_two_cards[0].push(person_one_card);
      }
    }
  }

  fn play_one_round_of_recursive_combat(&mut self) {
    if !self.person_one_cards[0].is_empty() && !self.person_two_cards[0].is_empty() {
      let depth = self.person_one_cards.len() - 1;
      let new_state = (
        self.person_one_cards[depth].clone(),
        self.person_two_cards[depth].clone(),
      );
      if self.previously_played[depth].contains(&new_state) {
        if self.person_one_cards.len() == 1 {
          self.person_two_cards.pop();
          self.person_two_cards.push(Vec::new());
        } else {
          self.previously_played.pop();
          self.person_one_cards.pop();
          self.person_two_cards.pop();
          let top_cards = self.in_play_cards.pop().unwrap();
          self.get_top_person_one_hand().push(top_cards.0);
          self.get_top_person_one_hand().push(top_cards.1);
        }
      } else if self.person_one_cards[depth].is_empty() {
        self.previously_played.pop();
        self.person_one_cards.pop();
        self.person_two_cards.pop();
        let top_cards = self.in_play_cards.pop().unwrap();
        self.get_top_person_two_hand().push(top_cards.1);
        self.get_top_person_two_hand().push(top_cards.0);
      } else if self.person_two_cards[depth].is_empty() {
        self.previously_played.pop();
        self.person_one_cards.pop();
        self.person_two_cards.pop();
        let top_cards = self.in_play_cards.pop().unwrap();
        self.get_top_person_one_hand().push(top_cards.0);
        self.get_top_person_one_hand().push(top_cards.1);
      } else {
        self.previously_played[depth].insert(new_state);
        let top_cards = (
          self.person_one_cards[depth].remove(0),
          self.person_two_cards[depth].remove(0),
        );

        if self.person_one_cards[depth].len() >= top_cards.0
          && self.person_two_cards[depth].len() >= top_cards.1
        {
          let new_person_one_cards = clone_first_n(self.get_top_person_one_hand(), top_cards.0);
          let new_person_two_cards = clone_first_n(self.get_top_person_two_hand(), top_cards.1);
          if can_short_circuit(&new_person_one_cards, &new_person_two_cards) {
            self.get_top_person_one_hand().push(top_cards.0);
            self.get_top_person_one_hand().push(top_cards.1);
          } else {
            self.previously_played.push(HashSet::new());
            self.in_play_cards.push(top_cards);
            self.person_one_cards.push(new_person_one_cards);
            self.person_two_cards.push(new_person_two_cards);
          }
        } else {
          if top_cards.0 > top_cards.1 {
            self.get_top_person_one_hand().push(top_cards.0);
            self.get_top_person_one_hand().push(top_cards.1);
          } else {
            self.get_top_person_two_hand().push(top_cards.1);
            self.get_top_person_two_hand().push(top_cards.0);
          }
        }
      }
    }
  }

  fn get_top_person_one_hand(&mut self) -> &mut Vec<usize> {
    let depth = self.person_one_cards.len() - 1;
    &mut self.person_one_cards[depth]
  }

  fn get_top_person_two_hand(&mut self) -> &mut Vec<usize> {
    let depth = self.person_two_cards.len() - 1;
    &mut self.person_two_cards[depth]
  }

  fn get_hand_score(hand: &Vec<usize>) -> usize {
    let mut multiplier = 1;
    let mut total = 0;
    for card in hand.iter().rev() {
      total += card * multiplier;
      multiplier += 1;
    }

    total
  }
}

fn can_short_circuit(player_one_deck: &Vec<usize>, player_two_deck: &Vec<usize>) -> bool {
  let max_player_one = player_one_deck.iter().max().unwrap();
  let max_player_two = player_two_deck.iter().max().unwrap();

  max_player_one > max_player_two && *max_player_one > player_one_deck.len() - 1
}

fn clone_first_n(original: &Vec<usize>, n: usize) -> Vec<usize> {
  let mut v = Vec::new();
  for i in 0..n {
    v.push(original[i]);
  }

  v
}

#[cfg(test)]
mod tests {
  use super::*;

  fn sample_input_1() -> Vec<String> {
    vec![
      String::from("Player 1:"),
      String::from("9"),
      String::from("2"),
      String::from("6"),
      String::from("3"),
      String::from("1"),
      String::from(""),
      String::from("Player 2:"),
      String::from("5"),
      String::from("8"),
      String::from("4"),
      String::from("7"),
      String::from("10"),
    ]
  }

  #[test]
  fn test_parse_deck() {
    let deck = Deck::parse(&sample_input_1());
    assert_eq!(deck.person_one_cards[0].len(), 5);
    assert_eq!(deck.person_two_cards[0].len(), 5);
    assert_eq!(deck.person_one_cards[0][0], 9);
    assert_eq!(deck.person_one_cards[0][4], 1);
    assert_eq!(deck.person_two_cards[0][0], 5);
    assert_eq!(deck.person_two_cards[0][4], 10);
  }

  #[test]
  fn test_play_one_round_of_combat() {
    let mut deck = Deck::parse(&sample_input_1());
    deck.play_one_round_of_combat();
    assert_eq!(deck.person_one_cards[0].len(), 6);
    assert_eq!(deck.person_two_cards[0].len(), 4);
    assert_eq!(deck.person_two_cards[0][0], 8);
    assert_eq!(deck.person_one_cards[0][4], 9);
    assert_eq!(deck.person_one_cards[0][5], 5);
  }

  #[test]
  fn test_puzzle_1() {
    assert_eq!(Day22::new().puzzle_1(&sample_input_1()), "306");
  }

  fn sample_input_2() -> Vec<String> {
    vec![
      String::from("Player 1:"),
      String::from("43"),
      String::from("19"),
      String::from(""),
      String::from("Player 2:"),
      String::from("2"),
      String::from("29"),
      String::from("14"),
    ]
  }

  #[test]
  fn test_stop_infinite_recursive_game() {
    assert_eq!(Day22::new().puzzle_2(&sample_input_2()), "105");
  }

  #[test]
  fn test_puzzle_2() {
    assert_eq!(Day22::new().puzzle_2(&sample_input_1()), "291");
  }
}
