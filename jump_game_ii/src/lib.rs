use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let len = nums.len();

        let graph = nums
            .iter()
            .enumerate()
            .fold(vec![vec![]; len], |mut acc, it| {
                let items = it.0..(it.0 + *it.1 as usize);
                acc[it.0] = items.map(|x| x as i32 + 1).collect::<Vec<_>>();

                acc
            });

        let parents_graph =
            graph
                .iter()
                .enumerate()
                .fold(HashMap::new(), |mut acc, (idx, childs)| {
                    for child in childs.iter() {
                        let entry = acc.entry(*child).or_insert(vec![]);

                        entry.push(idx as i32)
                    }

                    acc
                });

        let mut dp = vec![i32::MAX; len];
        let dst = (len - 1) as i32;
        dp[dst as usize] = 0;

        shortest_distance(dst, &parents_graph, &mut dp, &mut HashSet::<i32>::new());

        dp[0]
    }
}

fn shortest_distance(
    node: i32,
    parents_graph: &HashMap<i32, Vec<i32>>,
    dp: &mut Vec<i32>,
    visited: &mut HashSet<i32>,
) {
    visited.insert(node);
    let parents = parents_graph.get(&node);

    if let Some(parents) = parents {
        for parent in parents.iter() {
            let parent_idx = *parent as usize;

            let distance = match dp[node as usize] {
                i32::MAX => 1,
                _ => dp[node as usize] + 1,
            };

            dp[parent_idx] = dp[parent_idx].min(distance);

            if !visited.contains(parent) {
                shortest_distance(*parent, parents_graph, dp, visited);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec![2, 3, 1, 1, 4], 2),
            (vec![2, 3, 0, 1, 4], 2),
            (
                vec![
                    5, 6, 4, 4, 6, 9, 4, 4, 7, 4, 4, 8, 2, 6, 8, 1, 5, 9, 6, 5, 2, 7, 9, 7, 9, 6,
                    9, 4, 1, 6, 8, 8, 4, 4, 2, 0, 3, 8, 5,
                ],
                5,
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::jump(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
