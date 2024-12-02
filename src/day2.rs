#[path = "./lib.rs"]
mod lib;



fn check_criteria(vec: Vec<i32>) -> bool {
    vec.windows(2).all(|pair|{
        ((pair[1] - pair[0]) >= 1) &&
        ((pair[1] - pair[0]) <= 3)
    })
}



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

            let criteria = check_criteria(nums.clone());

            if criteria {
                Some(nums)
            } else {
                None
            }
        })
        .count();

    sum.to_string()
}



pub fn part_2(path: &str) -> String {
    let contents = lib::read_input(format!("input/{}", path));

    let sum = contents.iter()
        .filter_map(|line| {
            let mut nums: Vec<i32> = line.split_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect();

            if nums.first() > nums.last() {
                nums.reverse()
            }

            let possibles: Vec<Vec<i32>> = (0..nums.len())
                .map(|i| {
                    let mut possible = nums.clone();
                    possible.remove(i);
                    possible
                })
                .collect();

            let one_pass = possibles.iter()
                .any(|vec| {
                    check_criteria(vec.clone())
                });

            if one_pass {
                Some(possibles)
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

    #[test]
    fn test_day_2_part_2() {
        let test_result = part_2("day02_test.txt");
        assert_eq!(test_result, "4");

        let test_result = part_2("day02.txt");
        assert_eq!(test_result, "621");
    }

}
