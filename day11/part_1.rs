use std::{env, fs};
use std::collections::LinkedList;

fn process_stone(stone: u64) -> LinkedList<u64> {
  let mut result: LinkedList<u64> = LinkedList::new();
  let stone_string: String = stone.to_string();

  if stone == 0 {
    result.push_back(1);
  } else if stone_string.len() % 2 == 0 {
    let part_left: u64 = (&stone_string[0..stone_string.len()/2]).parse().unwrap();
    let part_right: u64 = (&stone_string[stone_string.len()/2..stone_string.len()]).parse().unwrap();
    result.push_back(part_left);
    result.push_back(part_right);
  } else {
    result.push_back(stone * 2024);
  }

  result
}

fn blink(stones: &mut LinkedList<u64>) {
  let mut result: LinkedList<u64> = LinkedList::new();

  for stone in stones.iter().copied() {
    let mut processed_stone = process_stone(stone);
    result.append(&mut processed_stone);
  }
  
  *stones = result;
}

fn parse_input(input: &String) -> LinkedList<u64> {
  input
    .split([' ', '\n'])
    .filter(|token| !token.is_empty())
    .map(|token| token.parse().unwrap())
    .collect()
}

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    println!("Not enough arguments");
    return;
  }

  let file_name = &args[1];
  let input: String = fs::read_to_string(file_name)
    .expect("Error reading the input file");
  let mut stones = parse_input(&input);
  for _ in 0..25 {
    blink(&mut stones);
  }
  println!("Stones after blink 25 times: {}", stones.len());
}
