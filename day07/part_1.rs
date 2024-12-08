use std::{env,fs};
use std::collections::LinkedList;

// An operation is a binary number who indicates the operator to use for each number in the equation 
// For example:
//   The equation 81 40 27
//   And if you wants to do 81 * 40 + 27
//   The corresponding operation in binary is 10:
//     - code 1 is multiplication
//     - code 0 is addition
//   
//   6 8 6 15 with operation 101 = 6 * 8 + 6 * 15
fn evaluate_operation(operation: u64, numbers: &Vec<i64>) -> i64 {
  let mut result = numbers[0];
  for i in 1..numbers.len() {
    if (operation / 2u64.pow(i as u32)) % 2 == 0 {
      result += numbers[i];
    } else {
      result *= numbers[i];
    }
  }
  result
}

fn validate_equation(equation: &(i64, Vec<i64>)) -> bool {
  let posible_operations = 2u64.pow(equation.1.len() as u32);
  (0..posible_operations)
  .any(|operation| {
    evaluate_operation(operation, &equation.1) == equation.0
  })
}

fn parse_input(input: &String) -> LinkedList<(i64, Vec<i64>)> {
  let mut result: LinkedList<(i64, Vec<i64>)> = LinkedList::new();

  for line in input.lines() {
    let mut test_value: Option<i64> = None;
    let mut equation: Vec<i64> = Vec::new();
    for token in line.split([':', ' ']).filter(|token| !token.is_empty()) {
      if test_value.is_none() {
        test_value = Some(token.parse().unwrap());
        continue;
      }
      equation.push(token.parse().unwrap());
    }

    result.push_back((test_value.unwrap(), equation));
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
  let input: String = fs::read_to_string(file_name)
    .expect("Error reading the input file");
  let equations = parse_input(&input);
  let result: i64 = equations
    .iter()
    .filter(|equation| validate_equation(equation))
    .map(|equation| equation.0)
    .sum();

  println!("Result: {}", result);
}
