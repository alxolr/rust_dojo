pub struct Solution;

impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut duplets = 0;

        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if (nums[i] - nums[j]).abs() == k {
                    duplets += 1;
                }
            }
        }

        duplets
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_ok() {
        let scenarios = vec![
            ((vec![1, 2, 2, 1], 1), 4),
            ((vec![1, 3], 3), 0),
            ((vec![3, 2, 1, 5, 4], 2), 3),
            ((vec![10, 2, 10, 9, 1, 6, 8, 9, 2, 8], 5), 1),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((nums, k), expected))| {
                let result = Solution::count_k_difference(nums, k);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
