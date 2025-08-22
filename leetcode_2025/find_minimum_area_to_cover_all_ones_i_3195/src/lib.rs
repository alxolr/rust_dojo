impl Solution {
    pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut min_row = usize::MAX;
        let mut max_row = 0;
        let mut min_col = usize::MAX;
        let mut max_col = 0;
        let rows = grid.len();
        let cols = grid[0].len();

        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] == 1 {
                    min_row = min_row.min(row);
                    min_col = min_col.min(col);
                    max_row = max_row.max(row);
                    max_col = max_col.max(col);
                }
            }
        }
        
        let result = (max_row - min_row + 1) * (max_col - min_col + 1);

        result as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec![vec![0, 1, 0]], 1),
            (vec![vec![0, 1, 0], vec![1, 0, 1]], 6),
            (vec![vec![1, 0], vec![0, 0]], 1),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::minimum_area(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
