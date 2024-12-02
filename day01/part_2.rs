use std::{collections::BTreeMap, env, fs};

fn parse_input(str: &String) -> (Vec<i32>, BTreeMap<i32, i32>) {
  let mut list: Vec<i32> = Vec::new();
  let mut map: BTreeMap<i32, i32> = BTreeMap::new();

  for line_str in str.lines() {
    let line = line_str.to_string();
    let mut is_list1 = true;
    
    for token in line.replace("   ", " ").split([' ', '\t']) {
      if token.is_empty() {
        continue;
      }
      
      if is_list1 {
        list.push(token.parse().unwrap());
        is_list1 = false;
      } else {
        let entry = map.entry(token.parse().unwrap()).or_insert(0);
        *entry += 1;
      }
    }

  }

  list.sort();

  (list, map)
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

  let (list, map) = parse_input(&buffer);

  let computed: i32 = list.iter().map(|id| {
    let appears = match map.get(id) {
      Some(res) => res,
      None => &0,
    };

    id * appears
  }).sum();

  println!("Result: {}\n", computed);
}
