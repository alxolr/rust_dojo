impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let total_sum: i32 = nums.iter().sum();
        
        nums.iter()
            .scan(0, |left_sum, &num| {
                if num != 0 {
                    *left_sum += num;
                    Some(0)
                } else {
                    let right_sum = total_sum - *left_sum;
                    let diff = (right_sum - *left_sum).abs();
                    
                    Some(match diff {
                        0 => 2,
                        1 => 1,
                        _ => 0,
                    })
                }
            })
            .sum()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec![1, 0, 2, 0, 3], 2),
            (vec![2, 3, 4, 0, 4, 1, 0], 0),
            // Additional edge cases
            (vec![0], 2),                    // Single zero element
            (vec![1, 2, 3], 0),             // No zeros
            (vec![0, 1, 0], 2),             // Zero at start and end
            (vec![3, 0, 0, 3], 4),          // Multiple zeros with equal sums
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::count_valid_selections(input.clone());
                assert_eq!(
                    result, expected,
                    "Failed on test case {}: input = {:?}, expected = {}, got = {}",
                    idx + 1, input, expected, result
                );
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
