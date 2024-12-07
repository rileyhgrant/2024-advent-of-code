#[path = "./lib.rs"]
mod lib;

fn parse_line(line: &str) -> (u64, Vec<u64>) {
    let (test_value, numbers) = line.split_once(":").unwrap();
    let test_value_int = test_value.parse().unwrap();
    let number_vec: Vec<u64> = numbers
        .trim()
        .split(" ")
        .map(|el| el.parse().unwrap())
        .collect();

    (test_value_int, number_vec)
}

fn check_equals(target: u64, op: char, check_vec: Vec<u64>) -> bool {
    let partial: u64 = match op {
        '+' => check_vec[0] + check_vec[1],
        '*' => check_vec[0] * check_vec[1],
        _ => {
            println!("Something is horribly wrong");
            0
        }
    };

    if partial > target {
        false
    } else if check_vec.len() == 2 {
        partial == target
    } else {
        let operators = vec!['+', '*'];
        let mut recur_vec = vec![partial];
        recur_vec.extend(&check_vec[2..]);

        let result = operators.iter().any(|&op| {
            return check_equals(target, op, recur_vec.clone());
        });
        result
    }
}

pub fn part_1(path: &str) -> String {
    let contents = lib::read_input(format!("input/{}", path));

    let mut sum = 0;
    for line in contents.iter() {
        let (test_value, numbers) = parse_line(line);
        let operators = vec!['+', '*'];

        let result = operators.iter().any(|&op| {
            return check_equals(test_value, op, numbers.clone());
        });

        if result == true {
            sum += test_value;
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_7_part_1() {
        let test_result = part_1("day07_test.txt");
        assert_eq!(test_result, "3749");

        let test_result = part_1("day07.txt");
        assert_eq!(test_result, "5540634308362");
    }
}
