#[path = "./lib.rs"]
mod lib;

use std::collections::HashMap;

fn generate_new_symbol(symbols: &Vec<i32>) -> i32 {
    let prev_symbol = symbols[symbols.len() - 1];
    prev_symbol + 1
}

pub fn part_1(path: &str) -> String {
    let contents = &mut lib::read_input(format!("input/{}", path))[0];
    let input: Vec<u32> = contents.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let mut symbols: Vec<i32> = vec![0];
    let mut symbols_map: HashMap<i32, u32> = HashMap::from([(-1, 0)]);
    let mut transformed_input: Vec<i32> = Vec::new();
    let mut symbol_value = 0;

    for (i, &num) in input.iter().enumerate() {
        let symbol = if i % 2 != 0 {
            -1
        } else {
            let temp = generate_new_symbol(&symbols);
            symbols.push(temp);
            symbols_map.insert(temp, symbol_value);
            symbol_value += 1;
            temp
        };

        for _ in 0..num {
            transformed_input.push(symbol.clone());
        }
    }

    let mut j = transformed_input.len() - 1;
    let mut clone = transformed_input.clone();

    let dot = -1;
    let total_dots = transformed_input.iter().filter(|&sym| *sym == dot).count();
    let final_j = transformed_input.len() - total_dots;

    for (i, _) in transformed_input.iter().enumerate() {
        if clone[i] == dot {
            clone.swap(i, j);
            j -= 1;
            if j < final_j {
                break;
            }
        }

        while clone[j] == dot {
            j -= 1;
        }
    }

    let as_nums: Vec<u64> = clone
        .iter()
        .map(|sym| *symbols_map.get(sym).unwrap() as u64)
        .collect();

    let sum: u64 = as_nums
        .iter()
        .enumerate()
        .map(|(i, num)| i as u64 * num)
        .sum();

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_9_part_1() {
        let test_result = part_1("day09_test.txt");
        assert_eq!(test_result, "1928");

        let test_result = part_1("day09.txt");
        assert_eq!(test_result, "6241633730082");
    }
}
