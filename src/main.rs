use std::env;
mod puzzles;

fn main() {
    let args: Vec<String> = env::args().collect();

    let value = match args.get(1) {
        None => 0,
        Some(n) => n.parse::<u8>().unwrap_or(0)
    };

    match value {
        n => {
            println!("No entry for day {n}");
        }
    }
}
