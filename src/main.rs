use std::env;

#[path = "./lib.rs"]
mod lib;
mod day1;
mod day2;
mod day3;

fn main() {
    println!("\n\n===================");
    println!("Advent of Code 2024");

    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    let current_day = 3;
    
    let days: Vec<String> = if config.day.to_lowercase() == "all" {
        (1..(current_day + 1)).map(|day| day.to_string()).collect()
    } else {
        vec![config.day]
    };

    for day in days {
        println!("\n=== Day {:02}===", day);
        match day.as_str() {
            "1" => {
                lib::print_output("1", day1::part_1("day01.txt"));
                lib::print_output("2", day1::part_2("day01.txt"));
            }
            "2" => {
                lib::print_output("1", day2::part_1("day02.txt"));
                lib::print_output("2", day2::part_2("day02.txt"));
            }
            "3" => {
                lib::print_output("1", day3::part_1("day03.txt"));
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

