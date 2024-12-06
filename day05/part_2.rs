use std::{env, fs};
use std::collections::{HashMap, HashSet, LinkedList};

fn order_pages(pages: &mut LinkedList<i64>, order: &HashMap<i64, HashSet<i64>>) {
  let mut result: LinkedList<i64> = LinkedList::new();

  while !pages.is_empty() {
    for page in pages.iter() {
      let pages_before = match order.get(page) {
        Some(pages) => pages.clone(),
        None => HashSet::new(),
      };

      if pages.iter().all(|p| page == p || pages_before.contains(p)) {
        result.push_front(page.clone());
        *pages = pages.iter().copied().filter(|p| p != page).collect();
        break;
      }
    }
  }

  *pages = result;
}

fn validate_pages(pages: &LinkedList<i64>, order: &HashMap<i64, HashSet<i64>>) -> bool {
  let mut end_part: LinkedList<i64> = pages.clone();
  end_part.pop_front();

  for page in pages {
    let pages_before = match order.get(page) {
      Some(pages) => pages,
      None => continue,
    };

    if end_part.iter().any(|p| pages_before.contains(p)) {
      return false;
    } 
    end_part.pop_front();
  }

  true
}

fn parse_input(input: &String) -> (HashMap<i64, HashSet<i64>>, LinkedList<LinkedList<i64>>) {
  // This map stores the numbers before the key
  let mut order: HashMap<i64, HashSet<i64>> = HashMap::new();
  let mut pages_list: LinkedList<LinkedList<i64>> = LinkedList::new();
  
  for line in input.lines() {
    if line.is_empty() { continue; }

    if line.contains("|") {
      let splitted: Vec<i64> = line.split('|').map(|num| num.parse().unwrap()).collect();
      let entry = order.entry(splitted[1]).or_insert(HashSet::new());
      entry.insert(splitted[0]);
    } else {
      pages_list.push_back(
        line
        .split(',')
        .filter(|num| !num.is_empty())
        .map(|num| num.parse().unwrap())
        .collect()
      );
    }
  }

  (order, pages_list)
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
  let (order, mut list_pages) = parse_input(&input);
  let result: i64 = list_pages
    .iter_mut()
    .filter(|pages| !validate_pages(pages, &order))
    .map(|pages| {
      order_pages(pages, &order);
      pages.iter().nth(pages.len() / 2).unwrap()
     })
    .sum();
  println!("Result: {}", result);
}
