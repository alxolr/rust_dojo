use std::collections::HashSet;

pub struct Solution;

type Coords = (usize, usize);

const WATER: i32 = 0;
const SOIL: i32 = 1;

impl Solution {
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        // Store the coordinates of the distinct islands from grid2
        let mut islands: Vec<Vec<Coords>> = vec![];

        // Store the already visited coordinates to avoid recursions
        let mut visited: HashSet<Coords> = HashSet::new();

        for i in 0..grid2.len() {
            for j in 0..grid2[0].len() {
                if grid2[i][j] == SOIL && !visited.contains(&(i, j)) {
                    let mut island = vec![];
                    Self::dfs(i, j, &mut island, &mut visited, &grid2);

                    islands.push(island);
                }
            }
        }

        let count = islands
            .into_iter()
            .filter(|island| island.into_iter().all(|(i, j)| grid1[*i][*j] == SOIL))
            .count();

        count as i32
    }

    fn dfs(
        i: usize,
        j: usize,
        island: &mut Vec<Coords>,
        visited: &mut HashSet<Coords>,
        grid: &Vec<Vec<i32>>,
    ) {
        if i >= grid.len() || j >= grid[0].len() {
            return;
        }

        if grid[i][j] == WATER {
            return;
        } else {
            if !visited.contains(&(i, j)) {
                island.push((i, j));
                visited.insert((i, j));

                if i.checked_sub(1).is_some() {
                    Self::dfs(i - 1, j, island, visited, grid);
                }

                Self::dfs(i + 1, j, island, visited, grid);

                if j.checked_sub(1).is_some() {
                    Self::dfs(i, j - 1, island, visited, grid);
                }

                Self::dfs(i, j + 1, island, visited, grid);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (
                (
                    vec![
                        vec![1, 1, 1, 0, 0],
                        vec![0, 1, 1, 1, 1],
                        vec![0, 0, 0, 0, 0],
                        vec![1, 0, 0, 0, 0],
                        vec![1, 1, 0, 1, 1],
                    ],
                    vec![
                        vec![1, 1, 1, 0, 0],
                        vec![0, 0, 1, 1, 1],
                        vec![0, 1, 0, 0, 0],
                        vec![1, 0, 1, 1, 0],
                        vec![0, 1, 0, 1, 0],
                    ],
                ),
                3,
            ),
            (
                (
                    vec![
                        vec![1, 0, 1, 0, 1],
                        vec![1, 1, 1, 1, 1],
                        vec![0, 0, 0, 0, 0],
                        vec![1, 1, 1, 1, 1],
                        vec![1, 0, 1, 0, 1],
                    ],
                    vec![
                        vec![0, 0, 0, 0, 0],
                        vec![1, 1, 1, 1, 1],
                        vec![0, 1, 0, 1, 0],
                        vec![0, 1, 0, 1, 0],
                        vec![1, 0, 0, 0, 1],
                    ],
                ),
                2,
            ),
            (
                (
                    vec![
                        vec![1, 1, 0, 1, 0, 1, 1, 1],
                        vec![0, 1, 1, 1, 1, 0, 1, 1],
                        vec![1, 1, 1, 1, 0, 1, 0, 1],
                        vec![1, 1, 1, 0, 1, 1, 1, 1],
                        vec![1, 1, 1, 1, 0, 1, 1, 0],
                        vec![1, 1, 1, 1, 0, 1, 0, 0],
                        vec![1, 0, 1, 1, 1, 1, 0, 0],
                        vec![1, 0, 0, 1, 1, 1, 1, 1],
                    ],
                    vec![
                        vec![1, 1, 1, 1, 0, 0, 0, 0],
                        vec![0, 1, 1, 1, 0, 0, 1, 1],
                        vec![1, 1, 1, 1, 0, 1, 1, 1],
                        vec![1, 1, 0, 1, 1, 1, 1, 0],
                        vec![1, 0, 0, 1, 0, 1, 1, 1],
                        vec![1, 1, 0, 1, 1, 1, 1, 0],
                        vec![1, 0, 1, 0, 1, 1, 1, 1],
                        vec![1, 1, 1, 1, 1, 0, 1, 1],
                    ],
                ),
                0,
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((grid1, grid2), expected))| {
                let result = Solution::count_sub_islands(grid1, grid2);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
