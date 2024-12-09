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
    let mut symbols: Vec<i32> = vec![-1];
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

fn find_indices(target: i32, to_check: &Vec<i32>) -> Vec<(usize, usize)> {
    let mut windows_vec: Vec<(usize, usize)> = Vec::new();

    let mut in_window = false;
    let mut curr_start: i32 = -1;
    let mut curr_stop: i32 = -1;

    for (i, &test) in to_check.iter().enumerate() {
        if test == target {
            if !in_window {
                in_window = true;
                curr_start = i as i32;
                curr_stop = i as i32;
            } else {
                curr_stop = i as i32;
            }

            if i == to_check.len() - 1 {
                if curr_start != -1 && curr_stop != -1 {
                    windows_vec.push((curr_start as usize, curr_stop as usize));
                }
            }
        } else {
            if curr_start != -1 && curr_stop != -1 {
                windows_vec.push((curr_start as usize, curr_stop as usize));
            }
            curr_start = -1;
            curr_stop = -1;
            in_window = false;
        }
    }

    windows_vec
}

pub fn part_2(path: &str) -> String {
    let contents = &mut lib::read_input(format!("input/{}", path))[0];
    let input: Vec<u32> = contents.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let mut symbols: Vec<i32> = vec![-1];
    let mut symbols_map: HashMap<i32, u32> = HashMap::from([(-1, 0)]);
    let mut transformed_input: Vec<i32> = Vec::new();
    let mut symbol_value = 0;

    let mut symbol_posns: Vec<(i32, usize, usize)> = Vec::new();

    let mut idx = 0;
    for (i, &num) in input.iter().enumerate() {
        let symbol = if i % 2 != 0 {
            idx += num as usize;
            -1
        } else {
            let temp = generate_new_symbol(&symbols);
            symbols.push(temp);
            symbols_map.insert(temp, symbol_value);
            symbol_value += 1;

            symbol_posns.push((temp, idx, idx + num as usize - 1));
            idx += num as usize;

            temp
        };

        for _ in 0..num {
            transformed_input.push(symbol.clone());
        }
    }

    let mut clone = transformed_input.clone();
    let mut reversed_symbols = symbols.clone();
    reversed_symbols.remove(0);
    reversed_symbols.reverse();

    let mut blank_indices = find_indices(-1, &clone);

    for symbol in reversed_symbols.iter() {
        let symbol_indices = symbol_posns
            .iter()
            .find(|el| el.0 == *symbol)
            .map(|el| (el.1, el.2))
            .unwrap();

        let symbol_start_idx = symbol_indices.0;
        let symbol_end_idx = symbol_indices.1;
        let len_needed = symbol_end_idx - symbol_start_idx + 1;
        for i in 0..blank_indices.len() - 1 {
            let blank = &blank_indices[i];

            let len_available = blank.1 - blank.0 + 1;
            let comes_before = blank.0 < symbol_start_idx;

            if !comes_before {
                break;
            }

            if len_available >= len_needed {
                let mut blank_start = blank.0;
                let mut mem_start = symbol_indices.0;
                for _ in 0..len_needed {
                    clone.swap(blank_start, mem_start);
                    blank_start += 1;
                    mem_start += 1;
                }

                let new_blank_start = blank_start;
                let new_blank_stop = blank.1;

                if new_blank_start > new_blank_stop {
                    blank_indices.remove(i);
                } else {
                    blank_indices[i] = (new_blank_start, new_blank_stop);
                }
                break;
            }
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

    #[test]
    fn test_day_9_part_2() {
        let test_result = part_2("day09_test.txt");
        assert_eq!(test_result, "2858");

        let test_result = part_2("day09.txt");
        assert_eq!(test_result, "6265268809555");
    }
}
