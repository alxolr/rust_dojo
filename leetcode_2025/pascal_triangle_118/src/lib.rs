use std::collections::HashMap;

impl Solution {
    fn next_pascal_from(nums: &Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        let first = nums[0];
        let last = nums[nums.len() - 1];

        result.push(first);
        for chunk in nums.windows(2) {
            let sum = chunk.iter().sum();
            result.push(sum);
        }

        result.push(last);

        result
    }

    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut dp = vec![vec![1], vec![1, 1], vec![1, 2, 1]];
        let mut result = vec![];

        for num in 0..num_rows as usize {
            if let Some(row) = dp.get(num) {
                result.push(row.clone());
            } else {
                dp[num] = Self::next_pascal_from(&dp[num - 1]);
                result.push(dp[num].clone());
            }
        }

        result
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (1, vec![vec![1]]),
            (2, vec![vec![1], vec![1, 1]]),
            (3, vec![vec![1], vec![1, 1], vec![1, 2, 1]]),
            (
                4,
                vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1]],
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::generate(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
