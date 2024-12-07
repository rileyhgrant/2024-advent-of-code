#[path = "./lib.rs"]
mod lib;

#[derive(Debug)]
struct Guard {
    pos: (usize, usize),
    dir: (i32, i32),
}

impl Guard {
    fn new(pos: (usize, usize)) -> Self {
        Guard {
            pos: (pos.0, pos.1),
            dir: (-1, 0),
        }
    }

    fn rotate(&mut self) {
        match self.dir {
            (-1, 0) => self.dir = (0, 1),
            (0, 1) => self.dir = (1, 0),
            (1, 0) => self.dir = (0, -1),
            (0, -1) => self.dir = (-1, 0),
            _ => self.dir = self.dir,
        };
    }

    fn get_next_position(&self) -> (usize, usize) {
        let next_row = (self.pos.0 as i32 + self.dir.0) as usize;
        let next_col = (self.pos.1 as i32 + self.dir.1) as usize;
        (next_row, next_col)
    }

    fn step(&mut self) {
        let next_row = (self.pos.0 as i32 + self.dir.0) as usize;
        let next_col = (self.pos.1 as i32 + self.dir.1) as usize;
        self.pos = (next_row, next_col);
    }

    fn get_position(&self) -> (usize, usize) {
        (self.pos.0, self.pos.1)
    }

    fn get_dir(&self) -> (i32, i32) {
        (self.dir.0, self.dir.1)
    }
}

fn find_char(grid: &Vec<Vec<char>>, target: char) -> (usize, usize) {
    for (i, row) in grid.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if *ch == target {
                return (i, j);
            }
        }
    }
    (0, 0)
}

fn guard_walk(path: &str) -> (String, Vec<Vec<char>>) {
    let mut grid = lib::create_padded_grid(path, 'e', 1);
    let mut guard = Guard::new(find_char(&grid, '^'));

    let mut count = 0;
    loop {
        let curr = guard.get_position();
        let next = guard.get_next_position();

        let next_char = grid[next.0][next.1];
        if next_char == '.' || next_char == '^' || next_char == 'X' {
            if next_char != 'X' {
                count += 1;
            }
            guard.step();
            grid[curr.0][curr.1] = 'X';
        } else if next_char == '#' {
            guard.rotate();
        } else if next_char == 'e' {
            grid[curr.0][curr.1] = 'X';
            count += 1;
            break;
        } else {
            println!("something is horribly wrong...");
        }
    }

    (count.to_string(), grid)
}

pub fn part_1(path: &str) -> String {
    guard_walk(path).0
}

fn has_infinite_loop(grid: &mut Vec<Vec<char>>) -> bool {
    let mut guard = Guard::new(find_char(&grid, '^'));
    let mut turn_pos_list = Vec::<((usize, usize), (i32, i32))>::new();

    loop {
        let curr = guard.get_position();
        let next = guard.get_next_position();

        let curr_dir = guard.get_dir();

        let next_char = grid[next.0][next.1];
        if next_char == '.' || next_char == '^' || next_char == 'X' {
            guard.step();
            grid[curr.0][curr.1] = 'X';
        } else if next_char == '#' {
            // if you ever encounter a turn you've already hit, in the same
            //   direction, that's means there's a loop
            if turn_pos_list.contains(&(next, curr_dir)) {
                return true;
            } else {
                turn_pos_list.push((next, curr_dir));
            }
            guard.rotate();
        } else if next_char == 'e' {
            grid[curr.0][curr.1] = 'X';
            break;
        } else {
            println!("something is horribly wrong...");
        }
    }
    false
}

pub fn part_2(path: &str) -> String {
    let grid = lib::create_padded_grid(path, 'e', 1);

    let guard_initial_position = find_char(&grid, '^');
    let mut to_check_grid = guard_walk(path).1;
    to_check_grid[guard_initial_position.0][guard_initial_position.1] = '^';

    let mut cycle_count = 0;
    for (i, row) in to_check_grid.iter().enumerate() {
        for (j, _col) in row.iter().enumerate() {
            if to_check_grid[i][j] == 'X' {
                let mut test_grid = to_check_grid.clone();
                test_grid[i][j] = '#';
                if has_infinite_loop(&mut test_grid) {
                    cycle_count += 1
                }
            } else {
                continue;
            }
        }
    }
    cycle_count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_6_part_1() {
        let test_result = part_1("day06_test.txt");
        assert_eq!(test_result, "41");

        let test_result = part_1("day06.txt");
        assert_eq!(test_result, "5199");
    }

    #[test]
    fn test_day_6_part_2() {
        let test_result = part_2("day06_test.txt");
        assert_eq!(test_result, "6");

        let test_result = part_2("day06.txt");
        assert_eq!(test_result, "1915");
    }
}
