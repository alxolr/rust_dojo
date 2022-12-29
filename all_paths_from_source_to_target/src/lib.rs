use std::collections::HashMap;

pub struct Solution;

fn reverse_graph(graph: &Vec<Vec<i32>>) -> HashMap<&i32, Vec<i32>> {
    graph
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (idx, edges)| {
            edges.iter().for_each(|edge| {
                let entry = acc.entry(edge).or_insert(vec![]);
                entry.push(idx as i32);
            });

            acc
        })
}

fn find_paths(
    results: &mut Vec<Vec<i32>>,
    reversed_graph: &HashMap<&i32, Vec<i32>>,
    current: &Vec<i32>,
) {
    let last = current.last().unwrap();
    let result = current.clone();

    let maybe_parents = reversed_graph.get(last);

    if let Some(parents) = maybe_parents {
        parents.iter().for_each(|parent| {
            let mut cur = result.clone();
            cur.push(*parent as i32);

            results.push(cur.clone());
            find_paths(results, reversed_graph, &cur);
        });
    }
}

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let target = graph.len() - 1;
        let reverse_graph = reverse_graph(&graph);
        let source = 0;

        let mut results = vec![];

        let solution = vec![target as i32];
        results.push(solution.clone());

        find_paths(&mut results, &reverse_graph, &solution);

        results
            .into_iter()
            .filter(|sol| sol.contains(&source))
            .map(|item| item.into_iter().rev().collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_one() {
        let graph = vec![vec![1, 2], vec![3], vec![3], vec![]];
        let result = Solution::all_paths_source_target(graph);

        assert_eq!(result, vec![vec![0, 1, 3], vec![0, 2, 3]]);
    }

    #[test]
    fn test_case_two() {
        let graph = vec![vec![4, 3, 1], vec![3, 2, 4], vec![3], vec![4], vec![]];
        let result = Solution::all_paths_source_target(graph);

        assert_eq!(
            result,
            vec![
                vec![0, 4],
                vec![0, 1, 4],
                vec![0, 3, 4],
                vec![0, 1, 3, 4],
                vec![0, 1, 2, 3, 4]
            ]
        );
    }

    #[test]
    fn test_case_three() {
        let graph = vec![vec![4, 3, 1], vec![3, 2, 4], vec![], vec![4], vec![]];
        let result = Solution::all_paths_source_target(graph);

        assert_eq!(
            result,
            vec![vec![0, 4], vec![0, 1, 4], vec![0, 3, 4], vec![0, 1, 3, 4]]
        );
    }
}
