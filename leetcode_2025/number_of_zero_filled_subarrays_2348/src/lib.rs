impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut total_count = 0;
        let mut last_count = 0;
        
        for num in nums {
            if num == 0 {
                last_count += 1;
                total_count += last_count;
            } else {
                last_count = 0;
            }
        }

        total_count
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec![1, 0, 0, 2, 0, 0, 0], 9),
            (vec![0, 0, 0, 0], 10),
            (vec![1, 2, 3, 4], 0),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::zero_filled_subarray(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
