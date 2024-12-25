use crate::error::Result;
use itertools::Itertools;
pub struct Solution;

impl Solution {
    pub fn part_1(input: &str) -> Result<i32> {
        let (keys, locks) = parse_keys_and_locks(input)?;

        let count = keys
            .iter()
            .cartesian_product(locks.iter())
            .filter(|(key, lock)| {
                let overlap: Vec<i8> = key
                    .iter()
                    .zip(lock.iter())
                    .map(|(key_val, lock_val)| *key_val + *lock_val)
                    .collect();

                overlap.iter().all(|it| it < &6)
            })
            .count();

        Ok(count as i32)
    }

    pub fn part_2(_input: &str) -> Result<i32> {
        Ok(0)
    }
}

fn parse_keys_and_locks(input: &str) -> Result<(Vec<Vec<i8>>, Vec<Vec<i8>>)> {
    let (key_grids, lock_grids): (Vec<Vec<Vec<char>>>, Vec<Vec<Vec<char>>>) = input
        .split("\n\n")
        .map(|grid| {
            grid.lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .partition(|grid| grid[0][0] == '.');

    let keys = flatten_grid(key_grids);
    let locks = flatten_grid(lock_grids);

    Ok((keys, locks))
}

fn flatten_grid(grid: Vec<Vec<Vec<char>>>) -> Vec<Vec<i8>> {
    let grid: Vec<Vec<i8>> = grid
        .into_iter()
        .map(|key| {
            key.into_iter().fold(vec![-1; 5], |mut acc, row| {
                row.iter().enumerate().for_each(|(idx, ch)| {
                    if ch == &'#' {
                        acc[idx] += 1;
                    }
                });
                acc
            })
        })
        .collect();

    grid
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() -> Result<()> {
        let input = r#"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"#;
        assert_eq!(Solution::part_1(input)?, 3);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = r#""#;
        assert_eq!(Solution::part_2(input)?, 0);

        Ok(())
    }
}
