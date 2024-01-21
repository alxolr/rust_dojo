use std::collections::HashMap;

pub struct Solution;

fn dp(memo: &mut HashMap<usize, i32>, i: usize, nums: &[i32]) -> i32 {
    if let Some(result) = memo.get(&i) {
        return *result;
    }

    if i == nums.len() - 1 {
        return nums[i];
    }

    if i >= nums.len() {
        return 0;
    }

    let result: i32 = i32::max(nums[i] + dp(memo, i + 2, nums), dp(memo, i + 1, nums));

    memo.insert(i, result);

    result
}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut memo = HashMap::new();

        dp(&mut memo, 0, &nums)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_ok() {
        let scenarios = vec![
            (vec![1, 2, 3, 1], 4),
            (vec![2, 7, 9, 3, 1], 12),
            (
                vec![
                    114, 117, 207, 117, 235, 82, 90, 67, 143, 146, 53, 108, 200, 91, 80, 223, 58,
                    170, 110, 236, 81, 90, 222, 160, 165, 195, 187, 199, 114, 235, 197, 187, 69,
                    129, 64, 214, 228, 78, 188, 67, 205, 94, 205, 169, 241, 202, 144, 240,
                ],
                4173,
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::rob(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
