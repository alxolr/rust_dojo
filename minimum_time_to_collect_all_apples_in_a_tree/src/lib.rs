use std::collections::{HashMap, HashSet};

struct Solution;

#[derive(Eq, PartialEq, Hash)]
struct Edge(i32, i32);

impl Solution {
    pub fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
        let mut visited_edges: HashSet<Edge> = HashSet::new();
        let parents_map = edges.iter().fold(HashMap::new(), |mut acc, it| {
            let leaf = it[1];
            let parent = it[0];

            acc.entry(leaf).or_insert(parent);

            acc
        });

        has_apple
            .iter()
            .enumerate()
            .filter(|it| it.1 == &true)
            .fold(0, |mut steps, (leaf, _)| {
                let mut stack = vec![leaf as i32];
                while !stack.is_empty() {
                    let curr = stack.pop().unwrap();

                    if let Some(parent) = parents_map.get(&(curr as i32)) {
                        let edge = Edge(curr as i32, *parent);

                        if !visited_edges.contains(&edge) {
                            steps += 2;
                            visited_edges.insert(edge);
                        }

                        stack.push(*parent);
                    } else {
                        if curr != 0 {
                            let mut children =
                                edges.iter().filter(|it| it[0] == curr).collect::<Vec<_>>();

                            let child = children.pop().unwrap();
                            let leaf = child[1];

                            let edge = Edge(curr as i32, leaf);

                            if !visited_edges.contains(&edge) {
                                steps += 2;
                                visited_edges.insert(edge);
                            }
                            stack.push(leaf);
                        }
                    }
                }

                steps
            })
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            // (
            //     (
            //         7,
            //         vec![
            //             vec![0, 1],
            //             vec![0, 2],
            //             vec![1, 4],
            //             vec![1, 5],
            //             vec![2, 3],
            //             vec![2, 6],
            //         ],
            //         vec![false, false, true, false, true, true, false],
            //     ),
            //     8,
            // ),
            (
                (
                    4,
                    vec![vec![0, 2], vec![0, 3], vec![1, 2]],
                    vec![false, true, false, false],
                ),
                4,
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((n, edges, has_apple), expected))| {
                let result = Solution::min_time(n, edges, has_apple);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
