use std::{env,fs};
use std::collections::LinkedList;

fn concatenation(number1: i64, number2: i64) -> i64 {
  let mut number2_copy = number2;
  let mut multiplier = 1;
  while number2_copy != 0 {
    multiplier *= 10;
    number2_copy /= 10;
  }

  number1 * multiplier + number2
}

// An operation is a ternary number who indicates the operator to use for each number in the equation 
// For example:
//   6 8 6 15 with operation 012 = 6 + 8 * 6 || 15
//   The corresponding operation in ternary is 012:
//     - code 0 is addition
//     - code 1 is multiplication
//     - code 2 is concatenation
fn evaluate_operation(operation: u64, numbers: &Vec<i64>) -> i64 {
  let mut result = numbers[0];
  for i in 1..numbers.len() {
    let operator = (operation / 3u64.pow(i as u32)) % 3;
    let number = numbers[i];
    match operator {
      0 => result += number,
      1 => result *= number,
      _ => result = concatenation(result, number)  
    }
  }
  result
}

fn validate_equation(equation: &(i64, Vec<i64>)) -> bool {
  let posible_operations = 3u64.pow(equation.1.len() as u32);
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
