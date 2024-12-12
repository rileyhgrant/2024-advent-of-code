#[path = "./lib.rs"]
mod lib;

use std::collections::HashMap;

fn recur(
    symb: char,
    pos: (usize, usize),
    grid: &Vec<Vec<char>>,
    seen: &mut HashMap<(usize, usize), bool>,
) -> (u32, u32) {
    let mut area = 0;
    let mut perimeter = 0;
    _recur(symb, pos, grid, seen, &mut area, &mut perimeter);
    (area, perimeter)
}

fn _recur(
    symb: char,
    pos: (usize, usize),
    grid: &Vec<Vec<char>>,
    seen: &mut HashMap<(usize, usize), bool>,
    area: &mut u32,
    perimeter: &mut u32,
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

    *perimeter += 4 - recur_vec.iter().len() as u32;
    *area += 1;

    recur_vec.iter().for_each(|coord| {
        if !seen.contains_key(coord) {
            _recur(symb, *coord, grid, seen, area, perimeter);
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

            let (area, perimeter) = recur(grid[i][j], (i, j), &grid, &mut seen);
            sum += area * perimeter;
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
}
