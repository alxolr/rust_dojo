pub struct Solution;

impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let len = arr.len();

        let mut i = 0;
        let mut j = 0;
        let mut sum = 0;

        while i < len {
            j = i;

            while j < len {
                sum += arr[i..=j].iter().sum::<i32>();
                j += 2;
            }

            i += 1;
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::format;

    use super::*;

    fn min_bit_flips(start: i32, goal: i32) -> i32 {
        let mut binary_start = format!("{:b}", start);
        let mut binary_goal = format!("{:b}", goal);

        if binary_goal.len() > binary_start.len() {
            let size = binary_goal.len();
            let leading_zeros = size - binary_start.len();

            binary_start = format!("{}{}", "0".repeat(leading_zeros), binary_start);
        } else {
            let size = binary_start.len();
            let leading_zeros = size - binary_goal.len();

            binary_goal = format!("{}{}", "0".repeat(leading_zeros), binary_goal);
        };
        
        binary_start.chars()
            .zip(binary_goal.chars())
            .filter(|(s, g)| s != g)
            .count() as i32
    }

    #[test]
    fn test_min_bif_flips_ok() {
        let scenarios = vec![
            ((10, 7), 3),
            ((3, 4), 3),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((start, goal), expected))| {
                let result = min_bit_flips(start, goal);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }

    #[test]
    fn test_solution_ok() {
        let scenarios = vec![
            (vec![1, 4, 2, 5, 3], 58),
            (vec![1, 2], 3),
            (vec![10, 11, 12], 66),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::sum_odd_length_subarrays(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
