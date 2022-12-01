use std::{env, fs};
mod puzzles;
use puzzles::food;

fn read_file(filename: &str) -> String {
    match fs::read_to_string(filename.to_string()) {
        Ok(v) => v,
        Err(_) => panic!("No filename ${filename} found")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let value = match args.get(1) {
        None => 0,
        Some(n) => n.parse::<u8>().unwrap_or(0)
    };

    match value {
        1 => {
            let day1 = read_file("in1.txt");
            let part1 = food::find_most_food(day1);
            println!("Question 1: {part1:?}");
        },
        n => {
            println!("No entry for day {n}");
        }
    }
}
