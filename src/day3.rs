#[path = "./lib.rs"]
mod lib;
use regex::Regex;



pub fn part_1(path: &str) -> String {
    let contents = lib::read_input(format!("input/{}", path));

    let sum: i32 = contents.iter()
        .map(|line| {
            let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

            re.captures_iter(line)
                .filter_map(|cap| {
                    match (cap[1].parse::<i32>().ok(), cap[2].parse::<i32>().ok()) {
                        (Some(n1), Some(n2)) => Some(n1 * n2),
                        _ => None
                    }
                })
                .sum::<i32>()
        })
        .sum();

    sum.to_string()
}




#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_day_2_part_1() {
        let test_result = part_1("day03_test.txt");
        assert_eq!(test_result, "161");

        let test_result = part_1("day03.txt");
        assert_eq!(test_result, "155955228");
    }




}
