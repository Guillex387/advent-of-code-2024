use std::{env, fs};
use std::collections::LinkedList;

fn path_finder_recursive(init: &(usize, usize), matrix: &Vec<Vec<u32>>, ends: &mut LinkedList<(usize, usize)>) {
  if matrix[init.0][init.1] == 9 {
    ends.push_back(init.clone());
    return;
  }

  for (i, j) in vec![(-1, 0), (0, -1), (0, 1), (1, 0)] {
    let pos_x = (init.0 as i32 + i) as usize;
    let pos_y = (init.1 as i32 + j) as usize;
    if i == 0 && j == 0 { continue; }  
    if pos_x >= matrix.len() || pos_y >= matrix[0].len() { continue; }
    if matrix[init.0][init.1] + 1 == matrix[pos_x][pos_y] {
      path_finder_recursive(&(pos_x, pos_y), matrix, ends);
    }
  }
}

fn path_finder(matrix: &Vec<Vec<u32>>) -> usize {
  let mut finded = 0;
  for i in 0..matrix.len() {
    for j in 0..matrix[0].len() {
      if matrix[i][j] != 0 { continue; }
      let mut points: LinkedList<(usize, usize)> = LinkedList::new();
      path_finder_recursive(&(i, j), matrix, &mut points);
      finded += points.len();
    }
  }

  finded
}

fn parse_input(input: &String) -> Vec<Vec<u32>> {
  input
    .lines()
    .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
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
  let matrix = parse_input(&input);
  let finded_paths = path_finder(&matrix);
  println!("Result: {}", finded_paths);
}
