struct Solution;

impl Solution {
    pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums.clone();
        nums.sort();

        let mut left = 0;
        let mut right = nums.len() - 1;

        let mut max = 0;

        while left < right {
            let sum = nums[left] + nums[right];
            if sum > max {
                max = sum;
            }
            left += 1;
            right -= 1;
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_pair_sum_ok() {
        let scenarios = vec![
            (vec![3, 5, 2, 3], 7),
            (vec![4, 1, 5, 1, 2, 5, 1, 5, 5, 4], 8),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::min_pair_sum(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
