use std::{collections::LinkedList, env, fs, usize};

fn rotate90(angle: &mut (i64, i64)) {
  *angle = match angle {
    (-1, 0) => (0, 1),
    (0, 1) => (1, 0),
    (1, 0) => (0, -1),
    (0, -1) => (-1, 0),
    &mut other => other.clone()
  };
}

fn advance(pos: &mut (usize, usize), angle: &(i64, i64)) {
  *pos = ((angle.0 + pos.0 as i64) as usize, (angle.1 + pos.1 as i64) as usize);
}

fn is_looped(matrix: &Vec<Vec<bool>>, mut pos: (usize, usize)) -> bool {
  let mut angle = (-1i64, 0i64);
  let mut lists_positions: LinkedList<((usize, usize), (i64, i64))> = LinkedList::new();
  
  loop {
    if lists_positions.contains(&(pos, angle)) {
      return true;
    }

    let mut front_pos = pos.clone();
    advance(&mut front_pos, &angle);

    if front_pos.0 >= matrix.len() || front_pos.1 >= matrix[0].len() {
      return false;
    }

    if matrix[front_pos.0][front_pos.1] {
      rotate90(&mut angle);
      continue;
    }

    lists_positions.push_back((pos, angle));
    advance(&mut pos, &angle);
  }
}

fn calculate_possible_loops(matrix: &mut Vec<Vec<bool>>, pos: (usize, usize)) -> usize {
  let mut loops = 0usize;
  let mut checked = 0usize;
  let amount = matrix.len() * matrix[0].len();
  for i in 0..matrix.len() {
    for j in 0..matrix[0].len() {
      if (i, j) == pos || matrix[i][j] { continue; }

      matrix[i][j] = true;
      if is_looped(matrix, pos) {
        loops += 1;
      }
      matrix[i][j] = false;
      checked += 1;
      println!("Checked {}/{} loops = {}", checked, amount, loops);
    }
  }

  loops
}

fn parse_input(input: &String) -> (Vec<Vec<bool>>, (usize, usize)) {
  let mut pos: (usize, usize) = (0, 0);
  (input
    .lines()
    .enumerate()
    .map(|(i, line)|
      line.chars().enumerate().map(|(j, c)| {
        if c == '^' {
          pos = (i, j);
        }

        c == '#'
      }).collect()
    )
    .collect(),
   pos)
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
  let (mut matrix, pos) = parse_input(&input);
  let result = calculate_possible_loops(&mut matrix, pos);

  println!("Result: {}", result);
}
