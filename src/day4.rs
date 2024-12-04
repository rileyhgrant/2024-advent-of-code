#[path = "./lib.rs"]
mod lib;



fn shape_input(filepath: &str) -> Vec<Vec<char>> {
    let contents = lib::read_input(format!("input/{}", filepath));

    let line_length = contents.iter().next().unwrap().len();
    let padding = 3;

    let padding_row = vec![vec!['.'; line_length + padding * 2]; padding];
    let padding_cols = vec!['.'; padding];

    let mut grid = padding_row.clone(); 
    contents.iter()
        .for_each(|line| {
            let mut padded_line = padding_cols.clone() ;
            padded_line.extend(line.chars());
            padded_line.extend(padding_cols.clone());
            grid.push(padded_line);
        });
    grid.extend(padding_row);

    grid
}



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
            };
        }
    }

    sum
}



pub fn part_1(path: &str) -> String {
    let grid = shape_input(path);

    let mut sum = 0;
    grid.iter().enumerate()
        .for_each(|(i, row)| {
            row.iter().enumerate()
                .for_each(|(j, _cell)| {
                    if grid[i][j] == 'X' {
                        let xmasses = sum_xmasses(&grid, i, j);
                        sum += xmasses
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
}
