use clap::{command, Arg};

mod problems;

use crate::problems::*;

fn main() {
    let matches = command!() 
        .arg(Arg::new("problems").short('p').long("problems"))
        .get_matches();
    let args = matches
        .get_many::<String>("problems")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();
    if args.contains(&"all") {
        run_all();
    } else {
        for arg in args {
            match arg {
                "1" => {
                    day1::print_solution();
                }
                "2" => {
                    day2::print_solution();
                }
                _ => {
                    println!("Unknown Problem")
                }
            }
        }
    }
}

fn run_all() {
    day1::print_solution();
    day2::print_solution();
}
