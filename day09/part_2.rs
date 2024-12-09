use std::{env,fs, usize};

#[derive(Debug, Clone, PartialEq, Eq)]
enum DiskSection {
  Free(usize),
  FileBlock(usize, usize),
}

fn checksum(map: &Vec<DiskSection>) -> usize {
  let mut i = 0usize;
  map
    .iter()
    .map(|section| match section {
      DiskSection::Free(size) => {
        i += size;
        0
      },
      DiskSection::FileBlock(id, size) => {
        (0..*size).map(|_| {
          let res = id * i;
          i += 1;
          res
        }).sum()
      },
    })
    .sum()
}

fn merge_free_spaces(map: &mut Vec<DiskSection>) {
  let mut result: Vec<DiskSection> = Vec::new();
  result.reserve(map.len());

  let mut merge_size = 0usize;
  for section in map.iter() {
    match section {
      DiskSection::Free(s) => merge_size += s,
      section => {
        if merge_size > 0 {
          result.push(DiskSection::Free(merge_size));
          merge_size = 0;
        }
        result.push(section.clone());
      }
    }
  }

  if merge_size > 0 {
    result.push(DiskSection::Free(merge_size));
  }

  *map = result;
}

fn move_front(file_id: usize, file_size: usize, map: &mut Vec<DiskSection>) -> bool {
  // Search the free space
  let free_space_finded = map.iter().position(|s| match s { DiskSection::Free(size) => size.ge(&file_size), _ => false });
  let file_finded = map.iter().position(|s| match s { DiskSection::FileBlock(id, _) => id.eq(&file_id), _ => false });
  if free_space_finded.is_none() || file_finded.is_none() { return false; }

  // Check if the free space is before the file
  let free_index = free_space_finded.unwrap();
  let file_index = file_finded.unwrap();
  if free_index > file_index { return false; }

  let free_space_size = match map[free_index] {
    DiskSection::Free(s) => s.clone(),
    _ => 0,
  };

  // Swap the elements
  map[free_index] = map[file_index].clone();
  map[file_index] = DiskSection::Free(file_size);
  map.insert(free_index+1, DiskSection::Free(free_space_size - file_size));

  merge_free_spaces(map);

  true
}

fn defragment(map: Vec<DiskSection>) -> Vec<DiskSection> {
  let mut result = map.clone();
  map
    .iter()
    .filter(|s| match s { DiskSection::Free(_) => false, DiskSection::FileBlock(_, _) => true})
    .map(|s| match s { DiskSection::FileBlock(id, s) => (id.clone(), s.clone()), _ => (0, 0) })
    .rev()
    .for_each(|(id, size)| {
      move_front(id, size, &mut result);
    });

  result
}

fn parse_input(input: &String) -> Vec<DiskSection> {
  let mut result: Vec<DiskSection> = Vec::new();
  result.reserve(input.chars().count());

  let mut free_space =  false;
  let mut file_id = 0;
  for c in input.chars() {
    if c.is_control() { continue; }
    let len = c.to_digit(10).unwrap() as usize;
    if free_space && len > 0 {
      result.push(DiskSection::Free(len));
    } else if len > 0 {
      result.push(DiskSection::FileBlock(file_id, len));
    }

    if !free_space {
      file_id += 1;
    }
    free_space = !free_space;
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
  let disk_map = parse_input(&input);
  let defragment_disk = defragment(disk_map);
  let result = checksum(&defragment_disk);
  println!("Result: {}", result);
}
