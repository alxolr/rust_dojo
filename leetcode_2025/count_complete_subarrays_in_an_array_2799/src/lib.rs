use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let distinct_count = nums.iter().collect::<HashSet<_>>().len();
        let n = nums.len();
        let mut count = 0;
        
        // For each possible start of subarray
        for start in 0..n {
            let mut freq = HashMap::new();
            
            // Extend window until we have all distinct elements
            for end in start..n {
                *freq.entry(nums[end]).or_insert(0) += 1;
                
                // If current window has all distinct elements, count subarrays
                if freq.len() == distinct_count {
                    count += 1;
                }
            }
        }
        
        count
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(vec![1, 3, 1, 2, 2], 4), (vec![5, 5, 5, 5], 10)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::count_complete_subarrays(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
