/**
 * You are given a tree (i.e. a connected, undirected graph that has no cycles) rooted at node 0
 * consisting of n nodes numbered from 0 to n - 1.
 *
 * The tree is represented by a 0-indexed array parent of size n, where parent[i] is the parent of node i.
 * Since node 0 is the root, parent[0] == -1.
 *
 * You are also given a string s of length n, where s[i] is the character assigned to node i.
 *
 * Return the length of the longest path in the tree such that no pair of adjacent nodes on the path
 * have the same character assigned to them.
 */
use std::collections::HashMap;

struct Solution;

fn dfs(
    node: &i32,
    childrens: &HashMap<i32, Vec<i32>>,
    labels: &Vec<char>,
    longest_path: &mut i32,
) -> i32 {
    if !childrens.contains_key(node) {
        return 1;
    }

    let mut longest_chain = 0;
    let mut second_longest_chain = 0;

    for child in childrens.get(node).unwrap().iter() {
        let longest_chain_starting_from_child = dfs(child, childrens, labels, longest_path);

        if labels[*node as usize] == labels[*child as usize] {
            continue;
        }

        if longest_chain_starting_from_child > longest_chain {
            second_longest_chain = longest_chain;
            longest_chain = longest_chain_starting_from_child;
        } else if longest_chain_starting_from_child > second_longest_chain {
            second_longest_chain = longest_chain_starting_from_child;
        }
    }

    let local_max = longest_chain + second_longest_chain + 1;

    *longest_path = (*longest_path).max(local_max);

    longest_chain + 1
}

impl Solution {
    pub fn longest_path(parent: Vec<i32>, s: String) -> i32 {
        let mut longest_path = 1;
        let labels = s.chars().collect::<Vec<_>>();

        let mut children = HashMap::new();

        for i in 1..parent.len() {
            children.entry(parent[i]).or_insert(vec![]).push(i as i32);
        }

        dfs(&0, &children, &labels, &mut longest_path);

        longest_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ((vec![-1, 0, 0, 1, 1, 2], "abacbe".to_string()), 3),
            ((vec![-1, 0, 0, 0], "aabc".to_string()), 3),
            ((vec![-1], "z".to_string()), 1),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((parent, s), expected))| {
                let result = Solution::longest_path(parent, s);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
