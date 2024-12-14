#[path = "./lib.rs"]
mod lib;

type Num = i64;
type Position = (Num, Num);
type Velocity = (Num, Num);

use std::collections::HashMap;

fn get_initial_robots(path: &str) -> Vec<(Position, Velocity)> {
    let contents = lib::read_input(format!("input/{}", path));

    let robots_init: Vec<(Position, Velocity)> = contents
        .iter()
        .map(|line| {
            let posns: Vec<Num> = line
                .split(" ")
                .nth(0)
                .unwrap()
                .split("=")
                .nth(1)
                .unwrap()
                .split(",")
                .map(|el| el.parse().ok().unwrap())
                .collect();

            let vel: Vec<Num> = line
                .split(" ")
                .nth(1)
                .unwrap()
                .split("=")
                .nth(1)
                .unwrap()
                .split(",")
                .map(|el| el.parse().ok().unwrap())
                .collect();

            let init = ((posns[0], posns[1]), (vel[0], vel[1]));
            init
        })
        .collect();

    robots_init
}

fn quadrant_vectors_after_seconds(
    robots: &Vec<(Position, Velocity)>,
    iterations: Num,
    width: Num,
    height: Num,
) -> (
    Vec<Position>,
    Vec<Position>,
    Vec<Position>,
    Vec<Position>,
    Vec<Position>,
) {
    let mut topleft: Vec<Position> = Vec::new();
    let mut topright: Vec<Position> = Vec::new();
    let mut botleft: Vec<Position> = Vec::new();
    let mut botright: Vec<Position> = Vec::new();

    let robot_posns_post: Vec<Position> = robots
        .iter()
        .map(|robot_init| {
            let pos = robot_init.0;
            let vel = robot_init.1;

            let new_x: Num = (((pos.0 + vel.0 * iterations) % width) + width) % width;
            let new_y: Num = (((pos.1 + vel.1 * iterations) % height) + height) % height;

            let new_posn = (new_x, new_y);
            new_posn
        })
        .collect();

    robot_posns_post.iter().for_each(|&posn| {
        if !(posn.0 == width / 2) && !(posn.1 == height / 2) {
            if posn.0 < width / 2 {
                if posn.1 < height / 2 {
                    topleft.push(posn);
                } else if posn.1 > height / 2 {
                    botleft.push(posn);
                }
            } else if posn.0 > width / 2 {
                if posn.1 < height / 2 {
                    topright.push(posn);
                } else if posn.1 > height / 2 {
                    botright.push(posn);
                }
            }
        }
    });

    (topleft, topright, botleft, botright, robot_posns_post)
}

fn solve_part_1(path: &str, iterations: Num, width: Num, height: Num) -> String {
    let robots = get_initial_robots(path);
    let (topleft, topright, botleft, botright, _all) =
        quadrant_vectors_after_seconds(&robots, iterations, width, height);

    let topleft_count = topleft.len();
    let topright_count = topright.len();
    let botleft_count = botleft.len();
    let botright_count = botright.len();
    let product = topleft_count * topright_count * botleft_count * botright_count;

    product.to_string()
}

pub fn part_1(path: &str) -> String {
    let iterations: Num = 100;
    let width: Num = 101;
    let height: Num = 103;
    solve_part_1(path, iterations, width, height)
}

fn print_board(width: Num, height: Num, robots: &Vec<Position>) {
    let print_width: usize = width as usize;
    let print_height: usize = height as usize;
    let mut grid = vec![vec!['.'; print_width]; print_height];

    robots.iter().for_each(|robot| {
        grid[robot.1 as usize][robot.0 as usize] = 'X';
    });

    println!("");
    for line in grid.iter() {
        println!("{}", line.iter().collect::<String>());
    }
    println!("");
}

fn has_horizontal_line(robots: Vec<Position>, min_length: usize) -> bool {
    let mut rows: HashMap<Num, Vec<Num>> = HashMap::new();

    robots
        .iter()
        .for_each(|(x, y)| rows.entry(*y).or_insert(Vec::new()).push(*x));

    for (_y, mut x_positions) in rows {
        x_positions.sort();

        let mut current_length = 1;
        let mut max_length = 1;

        for i in 1..x_positions.len() {
            if x_positions[i] == x_positions[i - 1] + 1 {
                current_length += 1;
                max_length = max_length.max(current_length);
            } else {
                current_length = 1;
            }
        }
        if max_length >= min_length {
            return true;
        }
    }
    false
}

fn solve_part_2(path: &str, width: Num, height: Num, print: bool) -> String {
    let robots_init = get_initial_robots(path);

    let mut i = 0;
    loop {
        let robots_post = quadrant_vectors_after_seconds(&robots_init, i, width, height);
        let all_robots_clone = robots_post.4.clone();
        let has_line = has_horizontal_line(robots_post.4, 10);
        if has_line {
            if print {
                print_board(width, height, &all_robots_clone);
            }
            break;
        }

        i += 1;
        if i >= 100_000 {
            println!("You'd best look elsewhere for your tree, check some assumptions");
            break;
        }
    }

    i.to_string()
}

pub fn part_2(path: &str, print: bool) -> String {
    let width: Num = 101;
    let height: Num = 103;
    solve_part_2(path, width, height, print)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_14_part_1() {
        let test_result = solve_part_1("day14_test.txt", 100, 11, 7);
        assert_eq!(test_result, "12");

        let test_result = part_1("day14.txt");
        assert_eq!(test_result, "218965032");
    }

    #[test]
    fn test_day_14_part_2() {
        let test_result = part_2("day14.txt", false);
        assert_eq!(test_result, "7037");
    }
}
