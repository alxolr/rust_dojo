use rayon::prelude::*;
use std::collections::{BTreeSet, HashMap, HashSet};

use crate::error::Result;
pub struct Solution;

impl Solution {
    pub fn part_1(input: &str) -> Result<usize> {
        let connections = load_connections(input);

        // Storing all connections in a graph form {source: [destination,...]}, {destination: [source, ...]}
        let graph = connections
            .into_iter()
            .fold(HashMap::new(), |mut acc, (left, right)| {
                acc.entry(left).or_insert(vec![]).push(right);
                acc.entry(right).or_insert(vec![]).push(left);

                acc
            });

        let triplets = graph
            .keys()
            .filter(|key| key.starts_with('t'))
            .flat_map(|node| {
                let collected = [*node].into_iter().collect();
                let mut memo = HashMap::new();
                let results = dfs(&mut memo, node, &graph, collected, 3);

                results
            })
            .collect::<HashSet<_>>();

        Ok(triplets.len())
    }

    pub fn part_2(input: &str) -> Result<String> {
        let connections = load_connections(input);

        // Storing all connections in a graph form {source: [destination,...]}, {destination: [source, ...]}
        let graph = connections
            .into_iter()
            .fold(HashMap::new(), |mut acc, (left, right)| {
                acc.entry(left).or_insert(vec![]).push(right);
                acc.entry(right).or_insert(vec![]).push(left);

                acc
            });

        let nodes = graph.keys().collect::<Vec<_>>();
        let memo = HashMap::new();

        let sets = nodes
            .into_par_iter()
            .flat_map(|node| {
                let collected = [*node].into_iter().collect();

                let results = dfs(&mut memo.clone(), node, &graph, collected, 12);

                results
            })
            .collect::<HashSet<_>>();

        let password = sets
            .into_iter()
            .next()
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>();

        Ok(password.join(","))
    }
}

fn dfs<'a>(
    memo: &mut HashMap<(String, BTreeSet<&'a str>), Vec<BTreeSet<&'a str>>>,
    node: &str,
    graph: &HashMap<&'a str, Vec<&'a str>>,
    collected: BTreeSet<&'a str>,
    depth: usize,
) -> Vec<BTreeSet<&'a str>> {
    if collected.len() == depth {
        return vec![collected.clone()];
    }

    let key = (node.to_string(), collected.clone());
    if let Some(result) = memo.get(&key) {
        return result.clone();
    }

    let children = graph.get(node).unwrap();
    children
        .into_iter()
        .flat_map(|child| {
            let grand_children = graph.get(child).unwrap();

            if collected.iter().all(|item| grand_children.contains(item)) {
                let mut collected = collected.clone();
                collected.insert(child);
                let results = dfs(memo, child, graph, collected, depth);

                memo.insert(key.clone(), results.clone());
                results
            } else {
                vec![]
            }
        })
        .collect()
}

fn load_connections<'a>(input: &'a str) -> Vec<(&'a str, &'a str)> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split('-');
            let left = split.next().unwrap();
            let right = split.next().unwrap();

            (left, right)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() -> Result<()> {
        let input = r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"#;
        assert_eq!(Solution::part_1(input)?, 7);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = r#"ka-co
ta-co
de-co
ta-ka
de-ta
ka-de"#;
        assert_eq!(Solution::part_2(input)?, "co,de,ka,ta");

        Ok(())
    }
}
