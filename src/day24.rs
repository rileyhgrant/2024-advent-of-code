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

fn generate_gates(gates_string: &str) -> Vec<(String, String, String, String)> {
    let gates_vec: Vec<(String, String, String, String)> =
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

    gates_vec
}

fn simulate_gates(gates_string: &str, wires: &mut HashMap<String, Num>) {
    let mut gates_vec = generate_gates(gates_string);
    println!("There are: {} gates", gates_vec.len());

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

fn find_malformed_gates(
    gates: &Vec<(String, String, String, String)>,
    max_z: Num,
    max_xy: Num,
) -> Vec<Vec<((String, String, String, String), usize)>> {
    let mut malformed_gates: Vec<Vec<((String, String, String, String), usize)>> =
        vec![Vec::new(), Vec::new(), Vec::new()];

    let gates_len = gates.len();

    for i in 0..gates_len {
        let gate = &gates[i];

        // rule 1 - unless its the final bit, z outputs need to have xor as the op to account for
        //   carry bit
        if gate.3.starts_with("z") {
            if !(gate.1 == "XOR") {
                if max_z == max_xy || (max_z != max_xy && !gate.3.ends_with(&max_z.to_string())) {
                    let mut rule_1_breakers = malformed_gates[0].clone();
                    rule_1_breakers.push((gate.clone(), i));
                    malformed_gates[0] = rule_1_breakers;
                }
            }

        // rule 2 random name registers are the carry, they must be and/or and NOT xor
        } else {
            let input_1_x_or_y = gate.0.starts_with("x") || gate.0.starts_with("y");
            let input_2_x_or_y = gate.2.starts_with("x") || gate.2.starts_with("y");
            if !(input_1_x_or_y && input_2_x_or_y) {
                if gate.1 == "XOR" {
                    let mut rule_2_breakers = malformed_gates[1].clone();
                    rule_2_breakers.push((gate.clone(), i));
                    malformed_gates[1] = rule_2_breakers;
                }
            }
        }

        // rule 3 if x/y as inputs, the output must be used in a specific operation
        //   XOR -> XOR, AND -> OR
        let input_1_x_or_y_and_not_zero =
            (gate.0.starts_with("x") || gate.0.starts_with("y")) && !gate.0.ends_with("00");
        let input_2_x_or_y_and_not_zero =
            (gate.2.starts_with("x") || gate.2.starts_with("y")) && !gate.2.ends_with("00");

        if input_1_x_or_y_and_not_zero && input_2_x_or_y_and_not_zero {
            let followup = if gate.1 == "XOR" {
                Some("XOR")
            } else if gate.1 == "AND" {
                Some("OR")
            } else {
                None
            };

            if let Some(followup_operator) = followup {
                let mut found = false;
                for j in 0..gates_len {
                    let gate_clone = &gates[j];
                    if gate_clone.1 == followup_operator {
                        if gate_clone.0 == gate.3 || gate_clone.2 == gate.3 {
                            found = true;
                        }
                    }
                }
                if !found && !gate.3.starts_with("z") {
                    let mut rule_3_breakers = malformed_gates[2].clone();
                    rule_3_breakers.push((gate.clone(), i));
                    malformed_gates[2] = rule_3_breakers;
                }
            }
        }
    }

    malformed_gates
}

pub fn part_2(path: &str) -> String {
    let contents = fs::read_to_string(format!("input/{}", path))
        .expect("Should have been able to read the file");
    let (wires_string, gates_string) = contents.split_once("\n\n").unwrap();

    let mut wires: HashMap<String, Num> = HashMap::new();
    initialize_wires(wires_string, &mut wires);

    let gates_vec = generate_gates(gates_string);

    let max_z: Num = gates_vec
        .iter()
        .filter(|gate| gate.3.starts_with("z"))
        .map(|gate| gate.3[1..].parse::<Num>().unwrap())
        .max()
        .unwrap();

    let max_xy: Num = gates_vec
        .iter()
        .flat_map(|gate| vec![&gate.0, &gate.2]) // Look at both positions
        .filter(|wire| wire.starts_with("x"))
        .map(|wire| wire[1..].parse::<Num>().unwrap())
        .max()
        .unwrap();

    let trouble_gates = find_malformed_gates(&gates_vec, max_z, max_xy);

    let mut output = trouble_gates
        .iter()
        .flatten()
        .map(|gate| gate.0 .3.clone())
        .collect::<Vec<String>>();

    output.sort();
    output.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_24_part_1() {
        let test_result = part_1("day24_test1.txt");
        assert_eq!(test_result, "4");

        // test input 2 is tricky! It obscures that your real input
        //   is a ripple carry adder, in the input there's things like
        //   x04 AND y07 -> vfj
        // but in your real input, they take the form of
        //   x04 XOR y04 -> vfj
        //   as a rule
        let test_result = part_1("day24_test2.txt");
        assert_eq!(test_result, "2024");

        let test_result = part_1("day24.txt");
        assert_eq!(test_result, "52956035802096");
    }

    #[test]
    fn test_day_24_part_2() {
        let test_result = part_2("day24.txt");
        assert_eq!(test_result, "hnv,hth,kfm,tqr,vmv,z07,z20,z28");
    }
}
