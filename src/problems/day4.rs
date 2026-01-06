use std::fs::File;
use std::io::Read;

pub fn print_solution() {
    let mut file = File::open("./inputs/day4.txt").unwrap();
    let mut input = String::with_capacity(10);
    file.read_to_string(&mut input).unwrap();
    let p1 = part_one(&input);
    let p2 = part_two(&input);
    println!("Day 4 Part 1: {}", p1);
    println!("Day 4 Part 2: {}", p2);
}

fn part_one(input: &str) -> u64 {
    let mut num = 0;
    loop {
        let test_str = input.to_string() + &num.to_string();
        let res = md5::compute(test_str.as_bytes());
        let res_str = format!("{:x}", res);
        if res_str.starts_with("00000") {
            return num;
        }
        num += 1;
    }
}

fn part_two(input: &str) -> u64 {
    let mut num = 0;
    loop {
        let test_str = input.to_string() + &num.to_string();
        let res = md5::compute(test_str.as_bytes());
        let res_str = format!("{:x}", res);
        if res_str.starts_with("000000") {
            return num;
        }
        num += 1;
    }
}