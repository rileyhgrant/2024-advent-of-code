#[path = "./lib.rs"]
mod lib;

type Num = i64;

fn generate_next_in_seq(initial_num: Num, num_times: Num) -> Num {
    let secret_number = if num_times == 1 {
        initial_num
    } else {
        generate_next_in_seq(initial_num, num_times - 1)
    };

    let modulo = 16777216;

    let step_1 = (secret_number << 6 ^ secret_number) % modulo;
    let step_2 = (step_1 >> 5 ^ step_1) % modulo;
    let step_3 = (step_2 << 11 ^ step_2) % modulo;

    step_3
}

pub fn part_1(path: &str) -> String {
    let contents = lib::read_input(format!("input/{}", path));
    let num_times: Num = 2000;

    let result: Num = contents
        .iter()
        .map(|line| line.parse().unwrap())
        .map(|initial_num| generate_next_in_seq(initial_num, num_times))
        .sum();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_18_part_1() {
        let test_result = part_1("day22_test.txt");
        assert_eq!(test_result, "37327623");

        let test_result = part_1("day22.txt");
        assert_eq!(test_result, "12979353889");
    }
}
