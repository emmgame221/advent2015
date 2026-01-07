use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn print_solution() {
    let file = File::open("./inputs/day6.txt").unwrap();
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    let p1 = part_one(&lines);
    println!("Day 6 Part 1: {}", p1);
    let p2 = part_two(&lines);
    println!("Day 6 Part 2: {}", p2);
}

fn part_one(lines: &[String]) -> usize {
    let mut grid = vec![vec![false; 1000]; 1000];
    for line in lines {
        if line.starts_with("turn on ") {
            let rest = line.trim_start_matches("turn on ");
            let (start, end) = rest.split_once(" through ").unwrap();
            let (start_x, start_y) = start.split_once(',').unwrap();
            let start_x: usize = start_x.parse().unwrap();
            let start_y: usize = start_y.parse().unwrap();
            let (end_x, end_y) = end.split_once(',').unwrap();
            let end_x: usize = end_x.parse().unwrap();
            let end_y: usize = end_y.parse().unwrap();
            for i in start_y..=end_y {
                for j in start_x..=end_x {
                    grid[i][j] = true;
                }
            }
        } else if line.starts_with("turn off ") {
            let rest = line.trim_start_matches("turn off ");
            let (start, end) = rest.split_once(" through ").unwrap();
            let (start_x, start_y) = start.split_once(',').unwrap();
            let start_x: usize = start_x.parse().unwrap();
            let start_y: usize = start_y.parse().unwrap();
            let (end_x, end_y) = end.split_once(',').unwrap();
            let end_x: usize = end_x.parse().unwrap();
            let end_y: usize = end_y.parse().unwrap();
            for i in start_y..=end_y {
                for j in start_x..=end_x {
                    grid[i][j] = false;
                }
            }
        } else if line.starts_with("toggle ") {
            let rest = line.trim_start_matches("toggle ");
            let (start, end) = rest.split_once(" through ").unwrap();
            let (start_x, start_y) = start.split_once(',').unwrap();
            let start_x: usize = start_x.parse().unwrap();
            let start_y: usize = start_y.parse().unwrap();
            let (end_x, end_y) = end.split_once(',').unwrap();
            let end_x: usize = end_x.parse().unwrap();
            let end_y: usize = end_y.parse().unwrap();
            for i in start_y..=end_y {
                for j in start_x..=end_x {
                    grid[i][j] = !grid[i][j];
                }
            }
        }
    }
    grid.iter()
        .fold(0, |acc, row| acc + row.iter().filter(|x| **x).count())
}

fn part_two(lines: &[String]) -> usize {
    let mut grid: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];
    for line in lines {
        if line.starts_with("turn on ") {
            let rest = line.trim_start_matches("turn on ");
            let (start, end) = rest.split_once(" through ").unwrap();
            let (start_x, start_y) = start.split_once(',').unwrap();
            let start_x: usize = start_x.parse().unwrap();
            let start_y: usize = start_y.parse().unwrap();
            let (end_x, end_y) = end.split_once(',').unwrap();
            let end_x: usize = end_x.parse().unwrap();
            let end_y: usize = end_y.parse().unwrap();
            for i in start_y..=end_y {
                for j in start_x..=end_x {
                    grid[i][j] += 1;
                }
            }
        } else if line.starts_with("turn off ") {
            let rest = line.trim_start_matches("turn off ");
            let (start, end) = rest.split_once(" through ").unwrap();
            let (start_x, start_y) = start.split_once(',').unwrap();
            let start_x: usize = start_x.parse().unwrap();
            let start_y: usize = start_y.parse().unwrap();
            let (end_x, end_y) = end.split_once(',').unwrap();
            let end_x: usize = end_x.parse().unwrap();
            let end_y: usize = end_y.parse().unwrap();
            for i in start_y..=end_y {
                for j in start_x..=end_x {
                    grid[i][j] = match grid[i][j] {
                        0 => 0,
                        val => val - 1
                    }
                }
            }
        } else if line.starts_with("toggle ") {
            let rest = line.trim_start_matches("toggle ");
            let (start, end) = rest.split_once(" through ").unwrap();
            let (start_x, start_y) = start.split_once(',').unwrap();
            let start_x: usize = start_x.parse().unwrap();
            let start_y: usize = start_y.parse().unwrap();
            let (end_x, end_y) = end.split_once(',').unwrap();
            let end_x: usize = end_x.parse().unwrap();
            let end_y: usize = end_y.parse().unwrap();
            for i in start_y..=end_y {
                for j in start_x..=end_x {
                    grid[i][j] += 2;
                }
            }
        }
    }
    grid.iter()
        .fold(0, |acc, row| acc + row.iter().sum::<usize>())
}