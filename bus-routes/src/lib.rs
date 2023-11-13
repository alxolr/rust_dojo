struct Solution;

use std::collections::HashMap;

// Idea build the network of routes
// Find the shortest amount of buses to take from source to target
// the shortest path I will use BFS and backtracking

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {
            return 0;
        }
        let mut dist = [501; 10usize.pow(5) + 1];
        dist[source as usize] = 0;

        let mut count = 0;

        let mut not_final = true;
        while not_final {
            not_final = false;
            for bus in 0..routes.len() {
                let min_stop = routes[bus].iter().map(|&i| dist[i as usize]).min().unwrap();
                for &stop in &routes[bus] {
                    if dist[stop as usize] > min_stop + 1 {
                        dist[stop as usize] = min_stop + 1;
                        not_final = true;
                    }
                }
            }
        }

        if dist[target as usize] < 501 {
            dist[target as usize]
        } else {
            -1
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_buses_to_destination_ok() {
        let scenarios = vec![
            ((vec![vec![1, 2, 7], vec![3, 6, 7]], 1, 6), 2),
            (
                (
                    vec![
                        vec![7, 12],
                        vec![4, 5, 15],
                        vec![6],
                        vec![15, 19],
                        vec![9, 12, 13],
                    ],
                    15,
                    12,
                ),
                -1,
            ),
            (
                (
                    vec![
                        vec![24],
                        vec![3, 6, 11, 14, 22],
                        vec![1, 23, 24],
                        vec![0, 6, 14],
                        vec![1, 3, 8, 11, 20],
                    ],
                    20,
                    8,
                ),
                1,
            ),
            (
                (
                    vec![
                        vec![0, 1, 6, 16, 22, 23],
                        vec![14, 15, 24, 32],
                        vec![4, 10, 12, 20, 24, 28, 33],
                        vec![1, 10, 11, 19, 27, 33],
                        vec![11, 23, 25, 28],
                        vec![15, 20, 21, 23, 29],
                        vec![29],
                    ],
                    4,
                    21,
                ),
                2,
            ),
        ];

        scenarios.into_iter().enumerate().for_each(
            |(idx, ((routes, source, target), expected))| {
                let result = Solution::num_buses_to_destination(routes, source, target);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            },
        );
    }
}
