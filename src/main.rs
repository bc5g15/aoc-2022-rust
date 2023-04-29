use std::{env, fs};
mod puzzles;
use puzzles::cleanup;
use puzzles::food;
use puzzles::rps;
use puzzles::reorg;
use puzzles::stacks;
use puzzles::tuning;
use puzzles::filewalk;
use puzzles::trees;
use puzzles::tail_follow;
use puzzles::crt;
use puzzles::monkeys;
use puzzles::climbing;
use puzzles::compare;
use puzzles::sand;
use puzzles::beacon;
use puzzles::valves;

fn read_file(filename: &str) -> String {
    match fs::read_to_string(filename.to_string()) {
        Ok(v) => v,
        Err(_) => panic!("No filename {filename} found")
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
            let part2 = reorg::badge_groups(&day3);
            println!("Question 3: {part1}, {part2}");
        },
        4 => {
            let day4 = read_file("in4.txt");
            let part1 = cleanup::how_many_containments(&day4);
            let part2 = cleanup::how_many_overlaps(&day4);
            println!("Question 4: {part1}, {part2}");
        },
        5 => {
            let day5 = read_file("in5.txt");
            let part1 = stacks::full_process(&day5);
            let part2 = stacks::full_process_9001(&day5);
            println!("Question 5: {part1}, {part2}");
        },
        6 => {
            let day6 = read_file("in6.txt");
            let part1 = tuning::first_marker(&day6, 4).unwrap();
            let part2 = tuning::first_marker(&day6, 14).unwrap();
            println!("Question 6: {part1}, {part2}");
        },
        7 => {
            let day7 = read_file("in7.txt");
            let part1 = filewalk::biggest_small_dirs(&day7);
            let part2 = filewalk::smallest_big_dir(&day7);
            println!("Question 7: {part1}, {part2}");
        },
        8 => {
            let day8 = read_file("in8.txt");
            let part1 = trees::count_visible_trees(&day8);
            let part2 = trees::best_scenic_score(&day8);
            println!("Question 8: {part1}, {part2}");
        },
        9 => {
            let day9 = read_file("in9.txt");
            let part1 = tail_follow::tail_visited_positions(&day9);
            let part2 = tail_follow::many_tail_visited_positions(&day9);
            println!("Question 9: {part1}, {part2}");
        },
        10 => {
            let day10 = read_file("in10.txt");
            let part1 = crt::sample_at_points(&day10);
            println!("Question 10: {part1}");
            crt::get_whole_image(&day10);
        },
        11 => {
            let day11 = read_file("in11.txt");
            let part1 = monkeys::monkey_business(&day11, false);
            let part2 = monkeys::monkey_business(&day11, true);
            println!("Question 11: {part1}, {part2}")
        },
        12 => {
            let day12 = read_file("in12.txt");
            let part1 = climbing::shortest_path(&day12);
            let part2 = climbing::shortest_from_low(&day12);
            println!("Question 12: {part1}, {part2}");
        },
        13 => {
            let day13 = read_file("in13.txt");
            let part1 = compare::evaluate_sorted(&day13);
            let part2 = compare::distress_position(&day13);
            println!("Question 13: {part1}, {part2}");
        },
        14 => {
            let day14 = read_file("in14.txt");
            let part1 = sand::maximum_static_sand(&day14);
            let part2 = sand::maximum_floor_sand(&day14);
            println!("Question 14: {part1}, {part2}");
        },
        15 => {
            let day15 = read_file("in15.txt");
            let part1 = beacon::retro_part_one(&day15, 2000000);
            let part2 = beacon::find_range_gap(&day15, 4000000);
            println!("Question 15: {part1}, {part2}");
        },
        16 => {
            let day16 = read_file("in16.txt");
            let part1 = valves::part1(&day16);
            let part2 = valves::part2(&day16);
            println!("Question 16: {part1}, {part2}");
        }
        n => {
            println!("No entry for day {n}");
        }
    }
}
