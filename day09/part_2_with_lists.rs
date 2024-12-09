use std::{env,fs, usize};
use std::collections::LinkedList;

#[derive(Debug, Clone, PartialEq, Eq)]
enum DiskSection {
  Free(usize),
  FileBlock(usize, usize),
}

fn checksum(map: &LinkedList<DiskSection>) -> usize {
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

fn merge_free_spaces(map: &mut LinkedList<DiskSection>) {
  let mut result: LinkedList<DiskSection> = LinkedList::new();
  let mut merge_size = 0usize;
  for section in map.iter() {
    match section {
      DiskSection::Free(s) => merge_size += s,
      section => {
        if merge_size > 0 {
          result.push_back(DiskSection::Free(merge_size));
          merge_size = 0;
        }
        result.push_back(section.clone());
      }
    }
  }

  if merge_size > 0 {
    result.push_back(DiskSection::Free(merge_size));
  }
  *map = result;
}

fn move_front(file_id: usize, file_size: usize, map: &mut LinkedList<DiskSection>) -> bool {
  let free_space_finded = map.iter().enumerate().find(|(_, s)| match s { DiskSection::Free(size) => size.ge(&file_size), _ => false });
  if free_space_finded.is_none() { return false; }
  let (free_index, free_section) = free_space_finded.unwrap();
  let free_space_size = match free_section {
    DiskSection::Free(s) => s.clone(),
    _ => 0,
  };

  // Replace free section with the file
  let mut sector1 = map.split_off(free_index);
  sector1.pop_front();
  map.push_back(DiskSection::FileBlock(file_id, file_size));
  map.push_back(DiskSection::Free(free_space_size - file_size));
  map.append(&mut sector1);

  let finded = map.iter().enumerate().rev().find(|(_, s)| match s { DiskSection::FileBlock(id, _) => id.eq(&file_id), _ => false });
  let (file_index, _) = finded.unwrap();

  // Replace file section with a free section
  let mut sector2 = map.split_off(file_index);
  sector2.pop_front();
  map.push_back(DiskSection::Free(file_size));
  map.append(&mut sector2);

  merge_free_spaces(map);

  true
}

fn defragment(map: LinkedList<DiskSection>) -> LinkedList<DiskSection> {
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

fn parse_input(input: &String) -> LinkedList<DiskSection> {
  let mut result: LinkedList<DiskSection> = LinkedList::new();
  let mut free_space =  false;
  let mut file_id = 0;
  for c in input.chars() {
    if c.is_control() { continue; }
    let len = c.to_digit(10).unwrap() as usize;
    if free_space && len > 0 {
      result.push_back(DiskSection::Free(len));
    } else if len > 0 {
      result.push_back(DiskSection::FileBlock(file_id, len));
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
