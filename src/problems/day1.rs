use std::{fs::File, io::Read};

pub fn print_solution() {
    let mut file = File::open("./inputs/day1.txt").unwrap();
    let mut input: String = String::with_capacity(8000);
    file.read_to_string(&mut input).unwrap();
    let p1 = part_one(&input);
    let p2 = part_two(&input);
    println!("Day 1 Part 1: {}", p1);
    println!("Day 1 Part 2: {}", p2);
}

fn part_one(input: &str) -> i64 {
    let mut floor = 0;
    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        }
    }
    floor
}

fn part_two(input: &str) -> usize {
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        }
        if floor == -1 {
            return i + 1;
        }
    }
    0
}
