use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::error::Result;
pub struct Solution;

impl Solution {
    fn common_part<F>(input: &str, solution: F, k: usize) -> Result<u64>
    where
        F: FnOnce(Vec<u64>, usize) -> u64 + Sync + Copy + Send,
    {
        Ok(parse_input(input)
            .into_par_iter()
            .map(|line| solution(line, k))
            .sum())
    }

    pub fn part_1(input: &str) -> Result<u64> {
        Self::common_part(input, solution, 2)
    }

    pub fn part_2(input: &str) -> Result<u64> {
        Self::common_part(input, solution, 12)
    }
}

fn solution(digits: Vec<u64>, key_length: usize) -> u64 {
    let digits_len = digits.len();
    let last_digit = digits[digits_len - 1];

    // Create a dynamic programming table to store all the intermediate results we initialize the table
    // with the last digit, as if we have only one item in the array then automatically this is the max sum
    // we can get out of it.
    let mut dp = vec![vec![last_digit; key_length]; digits.len()];

    // we go backwards on processing the table in reversed order, we skip the last number as we
    // already initialized it in previous iteration
    for (idx, digit) in digits.into_iter().enumerate().rev().skip(1) {
        // We populate the table for all possible k solutions
        for k in 0..key_length {
            let prev_idx = idx as isize + 1; // remember we go backwards from last to 0, so prev for us will be actually next
            let prev_k = k as isize - 1; // this is normal

            // Very important to compute the max pow as if we have k being 12 for example
            // but we are just on position 3 from the processing of array we can't multiply by 10^12
            // maximum allowed in this case is 10^3, we calculate using the formula (digits_len - 1 - current_idx)
            let pow = u32::min(k as u32, ((digits_len - 1) - idx) as u32);

            // we update the current dp table with the two possible solions, either we include the number
            // or we don't so we compare if we include with the max between previous and current
            dp[idx][k] = u64::max(
                digit * 10_u64.pow(pow) + safe_get(prev_idx, prev_k, &dp),
                safe_get(prev_idx, k as isize, &dp),
            );
        }
    }

    dp[0][key_length - 1] // as we don't go in reverse with the k, and k is from 0..k, last k will be actually k - 1
}

// Help function for convenience of handling out of bounds
fn safe_get(i: isize, j: isize, dp: &Vec<Vec<u64>>) -> u64 {
    if i < 0 || i >= dp.len() as isize {
        return 0;
    }

    if j < 0 {
        return 0;
    }

    return dp[i as usize][j as usize];
}

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .flat_map(|digit| digit.to_digit(10)) // to digit return a Result<u32> but we need u64
                .map(|item| item as u64)
                .collect::<Vec<_>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_ok() {
        let scenarios = vec![
            ((vec![1, 9, 5], 3), 195),
            ((vec![1, 9, 5, 3, 2, 1], 4), 9532),
            ((vec![1, 9, 5, 3, 2, 1], 5), 95321),
            ((vec![1, 9, 5, 3, 2, 1], 6), 195321),
            ((vec![1, 9, 5, 3, 2, 1], 1), 9),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((num, k), expected))| {
                let result = solution(num, k);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }

    #[test]
    fn test_part_1() -> Result<()> {
        let input = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;
        assert_eq!(Solution::part_1(input)?, 357);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;
        assert_eq!(Solution::part_2(input)?, 3121910778619);

        Ok(())
    }
}
