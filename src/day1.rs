#[path = "./lib.rs"]
mod lib;

fn shape_input(filepath: &str) -> (Vec<i32>, Vec<i32>) {
    let contents = lib::read_input(format!("input/{}", filepath));

    let (mut left, mut right): (Vec<i32>, Vec<i32>) = contents
        .iter()
        .filter_map(|line| {
            let nums: Vec<i32> = line.split_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect();
            match nums[..] {
                [left, right] => Some((left, right)),
                _ => None
            }   
        })
        .unzip();

    left.sort();
    right.sort();
    (left, right)
}



pub fn part_1(path: &str) -> String {
    let (left, right) = shape_input(path);
    
    let sum: i32 = left.iter()
        .zip(&right)
        .map(|(l, r)| (l - r).abs())
        .sum();

    return sum.to_string()
}



pub fn part_2(path: &str) -> String {
    let (left, right) = shape_input(path);

    let sum: i32 = left.iter()
        .map(|l_value| {
            let r_count: i32 = right.iter()
                .filter(|&r_value| r_value == l_value)
                .count() as i32;
            l_value * r_count
        })
        .sum();

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
