pub struct Solution;

impl Solution {
    /// This function calculates the number of ways to split the array into two non-empty parts
    /// such that the sum of the left part is greater than or equal to the sum of the right part.
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        // Handle the boundary cases
        if nums.len() <= 1 {
            return 0;
        }

        // Calculate the total sum of the array
        let total_sum: i64 = nums.iter().map(|it| *it as i64).sum();

        let mut left_sum: i64 = 0;
        let mut count = 0;
        // Iterate through the array to find the split points
        for idx in 0..nums.len() - 1 {
            left_sum += nums[idx] as i64;

            // Check if the left sum is greater than or equal to the right sum
            if left_sum >= total_sum - left_sum {
                count += 1;
            }
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
            (vec![10, 4, -8, 7], 2),
            (vec![2, 3, 1, 0], 2),
            (vec![], 0),
            (vec![1], 0),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::ways_to_split_array(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
