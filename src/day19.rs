#[path = "./lib.rs"]
mod lib;

use std::{collections::HashMap, fs};

type Num = u64;

fn parse_towel_options(towel_string: &str) -> Vec<String> {
    let towels_vec: Vec<String> = lib::string_to_vec_string(towel_string.to_string())
        .iter()
        .flat_map(|line| line.split(",").map(|towel| towel.trim().to_string()))
        .collect();

    towels_vec
}

fn parse_target_towels(target_towels: &str) -> Vec<String> {
    let towels_vec: Vec<String> = lib::string_to_vec_string(target_towels.to_string())
        .iter()
        .map(|line| line.to_string())
        .collect();

    towels_vec
}

fn pre_process_towels(towel_options: &Vec<String>) -> HashMap<u8, Vec<(usize, &[u8])>> {
    let mut starters: HashMap<u8, Vec<(usize, &[u8])>> = HashMap::new();

    for (idx, opt) in towel_options.iter().enumerate() {
        let bytes = opt.as_bytes();
        let first_char = bytes[0];
        starters
            .entry(first_char)
            .or_insert(Vec::new())
            .push((idx, bytes));
    }

    starters
}

fn determine_if_target_towel_is_makable(
    towel_options: &Vec<String>,
    target_towel: String,
    starter_map: &HashMap<u8, Vec<(usize, &[u8])>>,
    memo: &mut HashMap<Vec<u8>, Num>,
) -> Num {
    let target_bytes = target_towel.as_bytes();
    let option_bytes: Vec<&[u8]> = towel_options.iter().map(|opt| opt.as_bytes()).collect();

    let result = _backtrack(&option_bytes, target_bytes, 0, &starter_map, memo);

    result
}

fn _backtrack(
    towel_options: &Vec<&[u8]>,
    target_bytes: &[u8],
    idx: usize,
    starter_map: &HashMap<u8, Vec<(usize, &[u8])>>,
    memo: &mut HashMap<Vec<u8>, Num>,
) -> Num {
    let remaining = &target_bytes[idx..];

    if remaining.is_empty() {
        return 1;
    }

    if let Some(&count) = memo.get(remaining) {
        return count;
    }

    let mut result = 0;

    let first_char = target_bytes[idx];
    if let Some(starter_options) = starter_map.get(&first_char) {
        for &(_, opt) in starter_options {
            if remaining.starts_with(opt) {
                if opt.len() == remaining.len() {
                    result += 1;
                } else if opt.len() <= remaining.len() {
                    result += _backtrack(
                        towel_options,
                        target_bytes,
                        idx + opt.len(),
                        starter_map,
                        memo,
                    )
                }
            }
        }
    }

    memo.insert(remaining.to_vec(), result);

    result
}

fn solve_part_1(towel_options: Vec<String>, target_towels: Vec<String>) -> Num {
    let starter_map = pre_process_towels(&towel_options);
    let mut memo: HashMap<Vec<u8>, Num> = HashMap::new();

    let count = target_towels
        .iter()
        .filter(|target_towel| {
            let result = determine_if_target_towel_is_makable(
                &towel_options,
                target_towel.to_string(),
                &starter_map,
                &mut memo,
            );
            result != 0
        })
        .count();

    count as Num
}

pub fn part_1(path: &str) -> String {
    let contents = fs::read_to_string(format!("input/{}", path))
        .expect("Should have been able to read the file");
    let (towel_string, target_string) = contents.split_once("\n\n").unwrap();

    let towel_options = parse_towel_options(towel_string);
    let target_towels = parse_target_towels(target_string);
    let result = solve_part_1(towel_options, target_towels);

    result.to_string()
}

fn solve_part_2(towel_options: Vec<String>, target_towels: Vec<String>) -> Num {
    let starter_map = pre_process_towels(&towel_options);
    let mut memo: HashMap<Vec<u8>, Num> = HashMap::new();
    target_towels
        .iter()
        .map(|target_towel| {
            determine_if_target_towel_is_makable(
                &towel_options,
                target_towel.to_string(),
                &starter_map,
                &mut memo,
            )
        })
        .sum::<Num>()
}

pub fn part_2(path: &str) -> String {
    let contents = fs::read_to_string(format!("input/{}", path))
        .expect("Should have been able to read the file");
    let (towel_string, target_string) = contents.split_once("\n\n").unwrap();

    let towel_options = parse_towel_options(towel_string);
    let target_towels = parse_target_towels(target_string);
    let result = solve_part_2(towel_options, target_towels);

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_19_part_1() {
        let test_result = part_1("day19_test.txt");
        assert_eq!(test_result, "6");

        let test_result = part_1("day19.txt");
        assert_eq!(test_result, "336");
    }

    #[test]
    fn test_day_19_part_2() {
        let test_result = part_2("day19_test.txt");
        assert_eq!(test_result, "16");

        let test_result = part_2("day19.txt");
        assert_eq!(test_result, "758890600222015");
    }
}
