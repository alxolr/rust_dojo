use std::collections::{HashMap, HashSet};

use crate::error::Result;
pub struct Solution;

impl Solution {
    pub fn common<F, K, V>(input: &str, memo: &mut HashMap<K, V>, apply_strategy: F) -> Result<u32>
    where
        F: Fn(&mut HashMap<K, V>, usize, usize, &[Vec<u32>]) -> u32,
    {
        let grid = load_grid(input);
        let rows = grid.len();
        let cols = grid[0].len();

        let mut total_sum: u32 = 0;
        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] == 0 {
                    total_sum += apply_strategy(memo, row, col, &grid);
                }
            }
        }

        Ok(total_sum)
    }

    pub fn part_1(input: &str) -> Result<u32> {
        let mut memo: HashMap<(usize, usize, u32), HashSet<(usize, usize)>> = HashMap::new();
        let total_sum = Self::common(input, &mut memo, |memo, row, col, grid| {
            let result = dfs_part_1(memo, row, col, 0, grid);

            result.len() as u32
        })?;

        Ok(total_sum)
    }

    pub fn part_2(input: &str) -> Result<u32> {
        let mut memo: HashMap<(usize, usize, u32), u32> = HashMap::new();

        let total_sum: u32 = Self::common(input, &mut memo, |memo, row, col, grid| {
            let result = dfs_part_2(memo, row, col, 0, &grid);

            result
        })?;

        Ok(total_sum)
    }
}

fn dfs_part_1(
    memo: &mut HashMap<(usize, usize, u32), HashSet<(usize, usize)>>,
    row: usize,
    col: usize,
    curr: u32,
    grid: &[Vec<u32>],
) -> HashSet<(usize, usize)> {
    // The idea for part 1 we do a dfs algorithm and store at each cell the positions of the 9's found on the path
    // in the end we will have for each zero the amount of distinct nines you can arrive.

    if let Some(result) = memo.get(&(row, col, curr)) {
        return result.clone();
    }

    if grid[row][col] != curr {
        return HashSet::new();
    }

    if grid[row][col] == 9 && curr == 9 {
        return HashSet::from_iter([(row, col)]);
    }

    let mut destinations = HashSet::new();
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for &(dr, dc) in &directions {
        let new_row = row as isize + dr;
        let new_col = col as isize + dc;

        if new_row >= 0
            && new_row < grid.len() as isize
            && new_col >= 0
            && new_col < grid[0].len() as isize
        {
            let dept_destinations =
                dfs_part_1(memo, new_row as usize, new_col as usize, curr + 1, grid);
            destinations.extend(dept_destinations);
        }
    }

    memo.insert((row, col, curr), destinations.clone());

    destinations
}

fn dfs_part_2(
    memo: &mut HashMap<(usize, usize, u32), u32>,
    row: usize,
    col: usize,
    curr: u32,
    grid: &[Vec<u32>],
) -> u32 {
    // For part two the problem simplifies tremendously as we just need to sum all the possible nines
    // not the distinct ones, so no hashset to find the distincts
    if let Some(result) = memo.get(&(row, col, curr)) {
        return result.clone();
    }

    if grid[row][col] != curr {
        return 0;
    }

    if grid[row][col] == 9 && curr == 9 {
        return 1;
    }

    let mut sum = 0;
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for &(dr, dc) in &directions {
        let new_row = row as isize + dr;
        let new_col = col as isize + dc;

        if new_row >= 0
            && new_row < grid.len() as isize
            && new_col >= 0
            && new_col < grid[0].len() as isize
        {
            sum += dfs_part_2(memo, new_row as usize, new_col as usize, curr + 1, grid)
        }
    }

    memo.insert((row, col, curr), sum);

    sum
}

fn load_grid(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().flat_map(|it| it.to_digit(10)).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() -> Result<()> {
        let input = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;
        assert_eq!(Solution::part_1(input)?, 36);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;
        assert_eq!(Solution::part_2(input)?, 81);

        Ok(())
    }
}
