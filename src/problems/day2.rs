use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn print_solution() {
    let file = File::open("./inputs/day2.txt").unwrap();
    let lines: Vec<String> = BufReader::new(file).lines().map(|f| f.unwrap()).collect();
    let p1 = part_one(&lines);
    let p2 = part_two(&lines);
    println!("Day 2 Part 1: {}", p1);
    println!("Day 2 Part 2: {}", p2);
}

struct Dims {
    length: u64,
    width: u64,
    height: u64,
    smallest_side_area: u64,
}

impl Dims {
    fn from_string(s: &str) -> Dims {
        let parts: Vec<u64> = s.split('x').map(|x| x.parse().unwrap()).collect();
        let length = parts[0];
        let width = parts[1];
        let height = parts[2];
        let a1 = length * width;
        let a2 = length * height;
        let a3 = width * height;
        Dims {
            length,
            width,
            height,
            smallest_side_area: a1.min(a2).min(a3),
        }
    }
}

fn part_one(lines: &[String]) -> u64 {
    lines
        .iter()
        .map(|line| {
            let dims = Dims::from_string(line);
            surface_area(&dims) + dims.smallest_side_area
        })
        .sum()
}

fn part_two(lines: &[String]) -> u64 {
    lines
        .iter()
        .map(|line| {
            let dims = Dims::from_string(line);
            volume(&dims) + smallest_perimeter(&dims)
        })
        .sum()
}

fn surface_area(dims: &Dims) -> u64 {
    2 * dims.length * dims.width + 2 * dims.width * dims.height + 2 * dims.height * dims.length
}

fn volume(dims: &Dims) -> u64 {
    dims.length * dims.width * dims.height
}

fn smallest_perimeter(dims: &Dims) -> u64 {
    let p1 = 2 * dims.length + 2 * dims.width;
    let p2 = 2 * dims.length + 2 * dims.height;
    let p3 = 2 * dims.width + 2 * dims.height;
    p1.min(p2).min(p3)
}
