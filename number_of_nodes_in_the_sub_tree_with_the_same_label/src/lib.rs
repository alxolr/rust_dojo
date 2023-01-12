use std::collections::HashMap;

pub struct Solution;

fn dfs(
    node: i32,
    parent: i32,
    graph: &HashMap<i32, Vec<i32>>,
    labels: &Vec<char>,
    label_counter: &mut Vec<i32>,
    result: &mut Vec<i32>,
) {
    let idx = labels[node as usize] as u32 - 'a' as u32;
    let prev_counter = label_counter[idx as usize];
    label_counter[idx as usize] += 1;

    for child in graph.get(&node).unwrap().iter() {
        if *child != parent {
            dfs(*child, node, graph, labels, label_counter, result);
        }
    }

    result[node as usize] = label_counter[idx as usize] - prev_counter;
}

impl Solution {
    pub fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
        let labels = labels.chars().collect::<Vec<_>>();
        let mut label_counter = vec![0; 26];
        let graph = edges.iter().fold(HashMap::new(), |mut acc, it| {
            let first = acc.entry(it[0]).or_insert(vec![]);
            first.push(it[1]);

            let second = acc.entry(it[1]).or_insert(vec![]);
            second.push(it[0]);

            acc
        });

        let mut result = vec![0; n as usize];
        dfs(0, -1, &graph, &labels, &mut label_counter, &mut result);

        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (
                (
                    7,
                    vec![
                        vec![0, 1],
                        vec![0, 2],
                        vec![1, 4],
                        vec![1, 5],
                        vec![2, 3],
                        vec![2, 6],
                    ],
                    "abaedcd".to_string(),
                ),
                vec![2, 1, 1, 1, 1, 1, 1],
            ),
            (
                (
                    4,
                    vec![vec![0, 2], vec![0, 3], vec![2, 1]],
                    "aeed".to_string(),
                ),
                vec![1, 1, 2, 1],
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((n, edges, labels), expected))| {
                let result = Solution::count_sub_trees(n, edges, labels);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
