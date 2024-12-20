#[path = "./lib.rs"]
mod lib;

use std::iter::Iterator;

type Num = f64;

fn process_line(line: &String, second_split: &str) -> (Num, Num) {
    let xy: Vec<Num> = line
        .split(":")
        .nth(1)
        .unwrap()
        .split(",")
        .map(|el| el.split(second_split).nth(1).unwrap().parse().unwrap())
        .collect();

    (xy[0], xy[1])
}

fn process_in_groups_of_three(
    items: impl Iterator<Item = String>,
    mut processor: impl FnMut(&[String]) -> Num,
) -> Num {
    let items: Vec<String> = items.collect();
    let mut sum: Num = 0.0;
    for chunk in items.chunks(3) {
        let to_add = processor(chunk);
        sum += to_add;
    }
    sum
}

fn solve_for_token_cost(l: Num, m: Num, n: Num, o: Num, p: Num, q: Num, is_part_2: bool) -> Num {
    let b = (n * o - q * l) / (m * o - p * l);
    let a = (n - m * b) / l;

    let both_positive = a > -1.0 && b > -1.0;
    let both_whole = a % 1.0 == 0.0 && b % 1.0 == 0.0;
    let neither_above_limit = if is_part_2 {
        true
    } else {
        a < 101.0 && b < 101.0
    };
    let all_criteria_passed = both_positive && both_whole && neither_above_limit;

    let sum = if all_criteria_passed {
        a * 3.0 + b
    } else {
        0.0
    };
    sum
}

fn process_group(lines: &[String], is_part_2: bool) -> Num {
    let (l, o) = process_line(&lines[0], "+");
    let (m, p) = process_line(&lines[1], "+");
    let (mut n, mut q) = process_line(&lines[2], "=");

    if is_part_2 {
        let to_add: Num = 10_000_000_000_000.0;
        n += to_add;
        q += to_add;
    }
    solve_for_token_cost(l, m, n, o, p, q, is_part_2)
}

pub fn part_1(path: &str) -> String {
    let contents = lib::read_input(format!("input/{}", path));

    let sum = process_in_groups_of_three(contents.into_iter(), |group| process_group(group, false));

    sum.to_string()
}

pub fn part_2(path: &str) -> String {
    let contents = lib::read_input(format!("input/{}", path));

    let sum = process_in_groups_of_three(contents.into_iter(), |group| process_group(group, true));

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_13_part_1() {
        let test_result = part_1("day13_test.txt");
        assert_eq!(test_result, "480");

        let test_result = part_1("day13.txt");
        assert_eq!(test_result, "25629");
    }

    #[test]
    fn test_day_13_part_2() {
        let test_result = part_2("day13_test.txt");
        assert_eq!(test_result, "875318608908");

        let test_result = part_2("day13.txt");
        assert_eq!(test_result, "107487112929999");
    }
}
