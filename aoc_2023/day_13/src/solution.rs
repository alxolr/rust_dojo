use crate::error::Result;
pub struct Solution;

type Grid = Vec<Vec<char>>;

impl Solution {
    pub fn process(
        input: &str,
        rows_fn: fn(&Grid) -> Option<usize>,
        cols_fn: fn(&Grid) -> Option<usize>,
    ) -> Result<i64> {
        let result = grids(input)
            .iter()
            .map(|grid| find_mirror(grid, rows_fn, cols_fn))
            .sum::<i64>();

        Ok(result)
    }

    pub fn part_1(input: &str) -> Result<i64> {
        let result = Solution::process(input, maybe_rows, maybe_cols)?;

        Ok(result)
    }

    pub fn part_2(input: &str) -> Result<i64> {
        let result = Solution::process(input, maybe_rows_p2, maybe_cols_p2)?;

        Ok(result)
    }
}

fn diff(first: &Vec<char>, second: &Vec<char>) -> usize {
    first.iter().zip(second.iter()).fold(0, |mut acc, (a, b)| {
        if a != b {
            acc += 1
        }

        acc
    })
}

fn maybe_cols_p2(grid: &Grid) -> Option<usize> {
    let m = grid[0].len();
    // the key to the problem is that we know that there is exactly one smudge, only

    'mirror: for (left, right) in (0..m - 1).zip(1..m) {
        let mut diff_total = 0;

        diff_total += diff(&collect(&grid, left), &collect(&grid, right));

        if diff_total <= 1 {
            for dist in 1..=min_to_bound_dist(left, right, m - 1) {
                diff_total += diff(&collect(grid, left - dist), &collect(grid, right + dist));

                if diff_total > 1 {
                    continue 'mirror;
                }
            }
        }

        if diff_total == 1 {
            return Some(left);
        }
    }

    None
}

fn maybe_rows_p2(grid: &Grid) -> Option<usize> {
    let n = grid.len();

    'mirror: for (up, down) in (0..n - 1).zip(1..n) {
        let mut diff_total = 0;

        diff_total += diff(&grid[up], &grid[down]);

        if diff_total <= 1 {
            for dist in 1..=min_to_bound_dist(up, down, n - 1) {
                diff_total += diff(&grid[up - dist], &grid[down + dist]);

                if diff_total > 1 {
                    continue 'mirror;
                }
            }

            if diff_total == 1 {
                return Some(up);
            }
        }
    }

    None
}

fn maybe_rows(grid: &Grid) -> Option<usize> {
    let n = grid.len();

    'mirror: for (up, down) in (0..n - 1).zip(1..n) {
        if diff(&grid[up], &grid[down]) == 0 {
            for dist in 1..=min_to_bound_dist(up, down, n - 1) {
                if diff(&grid[up - dist], &grid[down + dist]) != 0 {
                    continue 'mirror;
                }
            }

            return Some(up);
        }
    }

    None
}

fn collect(grid: &Grid, col: usize) -> Vec<char> {
    grid.iter().map(|line| line[col]).collect()
}

fn maybe_cols(grid: &Grid) -> Option<usize> {
    let m = grid[0].len();

    'mirror: for (left, right) in (0..m - 1).zip(1..m) {
        if diff(&collect(&grid, left), &collect(&grid, right)) == 0 {
            for dist in 1..=min_to_bound_dist(left, right, m - 1) {
                if diff(&collect(grid, left - dist), &collect(grid, right + dist)) != 0 {
                    continue 'mirror;
                }
            }

            return Some(left);
        }
    }

    None
}

fn min_to_bound_dist(up: usize, down: usize, n: usize) -> usize {
    (n - down).min(up - 0)
}

fn find_mirror(
    grid: &Grid,
    rows_fn: fn(&Grid) -> Option<usize>,
    cols_fn: fn(&Grid) -> Option<usize>,
) -> i64 {
    if let Some(rows) = rows_fn(grid) {
        return 100 * (rows as i64 + 1);
    }

    if let Some(cols) = cols_fn(grid) {
        return cols as i64 + 1;
    }

    panic!("Mirror not found");
}

fn grids(input: &str) -> Vec<Grid> {
    input
        .split("\n\n")
        .map(|s| {
            s.lines()
                .map(|l| l.trim())
                .filter(|x| !x.is_empty())
                .map(|x| x.chars().collect())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> String {
        r"
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
        "
        .to_string()
    }

    #[test]
    fn test_solution_part_1_from_example() -> Result<()> {
        let expected = 405;
        let input = example();

        assert_eq!(Solution::part_1(&input)?, expected);

        Ok(())
    }

    #[test]
    fn test_solution_part_2_from_example() -> Result<()> {
        let expected = 400;
        let input = example();

        assert_eq!(Solution::part_2(&input)?, expected);

        Ok(())
    }
}
