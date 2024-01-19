use std::collections::HashMap;

pub struct Solution;

pub fn dfs(
    memo: &mut HashMap<(usize, usize), i32>,
    row: usize,
    col: usize,
    matrix: &Vec<Vec<i32>>,
) -> i32 {
    if let Some(result) = memo.get(&(row, col)) {
        return *result;
    }

    let rows = matrix.len();
    let cols = matrix[0].len();

    if row >= rows || (col >= cols) {
        // out of bounds check
        return i32::MAX / 2;
    }

    if row == rows - 1 {
        // if we got to the botom we got our solution
        if col < cols {
            return matrix[row][col];
        }
    }

    let mut result = i32::MAX / 2;

    if col > 0 {
        result = result.min(matrix[row][col] + dfs(memo, row + 1, col - 1, matrix));
    }

    result = result.min(matrix[row][col] + dfs(memo, row + 1, col, matrix));
    result = result.min(matrix[row][col] + dfs(memo, row + 1, col + 1, matrix));

    memo.insert((row, col), result);

    result
}

impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let mut result = i32::MAX;
        let mut memo = HashMap::new();

        for col in 0..matrix[0].len() {
            result = result.min(dfs(&mut memo, 0, col, &matrix));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_ok() {
        let scenarios = vec![
            (vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]], 13),
            (vec![vec![-19, 57], vec![-40, -5]], -59),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::min_falling_path_sum(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
