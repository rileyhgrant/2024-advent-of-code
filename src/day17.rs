#[path = "./lib.rs"]
mod lib;

use std::fs;

type Num = i64;

fn initialize_registers(register_string: &str) -> (Num, Num, Num) {
    let registers_vec: Vec<Num> = lib::string_to_vec_string(register_string.to_string())
        .iter()
        .map(|line| {
            line.split(":")
                .nth(1)
                .unwrap()
                .trim()
                .parse::<Num>()
                .unwrap()
        })
        .collect();

    (registers_vec[0], registers_vec[1], registers_vec[2])
}

fn parse_program(program_string: &str) -> Vec<Num> {
    let program_vec: Vec<Num> = lib::string_to_vec_string(program_string.to_string())
        .iter()
        .flat_map(|line| {
            line.split(":")
                .nth(1)
                .unwrap()
                .split(",")
                .map(|el| el.trim().parse::<Num>().unwrap())
        })
        .collect();

    program_vec
}

fn perform_operation(
    input_vec: &Vec<Num>,
    idx: &mut Num,
    max_idx: Num,
    reg_a: &mut Num,
    reg_b: &mut Num,
    reg_c: &mut Num,
) -> Option<Num> {
    if *idx >= max_idx {
        return None;
    }

    let opcode = input_vec[*idx as usize];
    if *idx + 1 >= max_idx {
        return None;
    }
    let operand_code = input_vec[*idx as usize + 1];

    let operand: Num = match operand_code {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => reg_a.clone(),
        5 => reg_b.clone(),
        6 => reg_c.clone(),
        _ => {
            println!(
                "operand code was: {}, something is horribly wrong",
                operand_code
            );
            panic!()
        }
    };

    match opcode {
        0 => {
            let numerator = reg_a.clone();
            let denominator = 2_i64.pow(operand as u32);
            let result = numerator / denominator;
            *reg_a = result;
            *idx += 2;
            return None;
        }
        1 => {
            let first = reg_b.clone();
            let second = operand_code;
            let result = first ^ second;
            *reg_b = result;
            *idx += 2;
            return None;
        }
        2 => {
            let result = operand % 8;
            *reg_b = result;
            *idx += 2;
            return None;
        }
        3 => {
            if *reg_a != 0 {
                *idx = operand_code;
                return None;
            } else {
                *idx += 2;
                return None;
            }
        }
        4 => {
            let first = reg_b.clone();
            let second = reg_c.clone();
            let result = first ^ second;
            *reg_b = result;
            *idx += 2;
            return None;
        }
        5 => {
            let result = operand % 8;
            *idx += 2;
            return Some(result);
        }
        6 => {
            let numerator = reg_a.clone();
            let denominator = 2_i64.pow(operand as u32);
            let result = numerator / denominator;
            *reg_b = result;
            *idx += 2;
            return None;
        }
        7 => {
            let numerator = reg_a.clone();
            let denominator = 2_i64.pow(operand as u32);
            let result = numerator / denominator;
            *reg_c = result;
            *idx += 2;
            return None;
        }
        _ => {
            println!("Something went horribly wrong");
            panic!();
        }
    }
}

pub fn part_1(path: &str) -> String {
    let contents = fs::read_to_string(format!("input/{}", path))
        .expect("Should have been able to read the file");
    let (register_string, program_string) = contents.split_once("\n\n").unwrap();
    let registers = initialize_registers(register_string);
    let mut register_a = registers.0.clone();
    let mut register_b = registers.1.clone();
    let mut register_c = registers.2.clone();
    let program = parse_program(program_string);
    let mut output_vec: Vec<Num> = Vec::new();

    let mut idx: Num = 0;
    let max_idx = program.iter().len() as Num;
    loop {
        if idx >= max_idx {
            break;
        }
        let result = perform_operation(
            &program,
            &mut idx,
            max_idx,
            &mut register_a,
            &mut register_b,
            &mut register_c,
        );

        match result {
            Some(res) => output_vec.push(res),
            None => (),
        }
    }

    let mut result_string: String = output_vec
        .iter()
        .map(|el| {
            let mut el_string = el.to_string();
            el_string.push(',');
            el_string
        })
        .collect();

    result_string.pop();
    result_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_17_part_1() {
        let test_result = part_1("day17_test.txt");
        assert_eq!(test_result, "4,6,3,5,6,3,5,2,1,0");

        let test_result = part_1("day17.txt");
        assert_eq!(test_result, "6,7,5,2,1,3,5,1,7");
    }
}
