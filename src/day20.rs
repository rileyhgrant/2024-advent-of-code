#[path = "./lib.rs"]
mod lib;

use std::collections::{HashMap, VecDeque};

type Num = i64;
type Pos = (usize, usize);
type Maze = Vec<Vec<char>>;

fn find_char(grid: &Vec<Vec<char>>, to_find: char) -> Pos {
    for (i, row) in grid.iter().enumerate() {
        for (j, _col) in row.iter().enumerate() {
            if grid[i][j] == to_find {
                return (i, j);
            }
        }
    }
    (0, 0)
}

fn safe_add(pos: Pos, delta: (Num, Num)) -> Pos {
    let new_x = (pos.0 as Num + delta.0) as usize;
    let new_y = (pos.1 as Num + delta.1) as usize;
    (new_x, new_y)
}

fn get_valid_moves(maze: &Maze, pos: &Pos, seen: &HashMap<Pos, Pos>) -> Vec<Pos> {
    let deltas: Vec<(Num, Num)> = vec![(0, 1), (0, -1), (-1, 0), (1, 0)];

    deltas
        .iter()
        .filter_map(|delta| {
            let test_pos = safe_add(*pos, *delta);
            let symbol = maze[test_pos.0][test_pos.1];
            if (symbol == '.' || symbol == 'E') && !seen.contains_key(&test_pos) {
                return Some(test_pos);
            } else {
                return None;
            }
        })
        .collect::<Vec<Pos>>()
}

fn bfs(
    maze: &Maze,
    start: Pos,
    memo: &mut HashMap<Pos, Num>,
    start_num: Num,
    max_time: Num,
) -> Num {
    let mut parents: HashMap<Pos, Pos> = HashMap::new();
    let mut queue: VecDeque<(Pos, Num)> = VecDeque::new();
    let mut best_path: Vec<Pos> = Vec::new();
    queue.push_front((start, start_num));

    while let Some(value) = queue.pop_back() {
        let curr_pos = value.0;
        let curr_time = value.1;

        if curr_time > max_time {
            continue;
        }

        if let Some(value) = memo.get(&curr_pos) {
            let memo_time = value;
            return memo_time + curr_time;
        }

        if maze[curr_pos.0][curr_pos.1] == 'E' {
            let mut rebuilt_path: Vec<Pos> = Vec::new();
            let mut current = curr_pos;
            while let Some(parent) = parents.get(&current) {
                rebuilt_path.push(current);
                current = *parent;
            }
            rebuilt_path.reverse();
            best_path = rebuilt_path;
            break;
        }
        let valid_moves = get_valid_moves(&maze, &curr_pos, &parents);
        for valid_move in valid_moves {
            parents.insert(valid_move, curr_pos);
            queue.push_front((valid_move, curr_time + 1));
        }
    }

    let total_len = best_path.len();
    memo.insert(start, total_len as Num);
    for i in 0..total_len {
        let curr_pos = best_path[i];
        memo.insert(curr_pos, total_len as Num - i as Num - 1);
    }

    best_path.len() as Num
}

fn get_valid_cheats(maze: &Maze, pos: &Pos, seen_cheats: &HashMap<(Pos, Pos), Num>) -> Vec<Pos> {
    let deltas: Vec<(Num, Num)> = vec![(0, 2), (0, -2), (-2, 0), (2, 0)];

    deltas
        .iter()
        .filter_map(|delta| {
            let test_pos = safe_add(*pos, *delta);
            let symbol = maze[test_pos.0][test_pos.1];
            if (symbol == '.' || symbol == 'E') && !seen_cheats.contains_key(&(*pos, test_pos)) {
                return Some(test_pos);
            } else {
                return None;
            }
        })
        .collect::<Vec<Pos>>()
}
fn determine_all_cheats_that_finish_before_cutoff(
    maze: &Maze,
    start: Pos,
    cutoff: Num,
    time_memo: &mut HashMap<Pos, Num>,
    cheat_memo: &mut HashMap<(Pos, Pos), Num>,
) -> Num {
    let mut queue: VecDeque<(Pos, Num)> = VecDeque::new();
    let mut parents: HashMap<Pos, Pos> = HashMap::new();
    queue.push_front((start, 0));

    while let Some(value) = queue.pop_back() {
        let curr_pos = value.0;
        let curr_time = value.1;

        let valid_cheats = get_valid_cheats(&maze, &curr_pos, &cheat_memo);
        for valid_cheat in valid_cheats {
            let cheat_total_time = bfs(&maze, valid_cheat, time_memo, curr_time + 2, cutoff);
            cheat_memo.insert((curr_pos, valid_cheat), cheat_total_time);
        }

        let valid_moves = get_valid_moves(&maze, &curr_pos, &parents);
        for valid_move in valid_moves {
            parents.insert(valid_move, curr_pos);
            queue.push_front((valid_move, curr_time + 1));
        }
    }

    let filtered_cheats: Vec<((Pos, Pos), Num)> = cheat_memo
        .iter()
        // TODO: i'm don't understand when a cheat would give me 0
        //   but filtering them out seems to work for now?
        .filter(|(_, &value)| value <= cutoff && value != 0)
        .map(|(&key, &value)| (key, value))
        .collect();

    filtered_cheats.len() as Num
}

fn solve_part_1(path: &str, min_ms_saved: Num) -> String {
    let grid = lib::create_padded_grid(path, 'o', 1);

    let mut time_memo: HashMap<Pos, Num> = HashMap::new();

    let start = find_char(&grid, 'S');
    let initial_time = bfs(&grid, start, &mut time_memo, 0, Num::MAX);

    let mut cheat_memo: HashMap<(Pos, Pos), Num> = HashMap::new();
    let cutoff_time = initial_time - min_ms_saved;
    let num_valid_cheats = determine_all_cheats_that_finish_before_cutoff(
        &grid,
        start,
        cutoff_time,
        &mut time_memo,
        &mut cheat_memo,
    );

    num_valid_cheats.to_string()
}

pub fn part_1(path: &str) -> String {
    solve_part_1(path, 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_20_part_1() {
        let test_result = solve_part_1("day20_test.txt", 20);
        assert_eq!(test_result, "5");

        let test_result = part_1("day20.txt");
        assert_eq!(test_result, "1296");
    }
}
