use std::collections::HashMap;

/// You are given an integer n.
/// There is an undirected graph with n nodes, numbered from 0 to n - 1.
/// You are given a 2D integer array edges where edges[i] = [ai, bi] denotes that there exists
/// an undirected edge connecting nodes ai and bi.
///
/// Return the number of pairs of different nodes that are unreachable from each other.

struct Solution;

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        for i in 0..n {
            parent[i] = i;
        }
        Self { parent }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let x_root = self.find(x);
        let y_root = self.find(y);

        if x_root != y_root {
            self.parent[y_root] = x_root;
        }
    }
}

impl Solution {
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;
        let mut uf = UnionFind::new(n);

        for edge in edges {
            let (a, b) = (edge[0] as usize, edge[1] as usize);
            uf.union(a, b);
        }

        let groups = uf.parent.iter().fold(HashMap::new(), |mut acc, &x| {
            *acc.entry(x).or_insert(0) += 1;

            acc
        });

        if groups.len() == 1 {
            return 0 as i64;
        }

        let mut result = 0;

        let items = groups.values().collect::<Vec<_>>();

        let mut idx = 0;
        while idx < items.len() - 1 {
            let first = items[idx];
            let rest = &items[idx + 1..];
            let rest_sum = rest.iter().map(|x| **x).sum::<i32>();

            result += *first * rest_sum;
            idx += 1;
        }

        result as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            // ((3, vec![vec![0, 1], vec![0, 2], vec![1, 2]]), 0),
            // (
            //     (
            //         7,
            //         vec![vec![0, 2], vec![0, 5], vec![2, 4], vec![1, 6], vec![5, 4]],
            //     ),
            //     14,
            // ),
            ((5, vec![vec![1, 0], vec![3, 1], vec![0, 4], vec![2, 1]]), 0),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((n, edges), expected))| {
                let result = Solution::count_pairs(n, edges);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
