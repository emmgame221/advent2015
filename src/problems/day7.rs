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
    let (p1, p2) = solution(&lines);
    println!("Day 7 Part 1: {}", p1);
    println!("Day 7 Part 2: {}", p2);
}

fn solution(lines: &[String]) -> (u16, u16) {
    let mut wires: HashMap<String, Option<u16>> = HashMap::new();
    let mut p1 = 0;
    loop {
        if let Some(&Some(val)) = wires.get("a") {
            if p1 == 0 {
                p1 = val;
                let keys: Vec<String> = wires.keys().map(|key| key.clone()).collect();
                for key in keys {
                    wires.insert(key, None);
                }
                wires.insert("b".to_string(), Some(val));
            } else {
                return (p1, val);
            }
        }
        for line in lines {
            let parts: Vec<&str> = line.split(' ').collect();
            if parts[0] == "NOT" {
                assert_eq!(parts.len(), 4);
                assert_eq!(parts[2], "->");
                let input = parts[1];
                let output = parts[3];
                if let Some(&Some(in_val)) = wires.get(input) {
                    wires.insert(output.to_string(), Some(!in_val));
                } else {
                    wires.insert(input.to_string(), None);
                    wires.insert(output.to_string(), None);
                }
            } else if parts[1] == "->" {
                assert_eq!(parts.len(), 3);
                let input = parts[0];
                let output = parts[2];
                if let Ok(val) = input.parse::<u16>() {
                    let output = parts[2];
                    wires.insert(output.to_string(), Some(val));
                } else if let Some(&Some(in_val)) = wires.get(input) {
                    wires.insert(output.to_string(), Some(in_val));
                } else {
                    wires.insert(input.to_string(), None);
                    wires.insert(output.to_string(), None);
                }
            } else {
                assert_eq!(parts.len(), 5);
                assert_eq!(parts[3], "->");
                let in1 = parts[0];
                let op = parts[1];
                let in2 = parts[2];
                let output = parts[4];
                let in1_val = if let Ok(val) = in1.parse::<u16>() {
                    Some(val)
                } else if let Some(&Some(val)) = wires.get(in1) {
                    Some(val)
                } else {
                    wires.insert(in1.to_string(), None);
                    None
                };
                let in2_val = if let Ok(val) = in2.parse::<u16>() {
                    Some(val)
                } else if let Some(&Some(val)) = wires.get(in2) {
                    Some(val)
                } else {
                    wires.insert(in2.to_string(), None);
                    None
                };
                let out_val = {
                    if let Some(in1_val) = in1_val
                        && let Some(in2_val) = in2_val
                    {
                        match op {
                            "AND" => Some(in1_val & in2_val),
                            "OR" => Some(in1_val | in2_val),
                            "LSHIFT" => Some(in1_val << in2_val),
                            "RSHIFT" => Some(in1_val >> in2_val),
                            _ => panic!(),
                        }
                    } else {
                        None
                    }
                };
                wires.insert(output.to_string(), out_val);
            }
        }
    }
}
