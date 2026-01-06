use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

pub fn print_solution() {
    let file = File::open("./inputs/day5.txt").unwrap();
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    let p1 = part_one(&lines);
    println!("Day 5 Part 1: {}", p1);
    let p2 = part_two(&lines);
    println!("Day 5 Part 2: {}", p2);
}

fn part_one(lines: &[String]) -> u64 {
    lines
        .iter()
        .fold(0, |acc, s| if is_nice(s) { acc + 1 } else { acc })
}

fn is_nice(s: &str) -> bool {
    !contains_bad_pattern(s) && has_double_letter(s) && has_three_vowels(s)
}

fn has_double_letter(s: &str) -> bool {
    for (c1, c2) in s.chars().tuple_windows() {
        if c1 == c2 {
            return true;
        }
    }
    false
}

fn has_three_vowels(s: &str) -> bool {
    s.chars()
        .fold(0, |acc, c| if is_vowel(c) { acc + 1 } else { acc })
        >= 3
}

fn contains_bad_pattern(s: &str) -> bool {
    s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy")
}

fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

fn part_two(lines: &[String]) -> u64 {
    lines.iter().fold(0, |acc, s| {
        if has_nonoverlapping_double(s) && has_xyx_patt(s) {
            acc + 1
        } else {
            acc
        }
    })
}

fn has_nonoverlapping_double(s: &str) -> bool {
    for i in 0..s.len() - 2 {
        let patt = &s[i..i + 2];
        if s[i + 2..].contains(patt) {
            return true;
        }
    }
    false
}

fn has_xyx_patt(s: &str) -> bool {
    for (x, _y, z) in s.chars().tuple_windows() {
        if x == z {
            return true;
        }
    }
    false
}
