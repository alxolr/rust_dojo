use std::collections::{HashMap, HashSet};

use crate::error::Result;
pub struct Solution;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Dir {
    E,
    W,
    N,
    S,
}

#[derive(Debug, Hash, PartialEq, Clone)]
enum Action {
    M,
    R
}

fn dp(
    memo: &mut HashMap<(isize, isize, Dir), Option<i32>>,
    row: isize,
    col: isize,
    dir: Dir,
    grid: &[Vec<char>],
    visited: &mut HashSet<(isize, isize, Dir)>,
) -> Option<i32> {
    if let Some(result) = memo.get(&(row, col, dir.clone())) {
        return *result;
    }

    if visited.contains(&(row, col, dir.clone())) {
        return None;
    }

    visited.insert((row, col, dir.clone()));

    if let Some(object) = safe_get(row, col, grid) {
        match object {
            '.' => {
                let min_cost = match dir {
                    Dir::E => {
                        let min_cost = [
                            (1, dp(memo, row, col + 1, Dir::E, grid, visited)),
                            (2000, dp(memo, row, col, Dir::W, grid, visited)),
                            (1000, dp(memo, row, col, Dir::N, grid, visited)),
                            (1000, dp(memo, row, col, Dir::S, grid, visited)),
                        ]
                        .iter()
                        .filter(|(_, result)| result.is_some())
                        .map(|(cost, result)| cost + result.unwrap())
                        .min();

                        min_cost
                    }
                    Dir::W => {
                        let min_cost = [
                            (1, dp(memo, row, col - 1, Dir::W, grid, visited)),
                            (2000, dp(memo, row, col, Dir::E, grid, visited)),
                            (1000, dp(memo, row, col, Dir::N, grid, visited)),
                            (1000, dp(memo, row, col, Dir::S, grid, visited)),
                        ]
                        .iter()
                        .filter(|(_, result)| result.is_some())
                        .map(|(cost, result)| cost + result.unwrap())
                        .min();

                        min_cost
                    }
                    Dir::N => {
                        let min_cost = [
                            (1, dp(memo, row - 1, col, Dir::N, grid, visited)),
                            (2000, dp(memo, row, col, Dir::S, grid, visited)),
                            (1000, dp(memo, row, col, Dir::E, grid, visited)),
                            (1000, dp(memo, row, col, Dir::W, grid, visited)),
                        ]
                        .iter()
                        .filter(|(_, result)| result.is_some())
                        .map(|(cost, result)| cost + result.unwrap())
                        .min();

                        min_cost
                    }
                    Dir::S => {
                        let min_cost = [
                            (1, dp(memo, row + 1, col, Dir::S, grid, visited)),
                            (2000, dp(memo, row, col, Dir::N, grid, visited)),
                            (1000, dp(memo, row, col, Dir::E, grid, visited)),
                            (1000, dp(memo, row, col, Dir::W, grid, visited)),
                        ]
                        .iter()
                        .filter(|(_, result)| result.is_some())
                        .map(|(cost, result)| cost + result.unwrap())
                        .min();

                        min_cost
                    }
                };
                memo.insert((row, col, dir.clone()), min_cost);

                min_cost
            }
            '#' => return None,
            _ => {
                return Some(0); // we arived at destination the remaining char is 'E'
            }
        }
    } else {
        None
    }
}

fn safe_get(row: isize, col: isize, grid: &[Vec<char>]) -> Option<char> {
    if row < 0 || col < 0 {
        return None;
    }
    let row = row as usize;
    let col = col as usize;
    if row >= grid.len() || col >= grid[0].len() {
        return None;
    }
    Some(grid[row][col])
}

impl Solution {
    pub fn part_1(input: &str) -> Result<i32> {
        let mut grid = load_grid(input);
        let (row, col) = find_start_point(&grid).unwrap();

        grid[row][col] = '.'; // remove the start button for the recursion to work

        let mut visited = HashSet::new();
        let mut memo = HashMap::new();
        let min_cost = dp(
            &mut memo,
            row as isize,
            col as isize,
            Dir::E,
            &grid,
            &mut visited,
        );

        // group memo by coordinates

        let mut debug_map: HashMap<(isize, isize), Vec<(Dir, Option<i32>)>> = HashMap::new();

        for ((row, col, dir), value) in memo {
            debug_map
                .entry((row, col))
                .or_default()
                .push((dir.clone(), value));
        }

        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if let Some(paths) = debug_map.get(&(row as isize, col as isize)) {
                    let min: String = paths
                        .iter()
                        .filter(|(_, value)| value.is_some())
                        .map(|(dir, _)| match dir {
                            Dir::E => '>',
                            Dir::W => '<',
                            Dir::N => '^',
                            Dir::S => 'v',
                        })
                        .collect();

                    print!("{:^4}", min);
                } else {
                    print!("{:^4}", grid[row][col])
                }
            }
            println!();
        }

        Ok(min_cost.unwrap())
    }

    pub fn part_2(_input: &str) -> Result<i32> {
        Ok(0)
    }
}

fn find_start_point(grid: &[Vec<char>]) -> Option<(usize, usize)> {
    for (i, row) in grid.iter().enumerate() {
        if let Some(j) = row.iter().position(|&c| c == 'S') {
            return Some((i, j));
        }
    }
    None
}

fn load_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
//         let input = r#"#################
// #...#...#...#..E#
// #.#.#.#.#.#.#.#.#
// #.#.#.#...#...#.#
// #.#.#.#.###.#.#.#
// #...#.#.#.....#.#
// #.#.#.#.#.#####.#
// #.#...#.#.#.....#
// #.#.#####.#.###.#
// #.#.#.......#...#
// #.#.###.#####.###
// #.#.#...#.....#.#
// #.#.#.#####.###.#
// #.#.#.........#.#
// #.#.#.#########.#
// #S#.............#
// #################"#;
//         assert_eq!(Solution::part_1(input)?, 11048);

        let input = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;
        assert_eq!(Solution::part_1(input)?, 7036);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = r#""#;
        assert_eq!(Solution::part_2(input)?, 0);

        Ok(())
    }
}
