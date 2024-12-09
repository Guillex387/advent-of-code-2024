use std::{env,fs};
use std::collections::LinkedList;

#[derive(Debug, Clone)]
enum DiskSection {
  Free,
  FileBlock(usize),
}

fn checksum(map: &LinkedList<DiskSection>) -> usize {
  map
    .iter()
    .enumerate()
    .map(|(i, section)| match section {
      DiskSection::Free => 0,
      DiskSection::FileBlock(id) => id.clone() * i,
    })
    .sum()
}

fn to_dense(mut map: LinkedList<DiskSection>) -> LinkedList<DiskSection> {
  let mut result: LinkedList<DiskSection> = LinkedList::new();

  loop {
    let element = match map.pop_front() {
      Some(v) => v,
      None => break
    };

    match element {
      DiskSection::FileBlock(id) => result.push_back(DiskSection::FileBlock(id)),
      DiskSection::Free => {
        loop {
          let element = match map.pop_back() {
            Some(v) => v,
            None => break
          };
          match element {
            DiskSection::FileBlock(id) => result.push_back(DiskSection::FileBlock(id)),
            DiskSection::Free => continue,
          }
          break;
        }
      }
    }
  }

  result
}

fn parse_input(input: &String) -> LinkedList<DiskSection> {
  let mut result: LinkedList<DiskSection> = LinkedList::new();
  let mut free_space =  false;
  let mut file_id = 0;
  for c in input.chars() {
    if c.is_control() { continue; }
    let len = c.to_digit(10).unwrap() as usize;
    if free_space {
      (0..len).for_each(|_| result.push_back(DiskSection::Free));
    } else {
      (0..len).for_each(|_| result.push_back(DiskSection::FileBlock(file_id)));
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
  // println!("Formatted: {} {:?}\n", disk_map.len(), disk_map);
  // println!("Checksum: {}", checksum(&disk_map));
  let dense_map = to_dense(disk_map);
  // println!("Dense: {} {:?}", dense_map.len(), dense_map);
  println!("Checksum: {}", checksum(&dense_map));
}
