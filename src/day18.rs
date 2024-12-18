#[path = "./lib.rs"]
mod lib;

type Num = i64;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Eq, PartialEq)]
struct SearchState {
    priority: Num,
    position: (usize, usize),
    steps: Num,
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn simulate_falling_bytes(
    grid: &mut Vec<Vec<char>>,
    bytes: Vec<(usize, usize)>,
    n_simulations: usize,
) {
    let byte_symbol = '#';

    for n in 0..n_simulations {
        let byte_loc = bytes[n];
        grid[byte_loc.0][byte_loc.1] = byte_symbol;
    }
}

fn get_valid_moves(
    curr_pos: (usize, usize),
    grid: &Vec<Vec<char>>,
    seen: &HashSet<(usize, usize)>,
    width: usize,
    height: usize,
) -> Vec<(usize, usize)> {
    let deltas = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    let valids: Vec<(usize, usize)> = deltas
        .iter()
        .filter_map(|delta| {
            let new_x: Num = curr_pos.0 as Num + delta.0;
            let new_y: Num = curr_pos.1 as Num + delta.1;

            let upper_x: Num = width as Num - 1;
            let upper_y: Num = height as Num - 1;

            if new_x < 0 || new_x > upper_x || new_y < 0 || new_y > upper_y {
                return None;
            }

            let new_safe_x = new_x as usize;
            let new_safe_y = new_y as usize;
            if grid[new_safe_x][new_safe_y] == '#' {
                return None;
            }

            let to_check = (new_safe_x, new_safe_y);
            if seen.contains(&to_check) {
                return None;
            }

            return Some(to_check);
        })
        .collect();

    valids
}

fn bfs(
    grid: &Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
    width: usize,
    height: usize,
    least_efficient: bool,
    max_steps: Num,
) -> Option<(Num, Vec<(usize, usize)>)> {
    let mut visited = HashSet::new();
    let mut came_from = HashMap::new();

    let calculate_priority = |pos: (usize, usize), steps: usize| -> Num {
        if !least_efficient {
            return steps as Num;
        } else {
            // TODO: this heuristic is not helping me much, if at all
            let dx = (end.0 as Num - pos.0 as Num).abs();
            let dy = (end.1 as Num - pos.1 as Num).abs();
            let manhattan = dx + dy;
            -(manhattan * 2 + steps as Num)
        }
    };

    let mut queue = BinaryHeap::new();
    queue.push(SearchState {
        priority: calculate_priority(start, 0),
        position: start,
        steps: 0,
    });

    while let Some(SearchState {
        position, steps, ..
    }) = queue.pop()
    {
        if steps > max_steps {
            continue;
        }

        if !visited.insert(position) {
            continue;
        }

        if position == end {
            let mut path = vec![];
            let mut current = position;
            while current != start {
                path.push(current);
                current = came_from[&current];
            }
            path.push(start);
            path.reverse();

            return Some((steps as Num, path));
        }

        for valid_move in get_valid_moves(position, grid, &visited, width, height) {
            came_from.insert(valid_move, position);

            queue.push(SearchState {
                priority: calculate_priority(valid_move, (steps as usize) + 1),
                position: valid_move,
                steps: steps + 1,
            });
        }
    }

    None
}

fn solve_part_1(
    path: &str,
    width: usize,
    height: usize,
    n_simulations: usize,
    least_efficient: bool,
    max_steps: Num,
) -> Option<Num> {
    let mut grid = vec![vec!['.'; width]; height];

    let bytes: Vec<(usize, usize)> = lib::read_input(format!("input/{}", path))
        .iter()
        .map(|b| {
            // The problems notion of x/y is flipped from normal convention
            //   First num in pair is distance from left edge
            //   Second num in pair is distance from right edge
            let second = b.split(",").nth(0).unwrap().parse().unwrap();
            let first = b.split(",").nth(1).unwrap().parse().unwrap();
            return (first, second);
        })
        .collect();

    simulate_falling_bytes(&mut grid, bytes, n_simulations);

    let start = (0, 0);
    let end = (width - 1, height - 1);
    let result = bfs(&grid, start, end, width, height, least_efficient, max_steps);

    match result {
        Some(val) => Some(val.0),
        None => None,
    }
}

pub fn part_1(path: &str) -> String {
    let width: usize = 71;
    let height: usize = 71;
    let n_simulations: usize = 1024;
    let result = solve_part_1(path, width, height, n_simulations, false, Num::MAX);
    result.unwrap().to_string()
}

fn solve_part_2(
    path: &str,
    width: usize,
    height: usize,
    n_initial_simulations: usize,
    least_efficient_step_count: Num,
) -> String {
    let mut grid = vec![vec!['.'; width]; height];
    let bytes: Vec<(usize, usize)> = lib::read_input(format!("input/{}", path))
        .iter()
        .map(|b| {
            // The problems notion of x/y is flipped from normal convention
            //   First num in pair is distance from left edge
            //   Second num in pair is distance from right edge
            let second = b.split(",").nth(0).unwrap().parse().unwrap();
            let first = b.split(",").nth(1).unwrap().parse().unwrap();
            return (first, second);
        })
        .collect();
    simulate_falling_bytes(&mut grid, bytes.clone(), n_initial_simulations);

    let start = (0, 0);
    let end = (width - 1, height - 1);

    let mut first_blocker = 0;
    for i in n_initial_simulations..bytes.len() {
        simulate_falling_bytes(&mut grid, vec![bytes[i]], 1);

        let res = bfs(
            &grid,
            start,
            end,
            width,
            height,
            false,
            least_efficient_step_count + 4,
        );
        match res {
            Some(_) => continue,
            None => {
                first_blocker = i;
                break;
            }
        }
    }

    let blocker = bytes[first_blocker];
    let to_return = format!("{},{}", blocker.1, blocker.0);
    to_return
}

fn part_2_helper(path: &str, width: usize, height: usize, n_initial_simulations: usize) -> String {
    let least_efficient_step_count: Num =
        solve_part_1(path, width, height, n_initial_simulations, true, Num::MAX).unwrap();

    let result = solve_part_2(
        path,
        width,
        height,
        n_initial_simulations,
        least_efficient_step_count,
    );
    result
}

pub fn part_2(path: &str) -> String {
    let width: usize = 71;
    let height: usize = 71;
    let n_simulations: usize = 1024;
    part_2_helper(path, width, height, n_simulations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_18_part_1() {
        let width = 7;
        let height = 7;
        let n_simulations = 12;
        let test_result = solve_part_1(
            "day18_test.txt",
            width,
            height,
            n_simulations,
            false,
            Num::MAX,
        );
        assert_eq!(test_result.unwrap().to_string(), "22");

        let test_result = part_1("day18.txt");
        assert_eq!(test_result, "338");
    }

    #[test]
    fn test_day_18_part_2() {
        let width = 7;
        let height = 7;
        let n_simulations = 12;
        let test_result = part_2_helper("day18_test.txt", width, height, n_simulations);
        assert_eq!(test_result, "6,1");

        let test_result = part_2("day18.txt");
        assert_eq!(test_result, "20,44");
    }
}
