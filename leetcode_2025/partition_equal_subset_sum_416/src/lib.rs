use std::collections::HashMap;

pub struct Solution;

impl Solution {
    fn dp(
        memo: &mut HashMap<(usize, i32), bool>,
        id: usize,
        remaining: i32,
        nums: &Vec<i32>,
    ) -> bool {
        memo.get(&(id, remaining)).copied().unwrap_or_else(|| {
            if remaining == 0 {
                true
            } else if remaining < 0 || id == nums.len() - 1 {
                false
            } else {
                let result = Self::dp(memo, id + 1, remaining - nums[id], nums)
                    || Self::dp(memo, id + 1, remaining, nums);
                memo.insert((id, remaining), result);
                result
            }
        })
    }

    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();

        if sum % 2 != 0 {
            return false;
        }

        Self::dp(&mut HashMap::new(), 0, sum / 2, &nums)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(vec![1, 5, 11, 5], true), (vec![1, 2, 3, 5], false)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::can_partition(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
