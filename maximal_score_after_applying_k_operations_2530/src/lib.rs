use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        let mut max_score: i64 = 0;
        let mut heap = BinaryHeap::from(nums);
        let mut remaining_operations = k;

        while remaining_operations > 0 {
            if let Some(max_element) = heap.pop() {
                max_score += max_element as i64;
                // Push the ceiling of max_element divided by 3 back into the heap
                heap.push((max_element as f32 / 3.0).ceil() as i32);
            }
            remaining_operations -= 1;
        }

        max_score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ((vec![10, 10, 10, 10, 10], 5), 50),
            ((vec![1, 10, 3, 3, 3], 3), 17),
            
            

        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((nums, k), expected))| {
                let result = Solution::max_kelements(nums, k);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
