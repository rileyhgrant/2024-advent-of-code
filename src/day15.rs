#[path = "./lib.rs"]
mod lib;

use std::fs;

type Num = i32;

fn robot_attempt_move(pos: &mut (usize, usize), grid: &mut Vec<Vec<char>>, direction: char) {
    let delta: (Num, Num) = match direction {
        '^' => (-1, 0),
        'v' => (1, 0),
        '<' => (0, -1),
        '>' => (0, 1),
        _ => (0, 0),
    };

    let mut i = 0;
    let mut to_move: Vec<(usize, usize)> = Vec::new();
    loop {
        let new_x: usize = ((pos.0 as Num + delta.0) + (delta.0 * i)) as usize;
        let new_y: usize = ((pos.1 as Num + delta.1) + (delta.1 * i)) as usize;
        let to_check = (new_x, new_y);

        let symb = grid[new_x][new_y];

        if symb == '#' {
            break;
        } else if symb == 'O' {
            to_move.push(to_check);
        } else if symb == '.' {
            grid[pos.0][pos.1] = '.';
            if to_move.iter().len() > 0 {
                let first_box = (to_move[0].0, to_move[0].1);
                grid[first_box.0][first_box.1] = '@';
                *pos = (first_box.0, first_box.1);
                grid[to_check.0][to_check.1] = 'O';
            } else {
                grid[to_check.0][to_check.1] = '@';
                *pos = (to_check.0, to_check.1);
            }
            break;
        }
        i += 1;
    }
}

pub fn part_1(path: &str) -> String {
    let contents = fs::read_to_string(format!("input/{}", path))
        .expect("Should have been able to read the file");
    let (grid_string, directions_string) = contents.split_once("\n\n").unwrap();

    let mut grid = lib::create_padded_grid_from_vec_string(
        lib::string_to_vec_string(grid_string.to_string()),
        '.',
        0,
    );

    let mut robot_pos = (0, 0);
    grid.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, _col)| {
            if grid[i][j] == '@' {
                robot_pos = (i, j);
            }
        })
    });

    let directions: Vec<char> = lib::string_to_vec_string(directions_string.to_string())
        .iter()
        .flat_map(|line| line.chars().map(|ch| ch))
        .collect();

    directions
        .iter()
        .for_each(|direction| robot_attempt_move(&mut robot_pos, &mut grid, *direction));

    let mut sum = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'O' {
                sum += i * 100 + j;
            }
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_15_part_1() {
        let test_result = part_1("day15_test1.txt");
        assert_eq!(test_result, "2028");

        let test_result = part_1("day15_test2.txt");
        assert_eq!(test_result, "10092");

        let test_result = part_1("day15.txt");
        assert_eq!(test_result, "1412971");
    }
}
