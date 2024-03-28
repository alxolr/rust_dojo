use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut freq = HashMap::new();
        let mut result = 0;

        let mut left = 0;
        for right in 0..nums.len() {
            let entry = freq.entry(nums[right]).or_insert(0);
            *entry += 1;

            loop {
                let right_freq = freq.get(&nums[right]).unwrap_or(&0);

                if *right_freq <= k {
                    break;
                }

                let entry = freq.entry(nums[left]).or_insert(0);
                *entry -= 1;
                left += 1;
            }

            result = result.max(right - left + 1);
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ((vec![1, 4, 4, 3], 1), 2),
            ((vec![3, 1, 1], 1), 2),
            ((vec![1, 2, 3, 1, 2, 3, 1, 2], 2), 6),
            ((vec![1, 2, 1, 2, 1, 2, 1, 2], 1), 2),
            ((vec![5, 5, 5, 5, 5, 5, 5, 5], 4), 4),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((nums, k), expected))| {
                let result = Solution::max_subarray_length(nums, k);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
