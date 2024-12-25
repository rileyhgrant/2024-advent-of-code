#[path = "./lib.rs"]
mod lib;

use std::collections::HashSet;
use std::fs;

type Num = i64;

fn add_to_keys_and_locks(
    grids: Vec<&str>,
    keys: &mut HashSet<Vec<Num>>,
    locks: &mut HashSet<Vec<Num>>,
) {
    grids.iter().for_each(|grid_string| {
        let mut nums: Vec<Num> = vec![-1, -1, -1, -1, -1];
        let grid_vec_string = lib::string_to_vec_string(grid_string.to_string());
        let grid = lib::create_padded_grid_from_vec_string(grid_vec_string, 'x', 0);

        let lock = grid[0].iter().all(|ch| *ch == '#');

        for (i, row) in grid.iter().enumerate() {
            for (j, _col) in row.iter().enumerate() {
                if grid[i][j] == '#' {
                    nums[j] += 1;
                }
            }
        }

        if lock {
            locks.insert(nums.clone());
        } else {
            keys.insert(nums.clone());
        }
    })
}

fn key_and_lock_fit(key: &Vec<Num>, lock: &Vec<Num>) -> bool {
    let mut no_overlap = true;
    let max = 6;
    for i in 0..key.len() {
        if key[i] + lock[i] >= max {
            no_overlap = false;
        }
    }
    no_overlap
}

fn test_keys_with_locks(keys: HashSet<Vec<Num>>, locks: HashSet<Vec<Num>>) -> Num {
    let mut result = 0;

    for key in keys.iter() {
        for lock in locks.iter() {
            if key_and_lock_fit(&key, &lock) {
                result += 1;
            }
        }
    }

    result
}

pub fn part_1(path: &str) -> String {
    let contents =
        fs::read_to_string(format!("input/{}", path)).expect("Should have been able to read file");
    let mut grids: Vec<&str> = contents.split("\n\n").collect();
    grids.pop();

    let mut keys: HashSet<Vec<Num>> = HashSet::new();
    let mut locks: HashSet<Vec<Num>> = HashSet::new();

    add_to_keys_and_locks(grids, &mut keys, &mut locks);

    let result = test_keys_with_locks(keys, locks);
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_25_part_1() {
        let test_result = part_1("day25_test.txt");
        assert_eq!(test_result, "3");

        let test_result = part_1("day25.txt");
        assert_eq!(test_result, "2978");
    }
}
