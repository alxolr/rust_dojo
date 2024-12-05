use std::collections::{HashMap, VecDeque};

use crate::error::Result;
pub struct Solution;

impl Solution {
    fn common<L>(input: &str, logic: L) -> Result<i32>
    where
        L: Fn(&Vec<i32>, &Vec<i32>) -> i32,
    {
        let (queues, prints) = parse_queues_and_prints(input);

        let (direct_graph, indirect_graph) = queues.iter().fold(
            (HashMap::new(), HashMap::new()),
            |(mut direct, mut indirect), item| {
                direct.entry(item[0]).or_insert_with(Vec::new).push(item[1]);

                indirect
                    .entry(item[1])
                    .or_insert_with(Vec::new)
                    .push(item[0]);

                (direct, indirect)
            },
        );

        let result = prints
            .iter()
            .map(|print| {
                let topological_order = topological_order(&direct_graph, &indirect_graph, print);

                logic(&topological_order, print)
            })
            .sum::<i32>();

        Ok(result)
    }

    pub fn part_1(input: &str) -> Result<i32> {
        Self::common(input, |topo_order, print| {
            if topo_order == print {
                let mid = print.len() / 2;
                print[mid]
            } else {
                0
            }
        })
    }

    pub fn part_2(input: &str) -> Result<i32> {
        Self::common(input, |topo_order, print| {
            if topo_order != print {
                let mid = print.len() / 2;
                topo_order[mid]
            } else {
                0
            }
        })
    }
}

fn topological_order(
    direct_graph: &HashMap<i32, Vec<i32>>,
    indirect_graph: &HashMap<i32, Vec<i32>>,
    pages: &Vec<i32>,
) -> Vec<i32> {
    let mut sorted_pages = Vec::with_capacity(pages.len());
    let mut dependency_count = HashMap::new();

    // Count dependencies for each page
    for &page in pages {
        let count = indirect_graph.get(&page).map_or(0, |parents| {
            parents
                .iter()
                .filter(|&&parent| pages.contains(&parent))
                .count()
        });
        dependency_count.insert(page, count);
    }

    let mut queue: VecDeque<_> = dependency_count
        .iter()
        .filter(|&(_, &count)| count == 0)
        .map(|(&page, _)| page)
        .collect();

    while let Some(page) = queue.pop_front() {
        sorted_pages.push(page);

        if let Some(children) = direct_graph.get(&page) {
            for &child in children.iter().filter(|&&child| pages.contains(&child)) {
                if let Some(count) = dependency_count.get_mut(&child) {
                    *count -= 1;
                    if *count == 0 {
                        queue.push_back(child);
                    }
                }
            }
        }
    }

    sorted_pages
}

fn parse_queues_and_prints(input: &str) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let sections: Vec<&str> = input.split("\n\n").collect();
    if sections.len() != 2 {
        panic!("invalid input");
    }

    let queues_section = sections[0];
    let prints_section = sections[1];

    let order_queues: Vec<Vec<i32>> = queues_section
        .lines()
        .map(|line| {
            line.split('|')
                .flat_map(|page| page.parse::<i32>())
                .collect()
        })
        .collect();

    let print_jobs: Vec<Vec<i32>> = prints_section
        .lines()
        .map(|line| {
            line.split(',')
                .flat_map(|page| page.parse::<i32>())
                .collect()
        })
        .collect();

    (order_queues, print_jobs)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() -> Result<()> {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
        assert_eq!(Solution::part_1(input)?, 143);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
        assert_eq!(Solution::part_2(input)?, 123);

        Ok(())
    }
}
