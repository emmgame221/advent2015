use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn print_solution() {
    let file = File::open("./inputs/day8.txt").unwrap();
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    let p1 = part_one(&lines);
    println!("Day 8 Part 1: {}", p1);
    let p2 = part_two(&lines);
    println!("Day 8 Part 2: {}", p2);
}

fn part_one(lines: &[String]) -> usize {
    let mut lit_count = 0;
    let mut resolved_count = 0;
    for line in lines {
        let line = line.as_bytes();
        lit_count += line.len();
        resolved_count += line.len() - 2;
        let mut slice = &line[1..line.len() - 1];
        while let Some(i) = slice.iter().position(|c| *c == b'\\') {
            if slice.len() == 1 {
                break;
            }
            if slice[i + 1] == b'x' {
                resolved_count -= 3;
                slice = &slice[i + 4..];
            } else {
                resolved_count -= 1;
                slice = &slice[i + 2..];
            }
        }
    }
    lit_count - resolved_count
}

fn part_two(lines: &[String]) -> usize {
    let mut original_count = 0;
    let mut new_count = 0;
    for line in lines {
        let line = line.as_bytes();
        original_count += line.len();
        new_count += line.len() + 2;
        let mut slice = &line[0..];
        while let Some(i) = slice.iter().position(|&c| c == b'"' || c == b'\\') {
            new_count += 1;
            slice = &slice[i + 1..];
        }
    }
    new_count - original_count
}
