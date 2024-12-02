#[path = "./lib.rs"]
mod lib;


pub fn part_1(path: &str) -> String {
    let contents = lib::read_input(format!("input/{}", path));

    let sum = contents.iter()
        .filter_map(|line| {
            let mut nums: Vec<i32> = line.split_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect();

            if nums.first() > nums.last() {
                nums.reverse()
            }

            let criteria = nums.windows(2)
                .all(|pair| {
                    ((pair[1] - pair[0]) >= 0) &&
                    ((pair[0] - pair[1]).abs() >= 1) &&  
                    ((pair[0] - pair[1]).abs() <= 3)
                });

            if criteria {
                Some(nums)
            } else {
                None
            }
        })
        .count();

    sum.to_string()
}



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_day_2_part_1() {

        let test_result = part_1("day02_test.txt");
        assert_eq!(test_result, "2");

        let test_result = part_1("day02.txt");
        assert_eq!(test_result, "591");

    }

}
