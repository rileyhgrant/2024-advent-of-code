use std::env;

#[path = "./lib.rs"]
mod lib;
mod day1;

fn main() {
    println!("\n===================");
    println!("\nAdvent of Code 2024");

    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    println!("\nRunning day {}", config.day);

    match config.day.as_ref() {
        "1" => {
            println!("\n=== Day 1 ===");
            lib::print_output("1", day1::part_1("day01.txt"));
            lib::print_output("2", day1::part_2("day01.txt"));
        }
        _ => println!("day {} not implemented yet!", config.day),
    }

    struct Config {
        day: String,
    }

    fn parse_config(args: &[String]) -> Config {
        let day = args[1].clone();

        Config { day }
    }
}

