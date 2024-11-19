use std::collections::HashSet;

pub struct Solution;

impl Solution {
    /// Finds the maximum subarray sum within a sliding window of size k.
    ///
    /// # Parameters
    ///
    /// * `nums`: The input array of integers.
    /// * `k`: The size of the sliding window.
    ///
    /// # Returns
    ///
    /// The maximum sum that can be achieved by moving the window over the array while maintaining a size of k.
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        // Initialize variables to track the window's position and maximum sum
        let len = nums.len();
        let mut left = 0;
        let mut right = 0;
        let mut max_sum: i32 = 0;

        // Use a set to keep track of unique elements within the window
        let mut unique_set = HashSet::new();
        let mut local_sum = 0;

        // Iterate over the array with the sliding window
        while right < len {
            // Expand the window to the right as long as it's smaller than k and doesn't contain duplicate values
            while right - left < k as usize && right < len {
                if unique_set.contains(&nums[right]) {
                    break;
                } else {
                    unique_set.insert(nums[right]);
                    local_sum += nums[right];
                    right += 1;
                }
            }

            // If the window has reached size k, update the maximum sum
            if (right - left) == k as usize {
                max_sum = std::cmp::max(max_sum, local_sum);
            }

            // Move the window to the right by removing the leftmost element and decrementing the local sum
            local_sum -= nums[left];
            unique_set.remove(&nums[left]);
            left += 1;
        }

        // Convert the maximum sum from i32 to i64 before returning it
        max_sum as i64
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ((vec![1, 5, 4, 2, 9, 9, 9], 3), 15),
            ((vec![9, 9, 9], 3), 0),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((nums, k), expected))| {
                let result = Solution::maximum_subarray_sum(nums, k);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
