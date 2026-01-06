use std::collections::HashSet;
use std::{fs::File, io::Read};

pub fn print_solution() {
    let mut file = File::open("./inputs/day3.txt").unwrap();
    let mut input = String::with_capacity(9000);
    file.read_to_string(&mut input).unwrap();
    let p1 = part_one(&input);
    let p2 = part_two(&input);
    println!("Day 3 Part 1: {}", p1);
    println!("Day 3 Part 2: {}", p2);
}

fn part_one(input: &str) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    visited.insert((x, y));
    for c in input.chars() {
        match c {
            '^' => y += 1,
            'v' => y -= 1,
            '<' => x -= 1,
            '>' => x += 1,
            _ => {}
        }
        visited.insert((x, y));
    }
    visited.len()
}

fn part_two(input: &str) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));
    let (mut santa_x, mut santa_y) = (0, 0);
    let (mut robo_x, mut robo_y) = (0, 0);
    for (i, c) in input.chars().enumerate() {
        if i % 2 == 0 {
            match c {
                '^' => santa_y += 1,
                'v' => santa_y -= 1,
                '<' => santa_x -= 1,
                '>' => santa_x += 1,
                _ => {}
            }
            visited.insert((santa_x, santa_y));
        } else {
            match c {
                '^' => robo_y += 1,
                'v' => robo_y -= 1,
                '<' => robo_x -= 1,
                '>' => robo_x += 1,
                _ => {}
            }
            visited.insert((robo_x, robo_y));
        }
    }
    visited.len()
}
