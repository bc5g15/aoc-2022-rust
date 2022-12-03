use std::{env, fs};
mod puzzles;
use puzzles::food;
use puzzles::rps;
use puzzles::reorg;

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
            let part1 = food::find_most_food(&day1);
            let part2 = food::find_top_three_holders(&day1);
            println!("Question 1: {part1:?} {part2:?}");
        },
        2 => {
            let day2 = read_file("in2.txt");
            let part1 = rps::guide_score(&day2);
            let part2 = rps::true_guide_score(&day2);
            println!("Question 2: {part1}, {part2}");
        },
        3 => {
            let day3 = read_file("in3.txt");
            let part1 = reorg::value_shared_priorities(&day3);
            println!("Question 3: {part1}")
        }
        n => {
            println!("No entry for day {n}");
        }
    }
}
