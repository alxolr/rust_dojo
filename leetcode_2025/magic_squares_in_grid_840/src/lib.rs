use std::collections::BTreeSet;

impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();

        if rows < 3 && cols < 3 {
            // it's not a valid magic square
            return 0;
        }

        let mut magic_square = 0;

        for row in 0..rows {
            for col in 0..cols {
                if Self::is_magic_square(row, col, &grid) {
                    magic_square += 1;
                }
            }
        }

        magic_square
    }

    fn is_magic_square(row_start: usize, col_start: usize, grid: &Vec<Vec<i32>>) -> bool {
        let nums_check = (1..=9).collect::<BTreeSet<_>>();
        let rows = grid.len();
        let cols = grid[0].len();
        let row_end = row_start + 2;
        let col_end = col_start + 2;

        // check boundaries for the 3x3 given starting point
        if row_end >= rows || col_end >= cols {
            return false;
        }

        let nums = (col_start..=col_end)
            .flat_map(|col| {
                (row_start..=row_end)
                    .map(|row| grid[row][col])
                    .collect::<Vec<_>>()
            })
            .collect::<BTreeSet<_>>();

        let rows_valid = (row_start..=row_end)
            .map(|row| grid[row][col_start..=col_end].iter().sum::<i32>())
            .all(|sum| sum == 15);

        let cols_valid = (col_start..=col_end)
            .map(|col| (row_start..=row_end).map(|row| grid[row][col]).sum::<i32>())
            .all(|sum| sum == 15);

        let (mut primary, mut anti) = (Vec::with_capacity(3), Vec::with_capacity(3));

        for (idx, row) in (row_start..=row_end).enumerate() {
            primary.push(grid[row][col_start + idx]);
            anti.push(grid[row][col_end - idx]);
        }

        let primary = primary.iter().sum::<i32>();
        let anti = anti.iter().sum::<i32>();

        rows_valid && cols_valid && primary == anti && primary == 15 && nums == nums_check
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (
                vec![vec![4, 3, 8, 4], vec![9, 5, 1, 9], vec![2, 7, 6, 2]],
                1,
            ),
            (vec![vec![5, 5, 5], vec![5, 5, 5], vec![5, 5, 5]], 0),
            (vec![vec![8]], 0),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::num_magic_squares_inside(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
