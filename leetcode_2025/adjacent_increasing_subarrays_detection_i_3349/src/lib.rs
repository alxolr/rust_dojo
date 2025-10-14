impl Solution {
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;

        let mut current_len = 1;
        let mut prev_len = 0;

        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                current_len += 1;
            } else {
                prev_len = current_len;
                current_len = 1;
            }

            if current_len >= 2 * k || (prev_len >= k && current_len >= k) {
                return true;
            }
        }

        false
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ((vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1], 3), true),
            ((vec![1, 2, 3, 4, 4, 4, 4, 5, 6, 7], 5), false),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::has_increasing_subarrays(input.0, input.1);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
