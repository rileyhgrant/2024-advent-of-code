#[path = "./lib.rs"]
mod lib;

use std::collections::{HashMap, HashSet};

type Num = i64;

fn generate_next_in_seq(initial_num: Num, num_times: Num) -> Num {
    let secret_number = if num_times == 1 {
        initial_num
    } else {
        generate_next_in_seq(initial_num, num_times - 1)
    };

    let modulo = 0b11111111_11111111_11111111;
    let step_1 = (secret_number << 6 ^ secret_number) & modulo;
    let step_2 = (step_1 >> 5 ^ step_1) & modulo;
    let step_3 = (step_2 << 11 ^ step_2) & modulo;

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

fn generate_ones_sequence(initial_num: Num, num_times: Num, vec: &mut Vec<Num>) -> Num {
    let secret_number = if num_times == 1 {
        let ones = initial_num % 10;
        vec.push(ones);
        initial_num
    } else {
        generate_ones_sequence(initial_num, num_times - 1, vec)
    };

    let modulo = 0b11111111_11111111_11111111;
    let step_1 = (secret_number << 6 ^ secret_number) & modulo;
    let step_2 = (step_1 >> 5 ^ step_1) & modulo;
    let step_3 = (step_2 << 11 ^ step_2) & modulo;

    let ones = step_3 % 10;
    vec.push(ones);

    step_3
}

fn add_sequences_to_set(ones_vec: &Vec<Num>, diffs_to_value_map: &mut HashMap<Vec<Num>, Num>) {
    let mut sequences_seen: HashSet<Vec<Num>> = HashSet::new();
    ones_vec.windows(5).for_each(|window| {
        let diffs_vec = vec![
            window[1] - window[0],
            window[2] - window[1],
            window[3] - window[2],
            window[4] - window[3],
        ];

        if !sequences_seen.contains(&diffs_vec) {
            sequences_seen.insert(diffs_vec.clone());
            diffs_to_value_map
                .entry(diffs_vec)
                .and_modify(|e| *e += window[4])
                .or_insert(window[4]);
        }
    })
}

pub fn part_2(path: &str) -> String {
    let contents = lib::read_input(format!("input/{}", path));
    let num_times: Num = 2000;

    let mut ones_sequence_map: HashMap<Num, Vec<Num>> = HashMap::new();
    contents
        .iter()
        .map(|line| line.parse().unwrap())
        .for_each(|initial_num| {
            let mut sequence = Vec::new();
            generate_ones_sequence(initial_num, num_times, &mut sequence);
            ones_sequence_map.insert(initial_num, sequence);
        });

    let mut diffs_to_value_map: HashMap<Vec<Num>, Num> = HashMap::new();
    ones_sequence_map.iter().for_each(|(_key, value)| {
        add_sequences_to_set(value, &mut diffs_to_value_map);
    });

    let mut as_vec: Vec<_> = diffs_to_value_map.iter().collect();
    as_vec.sort_by_key(|(_k, v)| *v);

    let max = diffs_to_value_map.values().max().unwrap();

    max.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_22_part_1() {
        let test_result = part_1("day22_test.txt");
        assert_eq!(test_result, "37327623");

        let test_result = part_1("day22.txt");
        assert_eq!(test_result, "12979353889");
    }

    #[test]
    fn test_day_22_part_2() {
        let test_result = part_2("day22_test2.txt");
        assert_eq!(test_result, "23");

        let test_result = part_2("day22.txt");
        assert_eq!(test_result, "1449");
    }
}
