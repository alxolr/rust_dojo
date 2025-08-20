impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let rows = matrix.len();
        let cols = matrix[0].len();

        let mut ans = 0;
        let mut dp = vec![vec![0; cols + 1]; rows + 1];

        for row in 0..rows {
            for col in 0..cols {
                if matrix[row][col] == 1 {
                    let min = dp[row][col + 1].min(dp[row + 1][col]).min(dp[row][col]);

                    dp[row + 1][col + 1] = min + 1;
                    ans += dp[row + 1][col + 1];
                }
            }
        }

        ans
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(
            vec![vec![0, 1, 1, 1], vec![1, 1, 1, 1], vec![0, 1, 1, 1]],
            15,
        )];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::count_squares(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
