use std::collections::HashMap;

impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let mut pairs = 0;
        let mut right = -1;
        let mut frequency = HashMap::new();
        let mut result = 0i64;

        for left in 0..n {
            while pairs < k && right + 1 < n as i32 {
                right += 1;
                let current_num = nums[right as usize];
                pairs += *frequency.get(&current_num).unwrap_or(&0);
                *frequency.entry(current_num).or_insert(0) += 1;
            }
            
            if pairs >= k {
                result += (n as i64) - (right as i64);
            }
            
            let left_num = nums[left];
            *frequency.entry(left_num).or_insert(0) -= 1;
            pairs -= *frequency.get(&left_num).unwrap_or(&0);
        }
        
        result
    }
}

pub struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ((vec![1, 1, 1, 1, 1], 10), 1),
            ((vec![3, 1, 4, 3, 2, 2, 4], 2), 4),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((nums, k), expected))| {
                let result = Solution::count_good(nums, k);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
