use std::collections::HashSet;
use std::collections::VecDeque;

fn traverse(
    dist: i32,
    i: isize,
    j: isize,
    grid: &mut Vec<Vec<i32>>,
    visited: &mut HashSet<(isize, isize)>,
) {
    let n = grid.len() as isize;

    if i >= 0 && j >= 0 && i < n && j < n && !visited.contains(&(i, j)) {
        visited.insert((i, j));

        let r = i as usize;
        let c = j as usize;

        let mut forward_dist = dist;
        if grid[r][c] >= 0 {
            grid[r][c] = dist;
            forward_dist += 1;
        }

        traverse(dist + 1, i + 1, j, grid, visited);
        traverse(dist + 1, i - 1, j, grid, visited);
        traverse(dist + 1, i, j + 1, grid, visited);
        traverse(dist + 1, i, j - 1, grid, visited);
    }
}

pub struct Solution;

impl Solution {
    pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut grid = grid
            .iter()
            .map(|row| {
                row.iter()
                    .map(|v| if v == &1 { -1 } else { *v })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == -1 {
                    traverse(0, i as isize, j as isize, &mut grid, &mut HashSet::new());
                    break;
                }
            }
        }

        let gr = 1;

        // mark all land with -1 and they will be starting points
        // for each -1 start incrementing the distance where we find watter or other distance

        2
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]], 2)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::max_distance(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
