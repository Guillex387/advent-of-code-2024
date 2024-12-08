use std::{char, env, fs};
use std::collections::{HashSet, HashMap, LinkedList};

fn calculate_antinodes(map: &HashMap<char, LinkedList<(i64, i64)>>, rows: i64, columns: i64) -> usize {
  let mut antinodes_set: HashSet<(i64, i64)> = HashSet::new();

  for (_, antennas) in map.iter() {
    for antenna_x in antennas {
      for antenna_y in antennas {
        if antenna_y == antenna_x { continue; }

        let diff_rows = antenna_x.0 - antenna_y.0;
        let diff_cols = antenna_x.1 - antenna_y.1;

        let antinode = (antenna_y.0 + diff_rows * 2, antenna_y.1 + diff_cols * 2);
        if antinode.0 < 0 || antinode.0 >= rows || antinode.1 < 0 || antinode.1 >= columns { continue; }

        antinodes_set.insert(antinode);
      }
    }
  }
  
  antinodes_set.len()
}

fn parse_input(input: &String) -> (HashMap<char, LinkedList<(i64, i64)>>, i64, i64) {
  let mut map: HashMap<char, LinkedList<(i64, i64)>> = HashMap::new();
  let mut rows: i64 = 0;
  let mut columns: i64 = 0;

  for (i, line) in input.lines().enumerate() {
    columns = 0;
    for (j, code) in line.chars().enumerate() {
      if code == '.' {
        columns += 1;
        continue;
      }

      let entry = map.entry(code).or_insert(LinkedList::new());
      entry.push_back((i as i64, j as i64));

      columns += 1;
    }
    rows += 1;
  }

  (map, rows, columns)
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
  let (map, rows, columns) = parse_input(&input);
  let result = calculate_antinodes(&map, rows, columns);
  println!("Result: {}", result);
}
