#[path = "./lib.rs"]
mod lib;

type Num = i32;
type Position = (Num, Num);
type Velocity = (Num, Num);

fn quadrant_product_after_seconds(path: &str, iterations: Num, width: Num, height: Num) -> String {
    let contents = lib::read_input(format!("input/{}", path));

    let mut topleft = 0;
    let mut topright = 0;
    let mut botleft = 0;
    let mut botright = 0;

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

    let robots_posns_post: Vec<Position> = robots_init
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

    robots_posns_post.iter().for_each(|posn| {
        if !(posn.0 == width / 2) && !(posn.1 == height / 2) {
            if posn.0 < width / 2 {
                if posn.1 < height / 2 {
                    topleft += 1
                } else if posn.1 > height / 2 {
                    botleft += 1
                }
            } else if posn.0 > width / 2 {
                if posn.1 < height / 2 {
                    topright += 1
                } else if posn.1 > height / 2 {
                    botright += 1
                }
            }
        }
    });

    let sum = topleft * topright * botleft * botright;
    sum.to_string()
}

pub fn part_1(path: &str) -> String {
    let iterations: Num = 100;
    let width: Num = 101;
    let height: Num = 103;
    quadrant_product_after_seconds(path, iterations, width, height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_14_part_1() {
        let test_result = quadrant_product_after_seconds("day14_test.txt", 100, 11, 7);
        assert_eq!(test_result, "12");

        let test_result = part_1("day14.txt");
        assert_eq!(test_result, "218965032");
    }
}
