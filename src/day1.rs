use std::fs;


pub fn part_1() {
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



    let mut sum = 0;

    for (i, left_value) in left_numbers.iter().enumerate() {
        let right_value = right_numbers[i];
        let difference = (left_value - right_value).abs();
        sum = sum + difference;
    }

    println!("Part 1: {}", sum)
}

