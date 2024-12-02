use std::collections::LinkedList;
use std::env;
use std::fs;

fn parse_input(content: &String) -> LinkedList<LinkedList<i64>> {
  let mut reports: LinkedList<LinkedList<i64>> = LinkedList::new();
  for line in content.lines() {
    let mut report: LinkedList<i64> = LinkedList::new();
    for token in line.split([' ']) {
      report.push_back(token.parse().unwrap());
    }
    reports.push_back(report);
  }

  reports
}

fn is_safe(report: &LinkedList<i64>) -> bool {
  let mut iter = report.iter();
  iter.next();

  let mut last_difference = 0;
  for level in report {
    let next_level_option = iter.next();
    if next_level_option.is_none() {
      break;
    }

    let next_level = next_level_option.unwrap();
    let diff = next_level - level;

    if diff.abs() < 1 || diff.abs() > 3 || last_difference * diff < 0 {
      return false;
    }

    last_difference = diff;
  }

  true
}

fn generate_report_without(report: &LinkedList<i64>, i: usize) -> LinkedList<i64> {
  report
    .iter()
    .copied()
    .enumerate()
    .filter(|(j, _)| { *j != i })
    .map(|(_, level)| { level })
    .collect()
}

fn is_safe_with_exception(report: &LinkedList<i64>) -> bool {
  let mut iter = report.iter();
  iter.next();

  let mut last_difference = 0;
  for (i, level) in report.iter().enumerate() {
    let next_level_option = iter.next();
    if next_level_option.is_none() {
      break;
    }

    let next_level = next_level_option.unwrap();
    let diff = next_level - level;

    if diff.abs() < 1 || diff.abs() > 3 || last_difference * diff < 0 {
      if i > 0 {
        let new_report0 = generate_report_without(&report, i - 1);
        let new_report1 = generate_report_without(&report, i);
        let new_report2 = generate_report_without(&report, i + 1);

        if is_safe(&new_report0) || is_safe(&new_report1) || is_safe(&new_report2) {
          return true;
        }
        return false;
      } else {
        let new_report1 = generate_report_without(&report, i);
        let new_report2 = generate_report_without(&report, i + 1);

        if is_safe(&new_report1) || is_safe(&new_report2) {
          return true;
        }
        return false;
      }
    }

    last_difference = diff;
  }

  true
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

  let reports = parse_input(&buffer);
  let n_safe = reports.iter().map(is_safe_with_exception).filter(|b| { *b }).count();

  println!("Result: {}\n", n_safe);
}
