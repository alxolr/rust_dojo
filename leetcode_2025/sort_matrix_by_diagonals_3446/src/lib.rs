use std::collections::VecDeque;

impl Solution {
    pub fn sort_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // gather the diagonals
        let mut grid = grid;
        let grid_len = grid.len();

        for row in (0..=grid_len - 1).rev() {
            let mut values = vec![];
            for col in 0..=grid_len - 1 - row {
                values.push(grid[col + row][col]);
            }
            values.sort_by(|a, b| b.cmp(a)); // sort non increasing order;
            let mut deque = VecDeque::from(values);

            for col in 0..=grid_len - 1 - row {
                grid[col + row][col] = deque.pop_front().unwrap_or_default();
            }
        }

        for col in (1..=grid_len - 1).rev() {
            let mut values = vec![];
            for row in 0..=grid_len - 1 - col {
                values.push(grid[row][col + row]);
            }

            values.sort(); // sort non increasing order;
            let mut deque = VecDeque::from(values);

            for row in 0..=grid_len - 1 - col {
                grid[row][col + row] = deque.pop_front().unwrap_or_default();
            }
        }

        grid
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(
            vec![vec![1, 7, 3], vec![9, 8, 2], vec![4, 5, 6]],
            vec![vec![8, 2, 3], vec![9, 6, 7], vec![4, 5, 1]],
        )];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::sort_matrix(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
