use std::collections::LinkedList;
use std::{fs, env};

#[derive(Debug)]
enum InstructionKind {
  DONT,
  DO,
  MUL,
}

#[derive(Debug)]
struct Instruction {
  kind: InstructionKind,
  x: i64,
  y: i64,
}

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
      if !token.is_empty() {
        tokens.push_back(token.clone());
        token.clear();
      }

      tokens.push_back(letter.to_string());
      continue;
    }

    token.push(letter);
  }

  tokens
}

fn parse_input(mut tokens: LinkedList<String>) -> LinkedList<Instruction> {
  let mut result: LinkedList<Instruction> = LinkedList::new();

  loop {
    let poped = tokens.pop_front();
    if poped.is_none() {
      break;
    }

    let keyword = poped.clone().unwrap();

    if keyword.ends_with("mul") {
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

      result.push_back(Instruction {
        kind: InstructionKind::MUL,
        x: number_x.unwrap().parse().unwrap(),
        y: number_y.unwrap().parse().unwrap(),
      });
    }

    if keyword.ends_with("do") {
      let open_paren = tokens.pop_front();
      if open_paren.is_none() || open_paren.clone().unwrap() != "(" {
        tokens.push_front(open_paren.unwrap());
        continue;
      }
      let close_paren = tokens.pop_front();
      if close_paren.is_none() || close_paren.clone().unwrap() != ")" {
        tokens.push_front(close_paren.unwrap());
        continue;
      }

      result.push_back(Instruction { kind: InstructionKind::DO, x: 0, y: 0 });
    }

    if keyword.ends_with("don't") {
      let open_paren = tokens.pop_front();
      if open_paren.is_none() || open_paren.clone().unwrap() != "(" {
        tokens.push_front(open_paren.unwrap());
        continue;
      }
      let close_paren = tokens.pop_front();
      if close_paren.is_none() || close_paren.clone().unwrap() != ")" {
        tokens.push_front(close_paren.unwrap());
        continue;
      }

      result.push_back(Instruction { kind: InstructionKind::DONT, x: 0, y: 0 });
    }
  }

  result
}

fn execute_instructions(instructions: LinkedList<Instruction>) -> i64 {
  let mut enable = true;
  let mut result = 0;

  for ins in instructions {
    match ins.kind {
      InstructionKind::DO => enable = true,
      InstructionKind::DONT => enable = false,
      InstructionKind::MUL => result += if enable { ins.x * ins.y } else { 0 },
    }
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
  let instructions = parse_input(tokens);
  let result = execute_instructions(instructions);
  println!("Result: {}\n", result);
}
