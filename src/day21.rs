#[path = "./lib.rs"]
mod lib;

type Num = i64;
type Pos = (usize, usize);
type Grid = Vec<Vec<char>>;

use std::collections::{HashSet, VecDeque};

fn find_char(grid: &Grid, to_find: char) -> Pos {
    let mut found_pos = (0, 0);

    for (i, row) in grid.iter().enumerate() {
        for (j, _col) in row.iter().enumerate() {
            if grid[i][j] == to_find {
                found_pos = (i, j);
            }
        }
    }

    found_pos
}

fn get_valid_moves(grid: &Grid, pos: &Pos, visited: &HashSet<Pos>) -> Vec<Pos> {
    let deltas = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];

    let width: Num = grid[0].len() as Num;
    let height: Num = grid.len() as Num;

    deltas
        .iter()
        .filter_map(|delta| {
            let new_x: Num = pos.0 as Num + delta.0;
            let new_y: Num = pos.1 as Num + delta.1;

            let new_x_valid = new_x < height && new_x > -1;
            let new_y_valid = new_y < width && new_y > -1;

            if new_x_valid && new_y_valid {
                let new_valid_x = new_x as usize;
                let new_valid_y = new_y as usize;
                if grid[new_valid_x][new_valid_y] != '.'
                    && !visited.contains(&(new_valid_x, new_valid_y))
                {
                    return Some((new_valid_x, new_valid_y));
                } else {
                    return None;
                }
            } else {
                None
            }
        })
        .collect()
}

fn determine_dir(child: &Pos, parent: &Pos) -> char {
    let child_nums = (child.0 as Num, child.1 as Num);
    let parent_nums = (parent.0 as Num, parent.1 as Num);

    let delta = (child_nums.0 - parent_nums.0, child_nums.1 - parent_nums.1);

    match delta {
        (0, 1) => '>',
        (0, -1) => '<',
        (-1, 0) => '^',
        (1, 0) => 'v',
        _ => panic!("Unexpected delta: {:?}", delta),
    }
}

fn count_turns(path: &[char]) -> usize {
    if path.len() < 2 {
        return 0;
    }
    let mut turns = 0;
    let mut current_dir = path[0];

    for &next_dir in path.iter().skip(1) {
        if next_dir != current_dir {
            turns += 1;
            current_dir = next_dir;
        }
    }
    turns
}

fn bfs(grid: &Grid, start_char: char, end_char: char) -> Vec<char> {
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut queue: VecDeque<(Pos, Vec<char>)> = VecDeque::new();
    let start_pos = find_char(&grid, start_char);

    queue.push_back((start_pos, Vec::new()));
    let mut shortest_paths: Vec<Vec<char>> = Vec::new();
    let mut min_length = usize::MAX;

    let mut next_visit: HashSet<Pos> = HashSet::new();

    while let Some((curr_pos, curr_path)) = queue.pop_front() {
        if grid[curr_pos.0][curr_pos.1] == end_char {
            if curr_path.len() <= min_length {
                min_length = curr_path.len();
                shortest_paths.push(curr_path);
            }
            continue;
        }

        for next_pos in get_valid_moves(&grid, &curr_pos, &visited) {
            let mut new_path = curr_path.clone();
            new_path.push(determine_dir(&next_pos, &curr_pos));
            queue.push_back((next_pos, new_path));
            next_visit.insert(next_pos);
        }

        if queue.is_empty() || queue.front().unwrap().1.len() > curr_path.len() {
            visited.extend(next_visit.drain());
        }
    }

    shortest_paths
        .into_iter()
        .min_by_key(|path| count_turns(path))
        .unwrap_or_default()
}

fn solve_line(line: String, keypad: &Grid, dirpad: &Grid) -> Num {
    let num: Num = line.clone()[0..line.len() - 1].parse().unwrap();
    let mut original_sequence: Vec<char> = line.chars().collect();

    original_sequence.insert(0, 'A');
    let mut expanded_1: Vec<_> = original_sequence
        .windows(2)
        .flat_map(|pair| {
            let mut path = bfs(&keypad, pair[0], pair[1]);
            path.push('A');
            return path;
        })
        .collect();

    expanded_1.insert(0, 'A');
    let mut expanded_2: Vec<_> = expanded_1
        .windows(2)
        .flat_map(|pair| {
            let mut path = bfs(&dirpad, pair[0], pair[1]);
            path.push('A');
            return path;
        })
        .collect();

    expanded_2.insert(0, 'A');
    let expanded_3: Vec<_> = expanded_2
        .windows(2)
        .flat_map(|pair| {
            let mut path = bfs(&dirpad, pair[0], pair[1]);
            path.push('A');
            return path;
        })
        .collect();

    num * (expanded_3.len() as Num)
}

pub fn part_1(path: &str) -> String {
    let contents = lib::read_input(format!("input/{}", path));

    let keypad = vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec!['.', '0', 'A'],
    ];
    let dirpad = vec![vec!['.', '^', 'A'], vec!['<', 'v', '>']];

    let result: Num = contents
        .iter()
        .map(|line| {
            return solve_line(line.to_string(), &keypad, &dirpad);
        })
        .sum();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_21_part_1() {
        let test_result = part_1("day21_test.txt");
        assert_eq!(test_result, "126384");

        let test_result = part_1("day21.txt");
        assert_eq!(test_result, "188398");
    }
}
