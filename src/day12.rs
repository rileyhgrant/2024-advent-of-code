#[path = "./lib.rs"]
mod lib;

use std::collections::HashMap;

fn recur(
    symb: char,
    pos: (usize, usize),
    grid: &Vec<Vec<char>>,
    seen: &mut HashMap<(usize, usize), bool>,
) -> (u32, u32, u32) {
    let mut area = 0;
    let mut perimeter = 0;
    let mut sides = 0;
    let mut contributed_sides_map: HashMap<(usize, usize), Vec<&str>> = HashMap::new();
    _recur(
        symb,
        pos,
        grid,
        seen,
        &mut area,
        &mut perimeter,
        &mut contributed_sides_map,
        &mut sides,
    );

    (area, perimeter, sides)
}

fn _recur(
    symb: char,
    pos: (usize, usize),
    grid: &Vec<Vec<char>>,
    seen: &mut HashMap<(usize, usize), bool>,
    area: &mut u32,
    perimeter: &mut u32,
    contributed_sides_map: &mut HashMap<(usize, usize), Vec<&str>>,
    sides: &mut u32,
) {
    seen.insert(pos, true);

    let check_vec: Vec<(i32, i32)> = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];
    let recur_vec: Vec<(usize, usize)> = check_vec
        .iter()
        .filter_map(|check| {
            let new_row = (pos.0 as i32 - check.0) as usize;
            let new_col = (pos.1 as i32 - check.1) as usize;
            let to_check = grid[new_row][new_col];

            if to_check == symb {
                Some((new_row, new_col))
            } else {
                None
            }
        })
        .collect();

    let this_perimeter_contribution = 4 - recur_vec.iter().len() as u32;
    *perimeter += this_perimeter_contribution;
    *area += 1;

    if this_perimeter_contribution > 0 {
        let outside_vec: Vec<(usize, usize)> = check_vec
            .iter()
            .filter_map(|check| {
                let new_row = (pos.0 as i32 - check.0) as usize;
                let new_col = (pos.1 as i32 - check.1) as usize;
                let to_check = grid[new_row][new_col];

                if to_check == symb {
                    None
                } else {
                    Some((new_row, new_col))
                }
            })
            .collect();

        let dirs: Vec<&str> = outside_vec
            .iter()
            .map(|outside| {
                let delta: (i32, i32) = (
                    outside.0 as i32 - pos.0 as i32,
                    outside.1 as i32 - pos.1 as i32,
                );

                let dir = match delta {
                    (0, 1) => "Right",
                    (1, 0) => "Down",
                    (0, -1) => "Left",
                    (-1, 0) => "Up",
                    _ => "Something is wrong",
                };

                dir
            })
            .collect();

        *sides += dirs.iter().len() as u32;
        for dir in dirs.iter() {
            for neighbor in recur_vec.iter() {
                if contributed_sides_map.contains_key(neighbor) {
                    let neighbors_sides = contributed_sides_map.get(neighbor).unwrap();
                    if neighbors_sides.contains(dir) {
                        *sides -= 1
                    }
                }
            }
        }
        contributed_sides_map.insert(pos, dirs);
    }

    recur_vec.iter().for_each(|coord| {
        if !seen.contains_key(coord) {
            _recur(
                symb,
                *coord,
                grid,
                seen,
                area,
                perimeter,
                contributed_sides_map,
                sides,
            );
        }
    })
}

pub fn part_1(path: &str) -> String {
    let grid = lib::create_padded_grid(path, '.', 1);
    let rows = grid.iter().len();
    let cols = grid[0].iter().len();

    let mut seen: HashMap<(usize, usize), bool> = HashMap::new();
    let mut sum = 0;
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == '.' {
                continue;
            }
            if seen.contains_key(&(i, j)) {
                continue;
            }

            let (area, perimeter, _sides) = recur(grid[i][j], (i, j), &grid, &mut seen);
            sum += area * perimeter;
        }
    }

    sum.to_string()
}

pub fn part_2(path: &str) -> String {
    let grid = lib::create_padded_grid(path, '.', 1);
    let rows = grid.iter().len();
    let cols = grid[0].iter().len();

    let mut seen: HashMap<(usize, usize), bool> = HashMap::new();
    let mut sum = 0;
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == '.' {
                continue;
            }
            if seen.contains_key(&(i, j)) {
                continue;
            }

            let (area, _perimeter, sides) = recur(grid[i][j], (i, j), &grid, &mut seen);
            sum += area * sides;
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_12_part_1() {
        let test_result = part_1("day12_test.txt");
        assert_eq!(test_result, "1930");

        let test_result = part_1("day12.txt");
        assert_eq!(test_result, "1473408");
    }

    #[test]
    fn test_day_12_part_2() {
        let test_result = part_2("day12_test.txt");
        assert_eq!(test_result, "1206");

        let test_result = part_2("day12.txt");
        assert_eq!(test_result, "886364");
    }
}
