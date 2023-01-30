use std::collections::HashMap;

///
/// There are n cities connected by some number of flights. You are given an array flights
/// where flights[i] = [fromi, toi, pricei] indicates that there is a flight from city fromi to city
/// to[i] with cost price[i].
///
/// You are also given three integers src, dst, and k, return the cheapest price from
/// src to dst with at most k stops. If there is no such route, return -1.

pub struct Solution;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let (edges, prices) = hash_edges(&flights);

        let start = vec![dst];
        let mut paths = vec![];

        dfs(start, &mut paths, &edges, src, dst, k);

        let maybe_solution = paths
            .into_iter()
            .map(|path| {
                path.windows(2)
                    .rev()
                    .map(|it| {
                        let tuple = (it[1], it[0]);
                        prices.get(&tuple).unwrap()
                    })
                    .sum::<i32>()
            })
            .min();

        match maybe_solution {
            Some(solution) => solution,
            None => -1,
        }
    }
}

fn dfs(
    current_path: Vec<i32>,
    paths: &mut Vec<Vec<i32>>,
    edges: &HashMap<i32, Vec<i32>>,
    src: i32,
    dst: i32,
    mut k: i32,
) {
    let last_node = current_path.last().unwrap();

    if last_node == &src {
        paths.push(current_path);

        return;
    }

    if last_node != &src && last_node != &dst {
        k -= 1;
    }

    let maybe_parents = edges.get(last_node);

    if k >= 0 {
        if let Some(parents) = maybe_parents {
            for parent in parents.iter() {
                let mut path = current_path.clone();
                path.push(*parent);
                dfs(path, paths, edges, src, dst, k)
            }
        }
    }
}

fn hash_edges(flights: &Vec<Vec<i32>>) -> (HashMap<i32, Vec<i32>>, HashMap<(i32, i32), i32>) {
    flights.iter().fold(
        (HashMap::new(), HashMap::new()),
        |(mut reversed, mut edge_price), edge| {
            let src = edge[0];
            let dst = edge[1];
            let price = edge[2];

            let entry = reversed.entry(dst).or_insert(vec![]);
            entry.push(src);

            edge_price.insert((src, dst), price);

            (reversed, edge_price)
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(
            (
                4,
                vec![
                    vec![0, 1, 100],
                    vec![1, 2, 100],
                    vec![2, 0, 100],
                    vec![1, 3, 600],
                    vec![2, 3, 200],
                ],
                0,
                3,
                1,
            ),
            700,
        )];

        scenarios.into_iter().enumerate().for_each(
            |(idx, ((n, flights, src, dst, k), expected))| {
                let result = Solution::find_cheapest_price(n, flights, src, dst, k);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            },
        );
    }
}
