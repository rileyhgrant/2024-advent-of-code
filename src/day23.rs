use std::collections::{HashMap, HashSet, VecDeque};

#[path = "./lib.rs"]
mod lib;

type Num = i64;

pub fn part_1(path: &str) -> String {
    let contents = lib::read_input(format!("input/{}", path));

    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    let _pairs: Vec<(&str, &str)> = contents
        .iter()
        .map(|line| {
            let splits = line.split_once('-').unwrap();
            let first = splits.0;
            let second = splits.1;

            graph.entry(first).or_insert_with(Vec::new).push(second);
            graph.entry(second).or_insert_with(Vec::new).push(first);

            (first, second)
        })
        .collect();

    let mut triplets: HashSet<Vec<&str>> = HashSet::new();

    for (node, neighbors) in graph.iter() {
        let mut queue: VecDeque<(&str, Num, Vec<&str>)> = VecDeque::new();
        neighbors.iter().for_each(|neighbor| {
            queue.push_front((neighbor, 1, vec![neighbor]));
        });

        while let Some(curr) = queue.pop_back() {
            let curr_neighbor = curr.0;
            let curr_value = curr.1;
            let curr_history = curr.2;

            graph
                .get(&curr_neighbor)
                .unwrap()
                .iter()
                .for_each(|new_neighb| {
                    if curr_value == 1 {
                        if new_neighb != node {
                            let mut new_history = curr_history.clone();
                            new_history.push(new_neighb);
                            queue.push_front((new_neighb, 2, new_history));
                        }
                    } else if curr_value == 2 {
                        if new_neighb == node {
                            let mut new_history = curr_history.clone();
                            new_history.push(new_neighb);
                            new_history.sort();
                            triplets.insert(new_history);
                        }
                    }
                });
        }
    }

    let result = triplets
        .iter()
        .filter_map(|triplet| {
            if triplet.iter().any(|s| s.starts_with('t')) {
                return Some(triplet);
            } else {
                return None;
            }
        })
        .count();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_23_part_1() {
        let test_result = part_1("day23_test.txt");
        assert_eq!(test_result, "7");

        let test_result = part_1("day23.txt");
        assert_eq!(test_result, "dunno");
    }
}
