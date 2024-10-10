pub struct Solution;

impl Solution {
    fn max_seen_from_right(nums: &Vec<i32>) -> Vec<i32> {
        let mut max_seen = vec![-1; nums.len()];
        let mut max_value = i32::MIN;

        for idx in (0..nums.len()).rev() {
            max_value = max_value.max(nums[idx]);
            max_seen[idx] = max_value
        }

        max_seen
    }

    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
        let max_seen = Self::max_seen_from_right(&nums);

        let mut left = 0;
        let mut right = 1;
        let mut max_width = 0;

        while left < right && right < nums.len() {
            if nums[left] <= nums[right] {
                let width = right - left;

                max_width = max_width.max(width);
                right += 1;
            } else {
                if nums[left] <= max_seen[right] {
                    right += 1;
                } else {
                    left += 1;
                    right += 1;
                }
            }
        }

        max_width as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec![6, 0, 8, 2, 1, 5], 4),
            (vec![9, 8, 1, 0, 1, 9, 4, 0, 4, 1], 7),
            (vec![3, 1, 2], 1),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::max_width_ramp(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
