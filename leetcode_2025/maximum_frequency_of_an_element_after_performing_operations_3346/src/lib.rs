impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        
        let n = *nums.iter().max().unwrap() as usize;
        let mut count = vec![0; n + 1];
        
        // Counting sort - count frequency of each value
        for &num in &nums {
            count[num as usize] += 1;
        }
        
        // Build prefix sum array
        let mut presum = vec![0; n + 2];
        for i in 0..=n {
            presum[i + 1] = presum[i] + count[i];
        }
        
        let mut ans = 0;
        
        // Try each possible target value that exists in nums
        for i in 0..=n {
            let left_start = (i as i32 - k).max(0) as usize;
            let right_end = ((i as i32 + k + 1).min(n as i32 + 1)) as usize;
            
            // Elements in [i-k, i-1] that can reach i
            let left = presum[i] - presum[left_start];
            
            // Elements in [i+1, i+k] that can reach i
            let right = presum[right_end] - presum[i + 1];
            
            // Current frequency: existing + min(operations_available, elements_that_can_reach)
            let cur = count[i] + (num_operations as usize).min(left + right);
            ans = ans.max(cur);
        }
        
        ans as i32
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![((vec![1, 4, 5], 1, 2), 2)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::max_frequency(input.0, input.1, input.2);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
