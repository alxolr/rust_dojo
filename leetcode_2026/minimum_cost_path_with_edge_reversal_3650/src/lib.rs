use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Eq, PartialEq)]
struct Node(i32, i32); // (node, cost)

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.1.cmp(&self.1) // Reverse order to make BinaryHeap a min-heap by cost
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        // add the reversal edges to the edges list
        // create the djikstra grid with costs
        // span out and find the minimum solution

        let mut costs = vec![i32::MAX; n as usize]; // We plan on minimizing
        costs[0] = 0; // Starting node has cost 0
        let end = (n - 1) as usize;

        let mut visited = HashSet::<i32>::new();

        let mut bin_heap = BinaryHeap::<Node>::new();
        bin_heap.push(Node(0, 0)); // (start node, cost)

        let all_edges =
            edges
                .iter()
                .fold(HashMap::<i32, Vec<(i32, i32)>>::new(), |mut acc, edge| {
                    let [start, end, cost] = edge[..] else {
                        panic!("Invalid length");
                    };

                    let entry = acc.entry(start).or_insert(vec![]);
                    entry.push((end, cost));

                    let entry = acc.entry(end).or_insert(vec![]);
                    entry.push((start, 2 * cost));

                    acc
                });

        let empty_vec = vec![];

        while let Some(node) = bin_heap.pop() {
            let Node(node_id, cost) = node;
            
            if visited.contains(&node_id) {
                continue;
            }

            visited.insert(node_id);

            let children = all_edges.get(&node_id).unwrap_or(&empty_vec);

            for (child, child_cost) in children {
                let new_cost = cost + child_cost;
                
                if new_cost < costs[*child as usize] {
                    costs[*child as usize] = new_cost;
                    bin_heap.push(Node(*child, new_cost));
                }
            }
        }

        if costs[end] == i32::MAX {
            -1
        } else {
            costs[end]
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (
                (
                    4,
                    vec![vec![0, 1, 3], vec![3, 1, 1], vec![2, 3, 4], vec![0, 2, 2]],
                ),
                5,
            ),
            (
                (
                    4,
                    vec![vec![0, 2, 1], vec![2, 1, 1], vec![1, 3, 1], vec![2, 3, 3]],
                ),
                3,
            ),
            (
                // [[1,2,5],[2,3,18],[2,0,8],[0,1,2],[1,0,2]]
                (
                    4,
                    vec![
                        vec![1, 2, 5],
                        vec![2, 3, 18],
                        vec![2, 0, 8],
                        vec![0, 1, 2],
                        vec![1, 0, 2],
                    ],
                ),
                25,
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::min_cost(input.0, input.1);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
