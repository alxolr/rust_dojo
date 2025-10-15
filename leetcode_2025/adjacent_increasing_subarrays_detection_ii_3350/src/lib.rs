impl Solution {
    /// Finds the maximum length k such that there exist two adjacent
    /// strictly increasing subarrays of length k each.
    pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
        let mut max_k = 0;
        let mut current_size = 1;
        let mut prev_size = 0;

        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                current_size += 1;
            } else {
                prev_size = current_size;
                current_size = 1;
            }

            // It tracks 3 possibilities, either we have two different adjacent increasing arrays
            // Or we could have a one continuous big array which we will divide in two
            max_k = max_k
                .max(current_size.min(prev_size))
                .max(current_size.max(prev_size) / 2);
        }

        max_k
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_max_adjacent_increasing_subarrays() {
        let scenarios = vec![
            (vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1], 3),
            (vec![1, 2, 3, 4, 4, 4, 4, 5, 6, 7], 2),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::max_increasing_subarrays(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
