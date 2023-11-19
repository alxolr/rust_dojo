struct Solution;

impl Solution {
    pub fn reduction_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        
        let mut result = 0;
        let mut up = 0;

        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                up += 1;
            }

            result += up;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduction_operation_ok() {
        let scenarios = vec![
            (vec![5, 1, 3], 3),
            (vec![1, 1, 1], 0),
            (vec![1, 1, 2, 2, 3], 4),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::reduction_operations(input);
                asseqrt_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
