use std::fs;

fn read_input() -> (Vec<i32>, Vec<i32>) {
    let contents = fs::read_to_string("input/day1.txt").expect("Should have been able to read the file");

    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();

    for line in contents.lines() {
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

pub fn part_1() {
    let (left_numbers, right_numbers) = read_input();

    let mut sum = 0;
    for (i, left_value) in left_numbers.iter().enumerate() {
        let right_value = right_numbers[i];
        let difference = (left_value - right_value).abs();
        sum = sum + difference;
    }

    println!("Part 1: {}", sum)
}

pub fn part_2() {
    let (left_numbers, right_numbers) = read_input();

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

    println!("Part 2: {}", sum)

}

