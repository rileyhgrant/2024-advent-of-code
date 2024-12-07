#[path = "./lib.rs"]
mod lib;
use regex::Regex;

fn compute_sum(line: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(line)
        .filter_map(
            |cap| match (cap[1].parse::<i32>().ok(), cap[2].parse::<i32>().ok()) {
                (Some(n1), Some(n2)) => Some(n1 * n2),
                _ => None,
            },
        )
        .sum::<i32>()
}

pub fn part_1(path: &str) -> String {
    let contents = lib::read_input(format!("input/{}", path));

    let sum: i32 = contents.iter().map(|line| compute_sum(line)).sum();

    sum.to_string()
}

pub fn part_2(path: &str) -> String {
    let contents = lib::read_input(format!("input/{}", path));

    let mut dont = false;
    let sum: i32 = contents
        .iter()
        .map(|line| {
            let split_on_dont: Vec<&str> = line.split("don't()").collect();

            split_on_dont
                .iter()
                .enumerate()
                .map(|(i, val)| {
                    let split_on_do: Vec<&str> = val.split("do()").collect::<Vec<&str>>();
                    let result = if i == 0 && !dont {
                        val.to_string()
                    } else if split_on_do.len() == 1 {
                        if i == split_on_dont.len() - 1 {
                            dont = true;
                        };
                        "".to_string()
                    } else {
                        if i == split_on_dont.len() - 1 {
                            dont = false;
                        };
                        split_on_do[1..].join("")
                    };

                    compute_sum(&result)
                })
                .sum::<i32>()
        })
        .sum();

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_3_part_1() {
        let test_result = part_1("day03_test.txt");
        assert_eq!(test_result, "161");

        let test_result = part_1("day03.txt");
        assert_eq!(test_result, "155955228");
    }

    #[test]
    fn test_day_3_part_2() {
        let test_result = part_2("day03_test2.txt");
        assert_eq!(test_result, "48");

        let test_result = part_2("day03.txt");
        assert_eq!(test_result, "100189366");
    }
}
