use std::env;

fn main() {
    println!("\n===================");
    println!("\nAdvent of Code 2024");

    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    println!("\nRunning day {}", config.day);

    match config.day.as_ref() {
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

