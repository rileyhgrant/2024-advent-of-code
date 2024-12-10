use std::env;

#[path = "./lib.rs"]
mod lib;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

fn main() {
    println!("\n\n===================");
    println!("Advent of Code 2024");

    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);
    let day_string = config.day.clone();

    let current_day = 10;

    let all = config.day.to_lowercase() == "all";
    let days: Vec<String> = if all {
        (1..(current_day + 1)).map(|day| day.to_string()).collect()
    } else {
        vec![config.day]
    };

    for day in days {
        println!("\n=== Day {:02} ===", day.parse::<i32>().unwrap());
        match day.as_str() {
            "1" => {
                lib::print_output("1", || day01::part_1("day01.txt"));
                lib::print_output("2", || day01::part_2("day01.txt"));
            }
            "2" => {
                lib::print_output("1", || day02::part_1("day02.txt"));
                lib::print_output("2", || day02::part_2("day02.txt"));
            }
            "3" => {
                lib::print_output("1", || day03::part_1("day03.txt"));
                lib::print_output("2", || day03::part_2("day03.txt"));
            }
            "4" => {
                lib::print_output("1", || day04::part_1("day04.txt"));
                lib::print_output("2", || day04::part_2("day04.txt"));
            }
            "5" => {
                lib::print_output("1", || day05::part_1("day05.txt"));
                lib::print_output("2", || day05::part_2("day05.txt"));
            }
            "6" => {
                lib::print_output("1", || day06::part_1("day06.txt"));
                lib::print_output("2", || day06::part_2("day06.txt"));
            }
            "7" => {
                lib::print_output("1", || day07::part_1("day07.txt"));
                lib::print_output("2", || day07::part_2("day07.txt"));
            }
            "8" => {
                lib::print_output("1", || day08::part_1("day08.txt"));
                lib::print_output("2", || day08::part_2("day08.txt"));
            }
            "9" => {
                lib::print_output("1", || day09::part_1("day09.txt"));
                lib::print_output("2", || day09::part_2("day09.txt"));
            }
            "10" => {
                lib::print_output("1", || day10::part_1("day10.txt"));
                lib::print_output("2", || day10::part_2("day10.txt"));
            }
            _ => println!(" -- not implemented yet"),
        }
    }

    if config.festive {
        let height = if all {
            current_day
        } else {
            day_string.parse::<usize>().unwrap()
        };
        let festive_height = config.festive_value.unwrap_or(height);
        lib::print_christmas_tree(festive_height);
    }

    struct Config {
        day: String,
        festive: bool,
        festive_value: Option<usize>,
    }

    fn parse_config(args: &[String]) -> Config {
        let day = args[1].clone();
        let mut festive = false;
        let mut festive_value = None;

        let mut i = 2;
        while i < args.len() {
            if args[i] == "--festive" {
                festive = true;
                if i + 1 < args.len() {
                    if let Ok(val) = args[i + 1].parse() {
                        festive_value = Some(val);
                        i += 1;
                    }
                }
            }
            i += 1;
        }

        Config {
            day,
            festive,
            festive_value,
        }
    }
}
