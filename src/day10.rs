#[path = "./lib.rs"]
mod lib;

fn recur(symb: i32, pos: (usize, usize), grid: &Vec<Vec<char>>, part_1: bool) -> i32 {
    let mut seen_nines: Vec<(usize, usize)> = Vec::new();
    _recur(symb, pos, grid, &mut seen_nines, part_1)
}

fn _recur(
    symb: i32,
    pos: (usize, usize),
    grid: &Vec<Vec<char>>,
    seen_nines: &mut Vec<(usize, usize)>,
    part_1: bool,
) -> i32 {
    let check_vec = vec![(0, -1), (-1, 0), (1, 0), (0, 1)];

    let mut sum = 0;

    for to_check in check_vec {
        let new_row: usize = (pos.0 as i32 - to_check.0) as usize;
        let new_col: usize = (pos.1 as i32 - to_check.1) as usize;
        let symbol_to_check = grid[new_row][new_col];
        if symbol_to_check == '.' {
            sum += 0;
        } else if symb == 8 && symbol_to_check == '9' && !seen_nines.contains(&(new_row, new_col)) {
            seen_nines.push((new_row, new_col));
            sum += 1;
        } else {
            let num_to_check = symbol_to_check.to_digit(10).unwrap() as i32;
            if num_to_check == symb + 1 {
                if part_1 {
                    sum += _recur(num_to_check, (new_row, new_col), &grid, seen_nines, part_1);
                } else {
                    sum += recur(num_to_check, (new_row, new_col), &grid, part_1);
                }
            }
        }
    }

    sum
}

pub fn part_1(path: &str) -> String {
    let grid = lib::create_padded_grid(path, '.', 1);

    let size = grid.iter().count();

    let mut sum = 0;
    for i in 0..size {
        for j in 0..size {
            if grid[i][j] == '0' {
                let this_sum = recur(grid[i][j].to_digit(10).unwrap() as i32, (i, j), &grid, true);
                sum += this_sum;
            }
        }
    }

    sum.to_string()
}

pub fn part_2(path: &str) -> String {
    let grid = lib::create_padded_grid(path, '.', 1);

    let size = grid.iter().count();

    let mut sum = 0;
    for i in 0..size {
        for j in 0..size {
            if grid[i][j] == '0' {
                let this_sum = recur(
                    grid[i][j].to_digit(10).unwrap() as i32,
                    (i, j),
                    &grid,
                    false,
                );
                sum += this_sum;
            }
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_10_part_1() {
        let test_result = part_1("day10_test.txt");
        assert_eq!(test_result, "36");

        let test_result = part_1("day10.txt");
        assert_eq!(test_result, "459");
    }

    #[test]
    fn test_day_10_part_2() {
        let test_result = part_2("day10_test.txt");
        assert_eq!(test_result, "81");

        let test_result = part_2("day10.txt");
        assert_eq!(test_result, "1034");
    }
}
