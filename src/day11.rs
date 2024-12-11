#[path = "./lib.rs"]
mod lib;

use std::collections::HashMap;

fn blink(stones: &Vec<u64>) -> Vec<u64> {
    stones
        .iter()
        .flat_map(|&st| {
            let num_digits = ((st as f64).log10().floor()) as u32 + 1;

            if st == 0 {
                vec![1]
            } else if (((st as f64).log10().floor()) as u32 + 1) % 2 == 0 {
                let divisor = 10_u64.pow(num_digits / 2);
                let first_half = st / divisor;
                let second_half = st % divisor;
                vec![first_half, second_half]
            } else {
                vec![st * 2024]
            }
        })
        .collect::<Vec<u64>>()
}

fn count_single_after_blinks(
    stone: u64,
    blinks: u32,
    lookup: &mut HashMap<(u64, u32), u64>,
) -> u64 {
    if lookup.contains_key(&(stone, blinks)) {
        let val = lookup[&(stone, blinks)];
        return val;
    }

    if blinks == 1 {
        let val = blink(&vec![stone]).len();
        lookup.insert((stone, blinks), val as u64);
        return val as u64;
    } else {
        let values = blink(&vec![stone]);
        let val: u64 = values
            .iter()
            .map(|&st| count_single_after_blinks(st, blinks - 1, lookup))
            .sum();
        lookup.insert((stone, blinks), val);
        return val;
    }
}

fn count_after_n_blinks(path: &str, blinks: u32) -> String {
    let contents: Vec<String> = lib::read_input(format!("input/{}", path));
    let stones: Vec<u64> = contents
        .iter()
        .flat_map(|line| line.split(" ").filter_map(|st| st.parse().ok()))
        .collect();

    let mut lookup: HashMap<(u64, u32), u64> = HashMap::new();

    let mut sum = 0;
    for stone in stones {
        let loop_sum = count_single_after_blinks(stone, blinks, &mut lookup);
        sum += loop_sum;
    }
    sum.to_string()
}

pub fn part_1(path: &str) -> String {
    count_after_n_blinks(path, 25)
}

pub fn part_2(path: &str) -> String {
    count_after_n_blinks(path, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_11_part_1() {
        let test_result = count_after_n_blinks("day11_test.txt", 6);
        assert_eq!(test_result, "22");

        let test_result = count_after_n_blinks("day11_test.txt", 25);
        assert_eq!(test_result, "55312");

        let test_result = part_1("day11.txt");
        assert_eq!(test_result, "197157");
    }

    #[test]
    fn test_day_11_part_2() {
        let test_result = part_2("day11.txt");
        assert_eq!(test_result, "234430066982597");
    }
}
