#[path = "./lib.rs"]
mod lib;

use std::collections::HashMap;
use std::collections::HashSet;

fn find_antinodes(
    pos_less: &(usize, usize),
    pos_more: (usize, usize),
    upper_bound: i32,
) -> Vec<(i32, i32)> {
    let mut to_return: Vec<(i32, i32)> = Vec::new();

    let delta: (i32, i32) = (
        (pos_more.0 as i32 - pos_less.0 as i32),
        (pos_more.1 as i32 - pos_less.1 as i32),
    );

    let big_antinode: (i32, i32) = (pos_more.0 as i32 + delta.0, pos_more.1 as i32 + delta.1);
    let little_antinode: (i32, i32) = (pos_less.0 as i32 - delta.0, pos_less.1 as i32 - delta.1);

    if big_antinode.0 < upper_bound && big_antinode.1 < upper_bound && big_antinode.1 > -1 {
        to_return.push(big_antinode);
    }

    if little_antinode.0 > -1 && little_antinode.1 > -1 && little_antinode.1 < upper_bound {
        to_return.push(little_antinode);
    }

    to_return
}

fn find_harmonic_antinodes(
    pos_less: &(usize, usize),
    pos_more: (usize, usize),
    upper_bound: i32,
) -> Vec<(i32, i32)> {
    let mut antinodes: Vec<(i32, i32)> = Vec::new();

    let delta: (i32, i32) = (
        (pos_more.0 as i32 - pos_less.0 as i32),
        (pos_more.1 as i32 - pos_less.1 as i32),
    );

    antinodes.push((pos_less.0 as i32, pos_less.1 as i32));
    antinodes.push((pos_more.0 as i32, pos_more.1 as i32));

    let mut pos_decrease = (pos_less.0 as i32, pos_less.1 as i32);
    loop {
        if !(pos_decrease.0 > -1 && pos_decrease.1 > -1 && pos_decrease.1 < upper_bound) {
            break;
        }
        antinodes.push(pos_decrease.clone());
        pos_decrease = (pos_decrease.0 - delta.0, pos_decrease.1 - delta.1);
    }

    let mut pos_increase = (pos_more.0 as i32, pos_more.1 as i32);
    loop {
        if !(pos_increase.0 < upper_bound && pos_increase.1 < upper_bound && pos_increase.1 > -1) {
            break;
        }
        antinodes.push(pos_increase.clone());
        pos_increase = (pos_increase.0 + delta.0, pos_increase.1 + delta.1);
    }

    antinodes
}

fn solve_problem(
    path: &str,
    func: fn(&(usize, usize), (usize, usize), i32) -> Vec<(i32, i32)>,
) -> String {
    let grid = lib::create_grid(path);

    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    let upper_bound = grid.iter().count();

    // if I run into borrow checking problems, I can just do (0..upper_bound)
    for (i, row) in grid.iter().enumerate() {
        for (j, _col) in row.iter().enumerate() {
            let symbol = grid[i][j];
            if symbol != '.' {
                if !antennas.contains_key(&symbol) {
                    antennas.insert(symbol, vec![(i, j)]);
                } else {
                    let seen_antennas = antennas.get(&symbol).unwrap();

                    for seen_antenna in seen_antennas {
                        let created_antinodes = func(seen_antenna, (i, j), upper_bound as i32);
                        antinodes.extend(created_antinodes);
                    }

                    antennas
                        .entry(symbol)
                        .and_modify(|value| value.push((i, j)));
                }
            }
        }
    }

    antinodes.len().to_string()
}

pub fn part_1(path: &str) -> String {
    solve_problem(path, find_antinodes)
}

pub fn part_2(path: &str) -> String {
    solve_problem(path, find_harmonic_antinodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_8_part_1() {
        let test_result = part_1("day08_test.txt");
        assert_eq!(test_result, "14");

        let test_result = part_1("day08.txt");
        assert_eq!(test_result, "413");
    }

    #[test]
    fn test_day_8_part_2() {
        let test_result = part_2("day08_test.txt");
        assert_eq!(test_result, "34");

        let test_result = part_2("day08.txt");
        assert_eq!(test_result, "1417");
    }
}
