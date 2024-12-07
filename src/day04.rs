#[path = "./lib.rs"]
mod lib;

fn sum_xmasses(grid: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    let check = [-1, 0, 1];
    let chars = ['M', 'A', 'S'];

    let mut sum = 0;
    for &row_offset in check.iter() {
        for &col_offset in check.iter() {
            for (i, &ch) in chars.iter().enumerate() {
                let r = (row as i32 + (row_offset * (i as i32 + 1))) as usize;
                let c = (col as i32 + (col_offset * (i as i32 + 1))) as usize;

                if grid[r][c] != ch {
                    break;
                }
                if ch == 'S' {
                    sum += 1;
                }
            }
        }
    }

    sum
}

pub fn part_1(path: &str) -> String {
    let grid = lib::create_padded_grid(path, '.', 3);

    let mut sum = 0;
    grid.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, _cell)| {
            if grid[i][j] == 'X' {
                let xmasses = sum_xmasses(&grid, i, j);
                sum += xmasses
            }
        })
    });

    sum.to_string()
}

fn sum_x_masses(grid: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    let check = [[[(-1, -1), (1, 1)], [(-1, 1), (1, -1)]]];

    // TODO: All these mut counters is pretty un idiomatic Rust
    //   clean this up later in a more functional way, if desired
    let mut sum = 0;
    for coord_pairs in check.iter() {
        let mut sum_mas = 0;
        for coord_pair in coord_pairs.iter() {
            let mut sum_m = 0;
            let mut sum_s = 0;
            for &coord in coord_pair {
                let r = (row as i32 + coord.0 as i32) as usize;
                let c = (col as i32 + coord.1 as i32) as usize;
                if grid[r][c] == 'S' {
                    sum_s += 1;
                } else if grid[r][c] == 'M' {
                    sum_m += 1;
                }
            }
            if sum_m == 1 && sum_s == 1 {
                sum_mas += 1;
            }
        }
        if sum_mas == 2 {
            sum += 1
        }
    }

    sum
}

pub fn part_2(path: &str) -> String {
    let grid = lib::create_padded_grid(path, '.', 3);

    let mut sum = 0;
    grid.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, _cell)| {
            if grid[i][j] == 'A' {
                let x_masses = sum_x_masses(&grid, i, j);
                sum += x_masses
            }
        })
    });

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_4_part_1() {
        let test_result = part_1("day04_test.txt");
        assert_eq!(test_result, "18");

        let test_result = part_1("day04.txt");
        assert_eq!(test_result, "2567");
    }

    #[test]
    fn test_day_4_part_2() {
        let test_result = part_2("day04_test.txt");
        assert_eq!(test_result, "9");

        let test_result = part_2("day04.txt");
        assert_eq!(test_result, "2029");
    }
}
