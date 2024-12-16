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

fn robot_attempt_move_2(pos: &mut (usize, usize), grid: &mut Vec<Vec<char>>, direction: char) {
    let delta: (Num, Num) = match direction {
        '^' => (-1, 0),
        'v' => (1, 0),
        '<' => (0, -1),
        '>' => (0, 1),
        _ => (0, 0),
    };

    let mut i = 0;
    let mut to_move: Vec<(usize, usize)> = Vec::new();

    if direction == '<' || direction == '>' {
        loop {
            let new_x: usize = ((pos.0 as Num + delta.0) + (delta.0 * i)) as usize;
            let new_y: usize = ((pos.1 as Num + delta.1) + (delta.1 * i)) as usize;
            let to_check = (new_x, new_y);
            let symb = grid[new_x][new_y];

            if symb == '#' {
                break;
            } else if symb == '[' || symb == ']' {
                let first_y = (new_y as Num + delta.1) as usize;
                let paired_y = (new_y as Num + delta.1 + delta.1) as usize;
                to_move.push((new_x, first_y));
                to_move.push((new_x, paired_y));
                i += 2;
            } else if symb == '.' {
                grid[pos.0][pos.1] = '.';
                let new_robot_y: usize = (pos.1 as Num + delta.1) as usize;
                grid[to_check.0][new_robot_y] = '@';
                *pos = (to_check.0, new_robot_y);
                if direction == '<' {
                    to_move.reverse();
                }
                to_move.iter().enumerate().for_each(|(i, (bx, by))| {
                    let symb = if i % 2 == 0 { '[' } else { ']' };
                    grid[*bx][*by] = symb;
                });
                break;
            }
        }
    } else {
        loop {
            let new_x: usize = ((pos.0 as Num + delta.0) + (delta.0 * i)) as usize;
            let new_y: usize = ((pos.1 as Num + delta.1) + (delta.1 * i)) as usize;
            let to_check = (new_x, new_y);
            let symb = grid[new_x][new_y];

            if symb == '#' {
                break;
            } else if symb == '.' {
                grid[pos.0][pos.1] = '.';
                let new_robot_x: usize = (pos.0 as Num + delta.0) as usize;
                grid[new_robot_x][to_check.1] = '@';
                *pos = (new_robot_x, to_check.1);
                break;
            } else if symb == '[' || symb == ']' {
                let result = can_move_boxes(to_check, &grid, direction);

                match result {
                    Some(boxes_to_move) => {
                        move_boxes(grid, direction, boxes_to_move);
                        grid[pos.0][pos.1] = '.';
                        let new_robot_x: usize = (pos.0 as Num + delta.0) as usize;
                        grid[new_robot_x][to_check.1] = '@';
                        *pos = (new_robot_x, to_check.1);
                        break;
                    }
                    None => (),
                }

                break;
            }
        }
    }
}

fn move_boxes(grid: &mut Vec<Vec<char>>, direction: char, boxes_to_move: Vec<(usize, usize)>) {
    let delta = if direction == '^' { -1 } else { 1 };

    let new_symbols_and_posns: Vec<(usize, usize, char)> = boxes_to_move
        .iter()
        .map(|b| {
            let symb = grid[b.0][b.1];
            let new_x = ((b.0 as Num) + delta) as usize;
            let new_y = b.1;
            (new_x, new_y, symb)
        })
        .collect();

    boxes_to_move.iter().for_each(|b| {
        if grid[b.0][b.1] != '@' {
            grid[b.0][b.1] = '.';
        }
    });

    new_symbols_and_posns.iter().for_each(|snp| {
        grid[snp.0][snp.1] = snp.2;
    });
}

fn can_move_boxes(
    pos: (usize, usize),
    grid: &Vec<Vec<char>>,
    direction: char,
) -> Option<Vec<(usize, usize)>> {
    let mut can_move_boxes = true;
    let mut boxes_found: Vec<(usize, usize)> = Vec::new();

    _can_move_boxes(pos, grid, direction, &mut can_move_boxes, &mut boxes_found);

    if can_move_boxes {
        Some(boxes_found)
    } else {
        None
    }
}

fn _can_move_boxes(
    pos: (usize, usize),
    grid: &Vec<Vec<char>>,
    direction: char,
    can_move_boxes: &mut bool,
    boxes_found: &mut Vec<(usize, usize)>,
) {
    let delta = if direction == '^' { -1 } else { 1 };

    let this_half = grid[pos.0][pos.1];
    let that_half_y: usize = if this_half == '[' {
        (pos.1 as Num + 1) as usize
    } else {
        (pos.1 as Num - 1) as usize
    };

    let new_x = ((pos.0 as Num) + delta) as usize;
    let above_this_half = grid[new_x][pos.1];
    let above_that_half = grid[new_x][that_half_y];

    if above_this_half == '#' || above_that_half == '#' {
        *can_move_boxes = false;
    } else {
        if above_this_half == '[' || above_this_half == ']' {
            _can_move_boxes((new_x, pos.1), grid, direction, can_move_boxes, boxes_found);
        }
        if above_that_half == '[' || above_that_half == ']' {
            _can_move_boxes(
                (new_x, that_half_y),
                grid,
                direction,
                can_move_boxes,
                boxes_found,
            );
        }

        boxes_found.push((pos.0, that_half_y));
        boxes_found.push((pos.0, pos.1));
    }
}

pub fn part_2(path: &str) -> String {
    let contents = fs::read_to_string(format!("input/{}", path))
        .expect("Should have been able to read the file");
    let (grid_string, directions_string) = contents.split_once("\n\n").unwrap();

    let initial_grid = lib::create_padded_grid_from_vec_string(
        lib::string_to_vec_string(grid_string.to_string()),
        '.',
        0,
    );

    let mut grid: Vec<Vec<char>> = initial_grid
        .iter()
        .map(|row| {
            row.iter()
                .flat_map(|col| match col {
                    '#' => vec!['#', '#'],
                    'O' => vec!['[', ']'],
                    '.' => vec!['.', '.'],
                    '@' => vec!['@', '.'],
                    _ => vec![],
                })
                .collect()
        })
        .collect();

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
        .for_each(|direction| robot_attempt_move_2(&mut robot_pos, &mut grid, *direction));

    let mut sum = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let symb = grid[i][j];
            if symb == '[' {
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

    #[test]
    fn test_day_15_part_2() {
        let test_result = part_2("day15_test2.txt");
        assert_eq!(test_result, "9021");

        let test_result = part_2("day15.txt");
        assert_eq!(test_result, "1429299");
    }
}
