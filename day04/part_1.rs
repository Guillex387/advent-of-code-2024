use std::{env, fs};

fn xmas_match(line: &String) -> usize {
  line.matches("XMAS").count() + line.matches("SAMX").count()
}

fn xmas_search(matrix: &Vec<Vec<char>>) -> usize {
  let horizontal_count: usize = matrix
    .iter()
    .map(|row| {
      let line: String = row.iter().copied().collect();
      xmas_match(&line)
    })
    .sum();

  let vertical_count: usize = (0..matrix.first().unwrap().len())
    .map(|j| {
      let column: String = matrix.iter().map(|row| { row[j] }).collect();
      xmas_match(&column)
    })
    .sum();
  
  let first_diagonal_count: usize = (0..matrix.len() - 3)
    .map(|i| {
      let diagonal: String = (0..matrix.len())
        .map(|j| {
          if i + j >= matrix.len() {
            '\0'
          } else {
            matrix[j][i + j]
          }
        })
        .filter(|c| { *c != '\0' })
        .collect();
      xmas_match(&diagonal)
    })
    .sum();

  let neg_first_diagonal_count: usize = (1..matrix.len() - 3)
    .map(|i| {
      let diagonal: String = (0..matrix.len())
        .map(|j| {
          if i + j >= matrix.len() {
            '\0'
          } else {
            matrix[i + j][j]
          }
        })
        .filter(|c| { *c != '\0' })
        .collect();
      xmas_match(&diagonal)
    })
    .sum();

  let reversed_matrix: Vec<Vec<char>> = matrix
    .iter()
    .map(|row| row.iter().copied().rev().collect())
    .collect();

  let second_diagonal_count: usize = (0..matrix.len() - 3)
    .map(|i| {
      let diagonal: String = (0..matrix.len())
        .map(|j| {
          if i + j >= matrix.len() {
            '\0'
          } else {
            reversed_matrix[j][i + j]
          }
        })
        .filter(|c| { *c != '\0' })
        .collect();
      xmas_match(&diagonal)
    })
    .sum();

  let neg_second_diagonal_count: usize = (1..matrix.len() - 3)
    .map(|i| {
      let diagonal: String = (0..matrix.len())
        .map(|j| {
          if i + j >= matrix.len() {
            '\0'
          } else {
            reversed_matrix[i + j][j]
          }
        })
        .filter(|c| { *c != '\0' })
        .collect();
      xmas_match(&diagonal)
    })
    .sum();

  horizontal_count +
  vertical_count +
  first_diagonal_count +
  neg_first_diagonal_count +
  second_diagonal_count +
  neg_second_diagonal_count
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
  let xmas_count = xmas_search(&matrix);

  // println!("{:?}\n", matrix);
  println!("Result: {}\n", xmas_count);
}
