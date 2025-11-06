use std::collections::{BTreeSet, VecDeque};

impl Solution {
    pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let c = c as usize;
        let mut graph = vec![vec![]; c + 1];
        for conn in connections {
            let (u, v) = (conn[0] as usize, conn[1] as usize);
            graph[u].push(v);
            graph[v].push(u);
        }

        let mut visited = vec![false; c + 1];
        let mut station_to_grid = vec![0; c + 1];
        let mut grids: Vec<BTreeSet<(i32, i32)>> = vec![];
        let mut grid_id = 0;

        for s in 1..=c {
            if !visited[s] {
                let mut q = VecDeque::new();
                let mut grid = BTreeSet::new();
                q.push_back(s);
                visited[s] = true;
                while let Some(u) = q.pop_front() {
                    station_to_grid[u] = grid_id;
                    grid.insert((1, u as i32));
                    for &v in &graph[u] {
                        if !visited[v] {
                            visited[v] = true;
                            q.push_back(v);
                        }
                    }
                }
                grids.push(grid);
                grid_id += 1;
            }
        }

        let mut result = vec![];
        for q in queries {
            let t = q[0];
            let x = q[1] as usize;
            let id = station_to_grid[x];
            if t == 1 {
                if grids[id].contains(&(1, x as i32)) {
                    result.push(x as i32);
                } else if let Some(&(st, sid)) = grids[id].iter().next() {
                    result.push(if st == 1 { sid } else { -1 });
                } else {
                    result.push(-1);
                }
            } else {
                if grids[id].remove(&(1, x as i32)) {
                    grids[id].insert((2, x as i32));
                }
            }
        }
        result
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
                    5,
                    vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]],
                    vec![vec![1, 3], vec![2, 1], vec![1, 1], vec![2, 2], vec![1, 2]],
                ),
                vec![3, 2, 3],
            ),
            (
                (3, vec![], vec![vec![1, 1], vec![2, 1], vec![1, 1]]),
                vec![1, -1],
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::process_queries(input.0, input.1, input.2);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
