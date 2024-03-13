struct Solution;

impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let mut left = 0;
        let mut right = 0;
        let mut count = 0;

        while left < nums.len() && right < nums.len() {
            
            let sum = nums[left..=right].iter().sum::<i32>();
            
            count += 1;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ((vec![1, 0, 1, 0, 1], 2), 4),
            ((vec![0, 0, 0, 0, 0], 0), 15),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((nums, goal), expected))| {
                let result = Solution::num_subarrays_with_sum(nums, goal);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
