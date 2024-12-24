#[path = "./lib.rs"]
mod lib;

use std::fs;

type Num = i64;

use std::collections::HashMap;

fn initialize_wires(wires_string: &str, wires: &mut HashMap<String, Num>) {
    lib::string_to_vec_string(wires_string.to_string())
        .iter()
        .for_each(|wire_string| {
            let parts = wire_string.split_once(":").unwrap();
            let wire = parts.0.to_string();
            let value: Num = parts.1.trim().parse().unwrap();
            wires.insert(wire, value);
        });
}

fn simulate_gates(gates_string: &str, wires: &mut HashMap<String, Num>) {
    let mut gates_vec: Vec<(String, String, String, String)> =
        lib::string_to_vec_string(gates_string.to_string())
            .iter()
            .map(|gate_string| {
                let parts = gate_string.split_once("->").unwrap();
                let output_wire = parts.1.trim().to_string();

                let input_parts: Vec<&str> = parts.0.trim().split_whitespace().collect();
                let input_wire_1 = input_parts[0].trim().to_string();
                let input_wire_2 = input_parts[2].trim().to_string();

                let operator = input_parts[1].trim().to_string();

                return (input_wire_1, operator, input_wire_2, output_wire);
            })
            .collect();

    loop {
        let curr_len = gates_vec.len();
        if curr_len == 0 {
            break;
        }

        for i in 0..curr_len {
            let (input_wire_1, operator, input_wire_2, output_wire) = &gates_vec[i];

            if wires.contains_key(input_wire_1) && wires.contains_key(input_wire_2) {
                match operator.as_str() {
                    "AND" => {
                        let val1 = wires.get(input_wire_1).unwrap();
                        let val2 = wires.get(input_wire_2).unwrap();
                        wires.insert(output_wire.to_string(), val1 & val2);
                    }
                    "OR" => {
                        let val1 = wires.get(input_wire_1).unwrap();
                        let val2 = wires.get(input_wire_2).unwrap();
                        wires.insert(output_wire.to_string(), val1 | val2);
                    }
                    "XOR" => {
                        let val1 = wires.get(input_wire_1).unwrap();
                        let val2 = wires.get(input_wire_2).unwrap();
                        wires.insert(output_wire.to_string(), val1 ^ val2);
                    }
                    _ => panic!("Unknown operator: {}", operator),
                }
                gates_vec.remove(i);
                break;
            }
        }
    }
}

fn return_z_bits_number(wires: &mut HashMap<String, Num>) -> Num {
    let mut result_vec: Vec<(&String, &i64)> = wires
        .iter()
        .filter(|(key, _)| key.starts_with('z'))
        .collect();

    result_vec.sort_by(|a, b| a.0.cmp(b.0));

    result_vec
        .iter()
        .enumerate()
        .map(|(i, &val)| *val.1 << i)
        .sum::<Num>()
}

pub fn part_1(path: &str) -> String {
    let contents = fs::read_to_string(format!("input/{}", path))
        .expect("Should have been able to read the file");
    let (wires_string, gates_string) = contents.split_once("\n\n").unwrap();

    let mut wires: HashMap<String, Num> = HashMap::new();
    initialize_wires(wires_string, &mut wires);

    simulate_gates(gates_string, &mut wires);

    let result = return_z_bits_number(&mut wires);

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_24_part_1() {
        let test_result = part_1("day24_test1.txt");
        assert_eq!(test_result, "4");

        let test_result = part_1("day24_test2.txt");
        assert_eq!(test_result, "2024");

        let test_result = part_1("day24.txt");
        assert_eq!(test_result, "52956035802096");
    }
}
