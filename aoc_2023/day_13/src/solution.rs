use std::{
    cmp::Ordering,
    collections::{HashSet, VecDeque},
};

use crate::error::Result;
use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};
pub struct Solution;

type Coords = (usize, usize);

impl Solution {
    pub fn process(input: &str, expansion_size: usize) -> Result<i32> {
        let universe = expand(&universe(input), expansion_size);
        let coords = get_coords_galaxies(&universe);

        let n = universe.len();
        let m = universe[0].len();

        let sum_shortest_paths = coords
            .into_iter()
            .combinations(2)
            .par_bridge()
            .map(|x| {
                let [start, end]: [Coords; 2] = x.try_into().unwrap();

                let mut num_grid = vec![vec![0; m]; n];
                let mut queue = VecDeque::new();
                let mut visited = HashSet::new();
                let mut min_distance = i32::MIN;

                queue.push_front((start, 0));

                loop {
                    let curr = queue.pop_back().unwrap();
                    visited.insert(curr.0);

                    let ((i, j), dist) = curr;
                    num_grid[i][j] = dist;

                    min_distance = min_distance.max(dist);

                    let mut maybe_nodes = vec![];

                    if i > 0 {
                        let node = ((i - 1, j), dist + 1);

                        if !visited.contains(&node.0) {
                            maybe_nodes.push(node);
                        }
                    }

                    if i < n - 1 {
                        let node = ((i + 1, j), dist + 1);
                        if !visited.contains(&node.0) {
                            maybe_nodes.push(node);
                        }
                    }

                    if j > 0 {
                        let node = ((i, j - 1), dist + 1);
                        if !visited.contains(&node.0) {
                            maybe_nodes.push(node);
                        }
                    }

                    if j < m - 1 {
                        let node = ((i, j + 1), dist + 1);
                        if !visited.contains(&node.0) {
                            maybe_nodes.push(node);
                        }
                    }

                    let min_node = maybe_nodes.into_iter().min_by(|x, y| {
                        let (first, _) = x;
                        let (second, _) = y;

                        let f_dist = distance(first, &end);
                        let s_dist = distance(second, &end);

                        if f_dist < s_dist {
                            Ordering::Less
                        } else if f_dist == s_dist {
                            Ordering::Equal
                        } else {
                            Ordering::Greater
                        }
                    });

                    if let Some(node) = min_node {
                        queue.push_front(node);
                    }

                    if queue.is_empty() || curr.0 == end {
                        break;
                    }
                }

                min_distance
            })
            .sum::<i32>();

        Ok(sum_shortest_paths)
    }

    pub fn part_1(input: &str) -> Result<i32> {
        Solution::process(input, 2)
    }

    pub fn part_2(input: &str) -> Result<i64> {
        let progression = [Solution::process(input, 1)?, Solution::process(input, 10)?];

        let mut growing_pace = (progression[1] - progression[0]) as i64;
        let mut last = progression[1] as i64;

        for _ in 0..5 {
            growing_pace *= 10;
            last += growing_pace;
        }

        Ok(last)
    }
}

fn get_coords_galaxies(universe: &Vec<Vec<char>>) -> Vec<Coords> {
    let n = universe.len();
    let m = universe[0].len();

    let mut coords = vec![];

    for (i, item) in universe.iter().enumerate().take(n) {
        for (j, ch) in item.iter().enumerate().take(m) {
            if ch == &'#' {
                coords.push((i, j));
            }
        }
    }

    coords
}

fn expand(universe: &Vec<Vec<char>>, size: usize) -> Vec<Vec<char>> {
    let n = universe.len();
    let m = universe[0].len();

    let empty_cols = universe
        .iter()
        .enumerate()
        .filter(|(_, x)| !x.contains(&'#'))
        .map(|(idx, _)| idx)
        .collect::<Vec<_>>();

    let mut empty_rows = vec![];

    for j in 0..m {
        let mut is_empty = true;

        for item in universe.iter().take(n) {
            if item[j] == '#' {
                is_empty = false;
                break;
            }
        }

        if is_empty {
            empty_rows.push(j)
        }
    }

    let mut expanded_universe = universe.clone();

    for item in expanded_universe.iter_mut().take(n) {
        let mut row_expansion_done: usize = 0;
        for row_idx in empty_rows.iter() {
            for _ in 0..size - 1 {
                item.insert(*row_idx + row_expansion_done, '.');
                row_expansion_done += 1;
            }
        }
    }

    let m = expanded_universe[0].len();

    let mut col_expansion_done = 0;
    for col_idx in empty_cols.iter() {
        let element = vec!['.'; m];
        for _ in 0..size - 1 {
            expanded_universe.insert(col_idx + col_expansion_done, element.clone());
            col_expansion_done += 1;
        }
    }

    expanded_universe
}

fn universe(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|it| it.chars().collect::<Vec<_>>())
        .collect()
}

fn distance(x: &Coords, y: &Coords) -> f32 {
    let first = (y.0 as i32 - x.0 as i32).pow(2);
    let second = (y.1 as i32 - x.1 as i32).pow(2);

    f32::sqrt((first + second) as f32)
}
#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> String {
        r"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
        "
        .to_string()
    }

    fn expanded_example() -> String {
        r"
        ....#........
        .........#...
        #............
        .............
        .............
        ........#....
        .#...........
        ............#
        .............
        .............
        .........#...
        #....#......."
            .to_string()
    }

    #[test]
    fn test_expand_universe() -> Result<()> {
        let galaxy = universe(&example());
        let expanded_universe = universe(&expanded_example());

        assert_eq!(expand(&galaxy, 2), expanded_universe);

        Ok(())
    }

    #[test]
    fn test_solution_part_1_from_example() -> Result<()> {
        let expected = 374;
        let input = example();
        assert_eq!(Solution::part_1(&input)?, expected);

        Ok(())
    }

    #[test]
    fn test_solution_part_2_from_example() -> Result<()> {
        let expected = 1030;
        let input = example();
        assert_eq!(Solution::process(&input, 10)?, expected);

        let expected = 8410;
        let input = example();
        assert_eq!(Solution::process(&input, 100)?, expected);

        Ok(())
    }
}
