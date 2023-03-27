///
/// There are n cities numbered from 0 to n - 1 and n - 1 roads such that there is only one way to travel between two different cities (this network form a tree). Last year, The ministry of transport decided to orient the roads in one direction because they are too narrow.
/// Roads are represented by connections where connections[i] = [ai, bi] represents a road from city ai to city bi.
/// This year, there will be a big event in the capital (city 0), and many people want to travel to this city.
/// Your task consists of reorienting some roads such that each city can visit the city 0. Return the minimum number of edges changed.
/// It's guaranteed that each city can reach city 0 after reorder.
///

struct Solution;

fn dfs(node: usize, visited: &mut Vec<bool>, count: &mut i32, graph: &Vec<Vec<(usize, bool)>>) {
    visited[node] = true;

    for (next, is_forward) in &graph[node] {
        if !visited[*next] {
            if !is_forward {
                *count += 1;
            }
            dfs(*next, visited, count, graph);
        }
    }
}

impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut graph = vec![vec![]; n as usize];
        let mut visited = vec![false; n as usize];
        let mut count = 0;

        for connection in connections {
            graph[connection[0] as usize].push((connection[1] as usize, true));
            graph[connection[1] as usize].push((connection[0] as usize, false));
        }

        dfs(0, &mut visited, &mut count, &graph);

        count
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
                    6,
                    vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]],
                ),
                3,
            ),
            ((5, vec![vec![1, 0], vec![1, 2], vec![3, 2], vec![3, 4]]), 2),
            ((3, vec![vec![1, 0], vec![2, 0]]), 0),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((n, connections), expected))| {
                let result = Solution::min_reorder(n, connections);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
