use std::collections::{HashMap, HashSet};

/// Find Closest Node to Given Two Nodes
///
/// You are given a directed graph of n nodes numbered from 0 to n - 1,
/// where each node has at most one outgoing edge.
///
/// The graph is represented with a given 0-indexed array edges of size n,
/// indicating that there is a directed edge from node i to node edges[i].
/// If there is no outgoing edge from i, then edges[i] == -1.
///
/// You are also given two integers node1 and node2.
/// Return the index of the node that can be reached from both node1 and node2,
/// such that the maximum between the distance from node1 to that node, and from node2 to that node is minimized.
/// If there are multiple answers, return the node with the smallest index, and if no possible answer exists, return -1.
///
/// Note that edges may contain cycles.

pub struct Solution;

fn dfs(
    node: i32,
    visited: &mut HashSet<i32>,
    edge_map: &HashMap<i32, i32>,
    distance: i32,
    distances: &mut Vec<i32>,
) {
    distances[node as usize] = distance;
    visited.insert(node);

    let maybe_next_node = edge_map.get(&node);

    if let Some(next_node) = maybe_next_node {
        if next_node != &-1 && !visited.contains(next_node) {
            dfs(*next_node, visited, edge_map, distance + 1, distances);
        }
    }
}

fn calculate_distances(nodes: usize, node: i32, edges_map: &HashMap<i32, i32>) -> Vec<i32> {
    let mut distances = vec![-1; nodes];
    distances[node as usize] = 0;
    dfs(node, &mut HashSet::new(), edges_map, 0, &mut distances);

    distances
}

impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        // first thing you do the edges_map to have O(1) time during traversal
        let nodes = edges.len();
        let edges_map = edges.into_iter().enumerate().fold(
            HashMap::new(),
            |mut acc: HashMap<i32, i32>, (idx, edge)| {
                acc.entry(idx as i32).or_insert(edge);

                acc
            },
        );

        let distances_one = calculate_distances(nodes, node1, &edges_map);
        let distances_two = calculate_distances(nodes, node2, &edges_map);

        let maybe_solution = distances_one
            .into_iter()
            .zip(distances_two.into_iter())
            .enumerate()
            .filter(|(_, (x, y))| *x >= 0 && *y >= 0)
            .map(|(idx, (x, y))| (idx, x.max(y)))
            .min_by(|x, y| x.1.cmp(&y.1));

        match maybe_solution {
            Some((closest_node, _)) => closest_node as i32,
            _ => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ((vec![2, 2, 3, -1], 0, 1), 2),
            ((vec![1, 2, -1], 0, 2), 2),
            ((vec![4, 3, 0, 5, 3, -1], 4, 0), 4),
            ((vec![4, 4, 8, -1, 9, 8, 4, 4, 1, 1], 5, 6), 1),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((edges, node1, node2), expected))| {
                let result = Solution::closest_meeting_node(edges, node1, node2);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
