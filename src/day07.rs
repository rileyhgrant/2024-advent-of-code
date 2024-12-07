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

pub enum Operator {
    Add,
    Multiply,
    Concat,
}

fn can_create_solution_recursion(
    target: u64,
    op: &Operator,
    operators: &Vec<Operator>,
    check_vec: &Vec<u64>,
) -> bool {
    let partial: u64 = match op {
        Operator::Add => check_vec[0] + check_vec[1],
        Operator::Multiply => check_vec[0] * check_vec[1],
        Operator::Concat => (check_vec[0].to_string() + &check_vec[1].to_string())
            .parse::<u64>()
            .ok()
            .unwrap(),
    };

    if partial > target {
        false
    } else if check_vec.len() == 2 {
        partial == target
    } else {
        let mut recur_vec = Vec::with_capacity(check_vec.len() - 1);
        recur_vec.push(partial);
        recur_vec.extend(&check_vec[2..]);

        let result = operators.iter().any(|op| {
            return can_create_solution_recursion(target, op, operators, &recur_vec);
        });
        result
    }
}

fn can_create_solution(target: u64, operators: &Vec<Operator>, check_vec: &Vec<u64>) -> bool {
    operators.iter().any(|op| {
        return can_create_solution_recursion(target, op, operators, check_vec);
    })
}

pub fn solve_with_operators(path: &str, operators: &Vec<Operator>) -> String {
    let contents = lib::read_input(format!("input/{}", path));

    let mut sum = 0;
    for line in contents.iter() {
        let (test_value, numbers) = parse_line(line);
        if can_create_solution(test_value, operators, &numbers) == true {
            sum += test_value
        }
    }

    sum.to_string()
}

pub fn part_1(path: &str) -> String {
    solve_with_operators(path, &vec![Operator::Add, Operator::Multiply])
}

pub fn part_2(path: &str) -> String {
    solve_with_operators(
        path,
        &vec![Operator::Add, Operator::Multiply, Operator::Concat],
    )
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

    #[test]
    fn test_day_7_part_2() {
        let test_result = part_2("day07_test.txt");
        assert_eq!(test_result, "11387");

        let test_result = part_2("day07.txt");
        assert_eq!(test_result, "472290821152397");
    }
}
