use std::collections::HashMap;

pub struct Solution;

impl Solution {
    fn is_divisible(left: i32, right: i32) -> bool {
        left % right == 0 || right % left == 0
    }

    pub fn dp(
        memo: &mut HashMap<(usize, Option<i32>), Vec<i32>>,
        current_index: usize,
        last_included: Option<i32>,
        sorted_nums: &Vec<i32>,
    ) -> Vec<i32> {
        if let Some(result) = memo.get(&(current_index, last_included)) {
            return result.clone();
        }

        if current_index == sorted_nums.len() {
            return vec![];
        }

        let subset_with_current = if last_included.map_or(true, |last| {
            Self::is_divisible(sorted_nums[current_index], last)
        }) {
            let mut subset = vec![sorted_nums[current_index]];
            subset.extend(Self::dp(
                memo,
                current_index + 1,
                Some(sorted_nums[current_index]),
                sorted_nums,
            ));

            subset
        } else {
            vec![]
        };

        let subset_without_current = Self::dp(memo, current_index + 1, last_included, sorted_nums);

        let answer = if subset_with_current.len() > subset_without_current.len() {
            subset_with_current
        } else {
            subset_without_current
        };

        memo.insert((current_index, last_included), answer.clone());

        answer
    }

    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort();

        Self::dp(&mut HashMap::new(), 0, None, &nums)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec![1, 2, 3], vec![1, 3]),
            (vec![1, 2, 4, 8], vec![1, 2, 4, 8]),
            (vec![1, 2, 4, 5], vec![1, 2, 4]),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::largest_divisible_subset(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
