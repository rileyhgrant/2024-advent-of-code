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

pub fn part_2(path: &str) -> String {
    let contents = lib::read_input(format!("input/{}", path));

    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    contents.iter().for_each(|line| {
        let splits = line.split_once('-').unwrap();
        let first = splits.0;
        let second = splits.1;

        graph.entry(first).or_insert_with(Vec::new).push(second);
        graph.entry(second).or_insert_with(Vec::new).push(first);
    });

    let values: Vec<_> = graph
        .iter()
        .map(|(key, val)| {
            let mut counts: HashMap<&str, Num> = HashMap::new();

            let mut curr_all = val.clone();
            curr_all.push(key);
            curr_all.sort();
            for s in curr_all.iter() {
                *counts.entry(s).or_insert(0) += 1;
            }

            val.iter().for_each(|v| {
                let mut this_all = graph.get(v).unwrap().clone();
                this_all.push(v);
                this_all.sort();
                for s in this_all.iter() {
                    *counts.entry(s).or_insert(0) += 1;
                }
            });

            let n = counts.len();
            for size in (2..=n).rev() {
                let nodes_with_count: Vec<&str> = counts
                    .iter()
                    .filter(|(_k, &v)| v >= size as Num) 
                    .map(|(k, _v)| *k)
                    .collect();

                if nodes_with_count.len() == size {
                    return nodes_with_count; 
                }
            }

            vec![""]
        })
        .collect();

    let mut longest: Vec<&str> = values.iter().max_by_key(|list| list.len()).unwrap().clone();
    longest.sort();
    let longest_string = longest.join(",");


    longest_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_23_part_1() {
        let test_result = part_1("day23_test.txt");
        assert_eq!(test_result, "7");

        let test_result = part_1("day23.txt");
        assert_eq!(test_result, "1302");
    }

    #[test]
    fn test_day_23_part_2() {
        let test_result = part_2("day23_test.txt");
        assert_eq!(test_result, "co,de,ka,ta");

        let test_result = part_2("day23.txt");
        assert_eq!(test_result, "cb,df,fo,ho,kk,nw,ox,pq,rt,sf,tq,wi,xz");
    }
}
