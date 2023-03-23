///  There are n computers numbered from 0 to n - 1 connected by ethernet cables connections forming a network where
///  connections[i] = [ai, bi] represents a connection between computers ai and bi.
///  Any computer can reach any other computer directly or indirectly through the network.
///  You are given an initial computer network connections.
///  You can extract certain cables between two directly connected computers, and place them between any pair of disconnected
///  computers to make them directly connected.
///  Return the minimum number of times you need to do this in order to make all the computers connected. If it is not possible, return -1.
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

        UnionFind { parent }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }

        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let x_root = self.find(x);
        let y_root = self.find(y);

        if x_root == y_root {
            false
        } else {
            self.parent[y_root] = x_root;

            true
        }
    }
}


impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut uf = UnionFind::new(n as usize);
        let mut spare_cable = 0;

        for connection in connections {
            if !uf.union(connection[0] as usize, connection[1] as usize) {
                spare_cable += 1;
            }
        }

        let mut unconnected_computers = 0;

        for i in 0..n as usize {
            if uf.find(i) == i {
                unconnected_computers += 1;
            }
        }

        if spare_cable >= unconnected_computers - 1 {
            unconnected_computers - 1
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ((4, vec![vec![0, 1], vec![0, 2], vec![1, 2]]), 1),
            (
                (
                    6,
                    vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2], vec![1, 3]],
                ),
                2,
            ),
            (
                (6, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2]]),
                -1,
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((n, connections), expected))| {
                let result = Solution::make_connected(n, connections);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
