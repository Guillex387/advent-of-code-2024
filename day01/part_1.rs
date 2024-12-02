use std::{env, fs};

fn parse_input(str: &String) -> (Vec<i32>, Vec<i32>) {
  let mut list1: Vec<i32> = Vec::new();
  let mut list2: Vec<i32> = Vec::new();

  for line_str in str.lines() {
    let line = line_str.to_string();
    let mut is_list1 = true;
    
    for token in line.replace("   ", " ").split([' ', '\t']) {
      if token.is_empty() {
        continue;
      }
      
      if is_list1 {
        list1.push(token.parse().unwrap());
        is_list1 = false;
      } else {
        list2.push(token.parse().unwrap());
      }
    }

  }

  list1.sort();
  list2.sort();

  (list1, list2)
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

  let (list1, list2) = parse_input(&buffer);

  let computed: i32 = list1.iter().enumerate().map(|(i, n1)| {
    let n2 = &list2[i];

    (n1 - n2).abs()
  }).sum();

  println!("Result: {}\n", computed);
}
