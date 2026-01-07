use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn print_solution() {
    let file = File::open("./inputs/day7.txt").unwrap();
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    let p1 = solution(&lines);
    println!("Day 7 Part 1: {}", p1);
}

fn solution(lines: &[String]) -> u16 {
    let mut wires: HashMap<&str, u16> = HashMap::new();
    loop {
        if let Some(&val) = wires.get("a") {
            return val;
        }
        for line in lines {
            let parts: Vec<&str> = line.split(' ').collect();
            if parts[0] == "NOT" {
                assert_eq!(parts.len(), 4);
                assert_eq!(parts[2], "->");
                let input = parts[1];
                let output = parts[3];
                if let Some(&in_val) = wires.get(input) {
                    wires.insert(output, !in_val);
                } else {
                    continue;
                }
            } else if parts[1] == "->" {
                assert_eq!(parts.len(), 3);
                let input = parts[0];
                let output = parts[2];
                if let Ok(val) = input.parse::<u16>() {
                    let output = parts[2];
                    wires.insert(output, val);
                } else if let Some(&in_val) = wires.get(input) {
                    wires.insert(output, in_val);
                } else {
                    continue;
                }
            } else {
                assert_eq!(parts.len(), 5);
                assert_eq!(parts[3], "->");
                let in1 = parts[0];
                let op = parts[1];
                let in2 = parts[2];
                let output = parts[4];
                let in1_val = if let Ok(val) = in1.parse::<u16>() {
                    val
                } else if let Some(&val) = wires.get(in1) {
                    val
                } else {
                    continue;
                };
                let in2_val = if let Ok(val) = in2.parse::<u16>() {
                    val
                } else if let Some(&val) = wires.get(in2) {
                    val
                } else {
                    continue;
                };
                let out_val = match op {
                    "AND" => in1_val & in2_val,
                    "OR" => in1_val | in2_val,
                    "LSHIFT" => in1_val << in2_val,
                    "RSHIFT" => in1_val >> in2_val,
                    _ => panic!(),
                };
                wires.insert(output, out_val);
            }
        }
    }
}
