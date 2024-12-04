use std::{env, fs};

fn x_mas_match(i: usize, j: usize, matrix: &Vec<Vec<char>>) -> bool {
  if matrix[i][j] != 'A' { return false }
  if matrix[i - 1][j - 1] == 'A' || matrix[i - 1][j - 1] == 'X' ||
     matrix[i + 1][j + 1] == 'A' || matrix[i + 1][j + 1] == 'X' ||
     matrix[i - 1][j - 1] == matrix[i + 1][j + 1]
  {
    return false
  }
  if matrix[i - 1][j + 1] == 'A' || matrix[i - 1][j + 1] == 'X' ||
     matrix[i + 1][j - 1] == 'A' || matrix[i + 1][j - 1] == 'X' ||
     matrix[i - 1][j + 1] == matrix[i + 1][j - 1] {
    return false
  }

  true
}

fn x_mas_search(matrix: &Vec<Vec<char>>) -> usize {
  // for i in (1..matrix.len() - 1) {
  //   for j in (1..matrix.len() - 1) {
  //     if x_mas_match(i, j, matrix) {
  //       println!("{}{}{}", matrix[i-1][j-1], matrix[i-1][j], matrix[i-1][j+1]);
  //       println!("{}{}{}", matrix[i][j-1], matrix[i][j], matrix[i][j+1]);
  //       println!("{}{}{}\n", matrix[i+1][j-1], matrix[i+1][j], matrix[i+1][j+1]);
  //     }
  //   }
  // }

  (1..matrix.len() - 1)
  .map(|i| {
    (1..matrix.len() - 1)
    .filter(|j| { x_mas_match(i, *j, matrix) })
    .count()
  })
  .sum()
}

fn parse_input(input: &String) -> Vec<Vec<char>> {
  input.lines().map(|line| { line.chars().collect() }).collect()
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

  let matrix = parse_input(&buffer);
  let x_mas_count = x_mas_search(&matrix);

  println!("Result: {}\n", x_mas_count);
}
