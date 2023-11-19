struct Solution;

impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut max = 0;

        

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_frequency_ok() {
        let scenarios = vec![
            ((vec![1, 2, 4], 5), 3),
            ((vec![1, 4, 8, 13], 5), 2),
            ((vec![3, 9, 6], 2), 1),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((nums, k), expected))| {
                let result = Solution::max_frequency(nums, k);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
