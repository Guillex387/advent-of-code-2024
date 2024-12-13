use std::{env, fs};
use std::collections::{HashMap, LinkedList};

const BLINK_TIMES: usize = 75;

fn process_stone(stone: u64, level: usize, cache: &mut HashMap<(u64, usize), usize>) -> usize {
  let cached_value = cache.get(&(stone, level));
  if cached_value.is_some() {
    return cached_value.unwrap().clone();
  }

  if level == BLINK_TIMES {
    return 1;
  }

  if stone == 0 {
    let result = process_stone(1, level + 1, cache);
    cache.insert((stone, level), result);
    return result;
  }

  let stone_len = stone.to_string().len();

  let result = if stone_len % 2 == 0 {
      let multiplier = 10u64.pow((stone_len / 2) as u32);

      let part_left = process_stone(stone / multiplier, level + 1, cache);
      let part_right = process_stone(stone % multiplier, level + 1, cache);

      part_left + part_right
    } else {
      process_stone(stone * 2024, level + 1, cache)
    };
  cache.insert((stone, level), result);
  return result;
}

fn blink(stones: LinkedList<u64>) -> usize {
  let mut cache: HashMap<(u64, usize), usize> = HashMap::new();
  let mut result: usize = 0;

  for stone in stones.iter().copied() {
    result += process_stone(stone, 0, &mut cache);
  }

  result
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
  let stones = parse_input(&input);
  println!("Stones after blink 25 times: {}", blink(stones));
}
