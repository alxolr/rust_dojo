pub struct Solution;

impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut max_value = i64::MIN;
        let mut max_first = nums[0];
        let mut max_diff = i32::MIN;

        for i in 1..nums.len() - 1 {
            max_diff = max_diff.max(max_first - nums[i]);
            max_value = max_value.max(max_diff as i64 * nums[i + 1] as i64);
            max_first = max_first.max(nums[i]);
        }

        max_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(vec![12, 6, 1, 2, 7], 77 as i64)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::maximum_triplet_value(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
