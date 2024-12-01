#[path = "./lib.rs"]
mod lib;

fn shape_input(filepath: &str) -> (Vec<i32>, Vec<i32>) {
    let contents = lib::read_input(format!("input/{}", filepath));

    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();

    for line in contents.iter() {
        let mut numbers = line.split_whitespace();

        if let Some(left) = numbers.next() {
            left_numbers.push(left.parse::<i32>().unwrap());
        }

        if let Some(right) = numbers.next() {
            right_numbers.push(right.parse::<i32>().unwrap());
        }
    }

    left_numbers.sort();
    right_numbers.sort();

    return (left_numbers, right_numbers)
}



pub fn part_1(path: &str) -> String {
    let (left_numbers, right_numbers) = shape_input(path);

    let mut sum = 0;
    for (i, left_value) in left_numbers.iter().enumerate() {
        let right_value = right_numbers[i];
        let difference = (left_value - right_value).abs();
        sum = sum + difference;
    }

    return sum.to_string()
}

pub fn part_2(path: &str) -> String {
    let (left_numbers, right_numbers) = shape_input(path);

    let mut sum = 0;

    for left_value in left_numbers.iter() {
        let mut right_count = 0;
        for right_value in right_numbers.iter() {
            if left_value == right_value {
                right_count += 1;
            }
        }
        sum = sum + left_value * right_count;
    }

    return sum.to_string()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1_part_1() {

        let test_result = part_1("day01_test.txt");
        assert_eq!(test_result, "11");

        let full_result = part_1("day01.txt");
        assert_eq!(full_result, "1834060");
    }

    #[test]
    fn test_day_1_part_2() {

        let test_result = part_2("day01_test.txt");
        assert_eq!(test_result, "31");

        let full_result = part_2("day01.txt");
        assert_eq!(full_result, "21607792");
    }
}
