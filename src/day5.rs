#[path = "./lib.rs"]
mod lib;

use std::fs;



#[derive(Debug)]
struct Rule {
    pre: u32,
    post: u32,
}


impl Rule {
    fn new(pattern: &str) -> Self {
        let (pre, post) = pattern.split_once('|').unwrap();
        Rule {
            pre: pre.parse().unwrap(),
            post: post.parse().unwrap(),
        }
    }

    fn contains(&self, to_check: u32) -> bool {
        to_check == self.pre || to_check == self.post
    }

    fn as_post_any_rules_broken(&self, post: u32, pre_candidates: &[u32]) -> bool {
        if post == self.pre {
            return pre_candidates.contains(&self.post)
        }
        false 
    }

    fn check_passes_rules(&self, post: u32, pre_candidates: &[u32]) -> bool {
        if self.contains(post) {
            return !self.as_post_any_rules_broken(post, pre_candidates);
        }
        true
    }

}



fn shape_input(filepath: &str) -> (Vec<Rule>, Vec<String>) {
    let contents = fs::read_to_string(format!("./input/{}", filepath)).expect("Should have been able to read the file");
    let (rules_text, updates_text) = contents.split_once("\n\n").unwrap(); 
    let rules_vec = lib::string_to_vec_string(rules_text.to_string());
    let updates_vec = lib::string_to_vec_string(updates_text.to_string());

    let rules: Vec<Rule> = rules_vec.iter()
        .map(|rule_string| Rule::new(rule_string))
        .collect();

    (rules, updates_vec)
}



pub fn part_1(path: &str) -> String {
    let (rules, updates) = shape_input(path);

    updates.iter()
        .filter(|line| {
            let numbers: Vec<u32> = line.split(',')
                .map(|n| n.trim().parse().unwrap())
                .collect();

            for (i, &num) in numbers.iter().enumerate() {
                let previous = &numbers[..i];
    
                if rules.iter().any(|rule| !rule.check_passes_rules(num, previous)) {
                    return false;
                }
            }
            true 
        })
        .map(|line| {
            let numbers: Vec<u32> = line.split(',')
                .map(|n| n.trim().parse().unwrap())
                .collect();
            let middle = numbers.len() / 2;
            numbers[middle]
        })
        .sum::<u32>()
        .to_string()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rules_check() {
        let rule = Rule::new("10|15");
        assert_eq!(rule.contains(10), true);
        assert_eq!(rule.contains(15), true);
        assert_eq!(rule.contains(20), false);
    }

    #[test] 
    fn test_rules_as_post() {
        let rule = Rule::new("10|15");
        assert_eq!(rule.as_post_any_rules_broken(15, &[9, 10, 11]), false);
        assert_eq!(rule.as_post_any_rules_broken(15, &[9, 11, 12]), false);
        assert_eq!(rule.as_post_any_rules_broken(10, &[14, 15, 16]), true);
        assert_eq!(rule.as_post_any_rules_broken(10, &[14, 16, 17]), false);
        assert_eq!(rule.as_post_any_rules_broken(14, &[9, 10, 11]), false);
        assert_eq!(rule.as_post_any_rules_broken(14, &[9, 11, 12]), false);
    }

    #[test] 
    fn test_passes_rules() {
        let rule = Rule::new("10|15");
        assert_eq!(rule.check_passes_rules(15, &[9, 10, 11]), true);
        assert_eq!(rule.check_passes_rules(15, &[9, 11, 12]), true);
        assert_eq!(rule.check_passes_rules(10, &[14, 15, 16]), false);
        assert_eq!(rule.check_passes_rules(10, &[14, 16, 17]), true);
        assert_eq!(rule.check_passes_rules(14, &[9, 10, 11]), true);
        assert_eq!(rule.check_passes_rules(14, &[9, 11, 12]), true);
    }


    #[test]
    fn test_day_5_part_1() {
        let test_result = part_1("day05_test.txt");
        assert_eq!(test_result, "143");

        let test_result = part_1("day05.txt");
        assert_eq!(test_result, "4905");
    }
}
