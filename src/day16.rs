#[path = "./lib.rs"]
mod lib;

type Num = i32;
type Position = (usize, usize);

use std::collections::{HashMap, HashSet};

fn find_char(grid: &Vec<Vec<char>>, to_find: char) -> (usize, usize) {
    for (i, row) in grid.iter().enumerate() {
        for (j, _col) in row.iter().enumerate() {
            if grid[i][j] == to_find {
                return (i, j);
            }
        }
    }
    (0, 0)
}

fn safe_add(pos: (usize, usize), delta: &(Num, Num)) -> (usize, usize) {
    let next_x = ((pos.0 as Num) + delta.0) as usize;
    let next_y = ((pos.1 as Num) + delta.1) as usize;
    (next_x, next_y)
}

fn get_possible_directions(dir: char) -> Vec<char> {
    match dir {
        '^' => vec!['^', '<', '>'],
        'v' => vec!['v', '<', '>'],
        '<' => vec!['^', 'v', '<'],
        '>' => vec!['^', 'v', '>'],
        _ => vec![],
    }
}

fn get_next_position(pos: Position, dir: char) -> Position {
    let delta = match dir {
        '^' => (-1, 0),
        'v' => (1, 0),
        '<' => (0, -1),
        '>' => (0, 1),
        _ => (0, 0),
    };

    safe_add(pos, &delta)
}

fn is_valid_move(maze: &Vec<Vec<char>>, pos: Position) -> bool {
    maze[pos.0][pos.1] != '#'
}

fn dfs(maze: Vec<Vec<char>>) -> (Num, usize) {
    let mut stack = Vec::new();
    let mut costs: HashMap<((usize, usize), char), i32> = HashMap::new();
    let mut best_cost = Num::MAX;

    let mut cost_and_path: HashMap<Num, Vec<Vec<Position>>> = HashMap::new();

    let start_pos = find_char(&maze, 'S');

    let init_path = vec![start_pos];
    stack.push((start_pos, '>', 0, init_path));

    while let Some(current) = stack.pop() {
        if current.2 > best_cost {
            continue;
        }

        if let Some(&prev_cost) = costs.get(&(current.0, current.1)) {
            // TODO: this is awful
            if current.2 >= prev_cost + 1000 {
                continue;
            }
        }

        costs.insert((current.0, current.1), current.2);

        if maze[current.0 .0][current.0 .1] == 'E' {
            best_cost = best_cost.min(current.2);
            cost_and_path
                .entry(current.2)
                .or_insert_with(Vec::new)
                .push(current.3.clone());
            continue;
        }

        for next_dir in get_possible_directions(current.1) {
            let next_pos = get_next_position(current.0, next_dir);
            if is_valid_move(&maze, next_pos) {
                let move_cost = if next_dir == current.1 { 1 } else { 1001 };

                let mut new_path = current.3.clone();
                new_path.push(next_pos);
                stack.push((next_pos, next_dir, current.2 + move_cost, new_path));
            }
        }
    }

    let best_paths = cost_and_path.get(&best_cost).unwrap();

    let best_squares = best_paths
        .iter()
        .flat_map(|v| v.iter())
        .map(|&pos| pos)
        .collect::<HashSet<(usize, usize)>>();

    (best_cost, best_squares.len())
}

pub fn part_1(path: &str) -> String {
    let grid = lib::create_grid(path);
    let tupl = dfs(grid);

    tupl.0.to_string()
}

pub fn part_2(path: &str) -> String {
    let grid = lib::create_grid(path);
    let tupl = dfs(grid);
    tupl.1.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_16_part_1() {
        let test_result = part_1("day16_test.txt");
        assert_eq!(test_result, "7036");

        let test_result = part_1("day16_test2.txt");
        assert_eq!(test_result, "11048");

        let test_result = part_1("day16.txt");
        assert_eq!(test_result, "89460");
    }

    #[test]
    fn test_day_16_part_2() {
        let test_result = part_2("day16_test.txt");
        assert_eq!(test_result, "45");

        let test_result = part_2("day16_test2.txt");
        assert_eq!(test_result, "64");

        let test_result = part_2("day16.txt");
        assert_eq!(test_result, "504");
    }
}
