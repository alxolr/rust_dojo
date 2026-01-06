impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        // Observations:
        // If we have an even number of negative numbers there should be always
        // a way to transform them in positive
        // If we have an odd count of numbers then there will be always one remaining
        //
        //
        // Algorithm:
        // - Count all the negative numbers
        // - Sum all the numbers but make the negative positives
        // - Keep the smallest negative element
        // - If we have an odd number return the sum - the smallest
        // - If we have even return the sum

        let mut max_negative = i32::MIN;
        let mut positive_sum = 0;
        let mut negative_count = 0;

        for row in matrix {
            for item in row {
                if item < 0 {
                    negative_count += 1;
                    max_negative = max_negative.max(item);
                }

                positive_sum += item.abs() as i64
            }
        }

        if negative_count % 2 == 0 {
            positive_sum
        } else {
            positive_sum + 2 * max_negative as i64
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            // (vec![vec![1, -1], vec![-1, 1]], 4),
            // (vec![vec![1, 2, 3], vec![-1, -2, -3], vec![1, 2, 3]], 16),
            (vec![vec![-1, 0, -1], vec![-2, 1, 3], vec![3, 2, 2]], 15),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::max_matrix_sum(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
