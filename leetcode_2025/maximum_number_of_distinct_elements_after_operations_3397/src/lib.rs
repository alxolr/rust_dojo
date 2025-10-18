/// You are given an integer array nums and an integer k.
/// You are allowed to perform the following operation on each element of the array at most once:
/// Add an integer in the range [-k, k] to the element.
/// Return the maximum possible number of distinct elements in nums after performing the operations.
impl Solution {
    pub fn max_distinct_elements(nums: Vec<i32>, k: i32) -> i32 {
        // Step 1: Sort the array to process elements systematically
        let mut nums = nums;
        nums.sort();
        
        // Step 2: Start with a very small previous value
        let mut previous = i32::MIN;
        let mut distinct_count = 0;
        
        // Step 3: Process each element greedily
        for num in nums {
            // Calculate the optimal target directly (no loop needed!)
            let target = (previous + 1).max(num - k);
            
            if target <= num + k {
                // We can assign this element a distinct value
                distinct_count += 1;
                previous = target;
            }
            // If target > num + k, we can't assign this element
        }
        
        distinct_count
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![((vec![1, 2, 2, 3, 3, 4], 2), 6), ((vec![4, 4, 4, 4], 1), 3)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::max_distinct_elements(input.0, input.1);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
