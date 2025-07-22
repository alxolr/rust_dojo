use std::collections::HashSet;

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut max_sum = 0;
        let mut current_sum = 0;
        let mut seen = HashSet::new();

        for (_, &num) in nums.iter().enumerate() {
            while seen.contains(&num) {
                current_sum -= nums[left];
                seen.remove(&nums[left]);
                left += 1;
            }

            seen.insert(num);
            current_sum += num;
            max_sum = max_sum.max(current_sum);
        }

        max_sum
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec![4, 2, 4, 5, 6], 17),
            (vec![5, 2, 1, 2, 5, 2, 1, 2, 5], 8),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::maximum_unique_subarray(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
