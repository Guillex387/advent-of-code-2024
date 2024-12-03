use std::collections::LinkedList;
use std::{fs, env};

fn is_separator(letter: char) -> bool {
  "(),".contains(letter)
}

fn is_number(token: &String) -> bool {
  token.chars().all(|letter| { letter.is_ascii_digit() })
}

fn tokenize_input(input: &String) -> LinkedList<String> {
  let mut tokens: LinkedList<String> = LinkedList::new();
  let mut token = String::new();
  for letter in input.chars() {
    if is_separator(letter) {
      if token.is_empty() {
      } else if token.ends_with("mul") {
        tokens.push_back("mul".to_string());
      } else {
        tokens.push_back(token.clone());
      }
      token.clear();

      tokens.push_back(letter.to_string());
      continue;
    }

    token.push(letter);
  }

  tokens
}

fn parse_input(mut tokens: LinkedList<String>) -> LinkedList<(i64, i64)> {
  let mut result: LinkedList<(i64, i64)> = LinkedList::new();

  loop {
    let poped = tokens.pop_front();
    if poped.is_none() {
      break;
    }

    if poped.clone().unwrap() != "mul" {
      continue;
    }
    let open_paren = tokens.pop_front();
    if open_paren.is_none() || open_paren.clone().unwrap() != "(" {
      tokens.push_front(open_paren.unwrap());
      continue;
    }
    let number_x = tokens.pop_front();
    if number_x.is_none() || !is_number(&number_x.clone().unwrap()) {
      tokens.push_front(number_x.unwrap());
      continue;
    }
    let comma = tokens.pop_front();
    if comma.is_none() || comma.clone().unwrap() != "," {
      tokens.push_front(comma.unwrap());
      continue;
    }
    let number_y = tokens.pop_front();
    if number_y.is_none() || !is_number(&number_y.clone().unwrap()) {
      tokens.push_front(number_y.unwrap());
      continue;
    }
    let close_paren = tokens.pop_front();
    if close_paren.is_none() || close_paren.clone().unwrap() != ")" {
      tokens.push_front(close_paren.unwrap());
      continue;
    }

    result.push_back((number_x.unwrap().parse().unwrap(), number_y.unwrap().parse().unwrap()));
  }

  result
}

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    println!("Not enough arguments");
    return;
  }

  let file_name = &args[1];
  let buffer: String = fs::read_to_string(file_name)
    .expect("Error reading the input file");

  let tokens = tokenize_input(&buffer);
  let muls = parse_input(tokens);
  let result: i64 = muls.iter().map(|mul| { mul.0 * mul.1 }).sum();

  println!("Result: {}\n", result);
}
