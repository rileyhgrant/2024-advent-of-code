use std::env;

#[path = "./lib.rs"]
mod lib;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn main() {
    println!("\n\n===================");
    println!("Advent of Code 2024");

    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    let current_day = 6;
    
    let days: Vec<String> = if config.day.to_lowercase() == "all" {
        (1..(current_day + 1)).map(|day| day.to_string()).collect()
    } else {
        vec![config.day]
    };

    for day in days {
        println!("\n=== Day {:02} ===", day.parse::<i32>().unwrap());
        match day.as_str() {
            "1" => {
                lib::print_output("1", day01::part_1("day01.txt"));
                lib::print_output("2", day01::part_2("day01.txt"));
            }
            "2" => {
                lib::print_output("1", day02::part_1("day02.txt"));
                lib::print_output("2", day02::part_2("day02.txt"));
            }
            "3" => {
                lib::print_output("1", day03::part_1("day03.txt"));
                lib::print_output("2", day03::part_2("day03.txt"));
            }
            "4" => {
                lib::print_output("1", day04::part_1("day04.txt"));
                lib::print_output("2", day04::part_2("day04.txt"));
            }
            "5" => {
                lib::print_output("1", day05::part_1("day05.txt"));
                lib::print_output("2", day05::part_2("day05.txt"));
            }
            "6" => {
                lib::print_output("1", day06::part_1("day06.txt"));
                lib::print_output("2", day06::part_2("day06.txt"));
            }
            _ => println!(" -- not implemented yet"),
        }
    }

    struct Config {
        day: String,
    }

    fn parse_config(args: &[String]) -> Config {
        let day = args[1].clone();

        Config { day }
    }
}

