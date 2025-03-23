use std::cmp::Reverse;
use std::collections::BinaryHeap;

const MOD: i32 = 1_000_000_007;
pub struct Solution;

impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut graph = vec![vec![]; n];

        for road in &roads {
            let (u, v, time) = (road[0] as usize, road[1] as usize, road[2]);
            graph[u].push((v, time));
            graph[v].push((u, time));
        }

        let mut dist = vec![i64::MAX; n];
        let mut ways = vec![0; n];

        dist[0] = 0;
        ways[0] = 1;

        let mut pq = BinaryHeap::new();
        pq.push(Reverse((0, 0)));

        while let Some(Reverse((cur_dist, node))) = pq.pop() {
            if cur_dist > dist[node] {
                continue;
            }

            for &(neighbor, time) in &graph[node] {
                let new_dist = cur_dist + time as i64;

                if new_dist < dist[neighbor] {
                    dist[neighbor] = new_dist;
                    ways[neighbor] = ways[node];
                    pq.push(Reverse((new_dist, neighbor)));
                } else if new_dist == dist[neighbor] {
                    ways[neighbor] = (ways[neighbor] + ways[node]) % MOD;
                }
            }
        }

        ways[n - 1]
    }
}

fn print_dp(dp: &Vec<Vec<i32>>) {
    println!();
    for row in dp {
        for col in row {
            print!("{:<10}, ", col);
        }
        println!();
    }
    println!();
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
                        vec![0, 6, 7],
                        vec![0, 1, 2],
                        vec![1, 2, 3],
                        vec![1, 3, 3],
                        vec![6, 3, 3],
                        vec![3, 5, 1],
                        vec![6, 5, 1],
                        vec![2, 5, 1],
                        vec![0, 4, 5],
                        vec![4, 6, 2],
                    ],
                ),
                4,
            ),
            ((2, vec![vec![1, 0, 8]]), 1),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((n, roads), expected))| {
                let result = Solution::count_paths(n, roads);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
