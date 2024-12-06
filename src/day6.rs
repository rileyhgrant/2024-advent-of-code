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
            dir: (-1, 0)
        }
    }
   
    fn rotate(&mut self) {
        match self.dir {
            (-1, 0) => self.dir = (0, 1),
            (0, 1)  => self.dir = (1, 0),
            (1, 0)  => self.dir = (0, -1),
            (0, -1) => self.dir = (-1, 0),
            _ => self.dir = self.dir
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

pub fn part_1(path: &str) -> String {
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

    count.to_string()
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



}
