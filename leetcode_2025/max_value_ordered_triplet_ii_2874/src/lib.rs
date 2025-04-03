pub struct Solution;

impl Solution {
    pub fn maximum_triplet_value(numbers: Vec<i32>) -> i64 {
        let mut max_triplet_value = 0i64;
        let mut max_prefix = numbers[0];
        let mut max_prefix_diff = 0;

        for index in 1..numbers.len() - 1 {
            max_prefix = max_prefix.max(numbers[index]);
            max_prefix_diff = max_prefix_diff.max(max_prefix - numbers[index]);
            max_triplet_value = max_triplet_value.max(max_prefix_diff as i64 * numbers[index + 1] as i64);
        }

        max_triplet_value as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec![12, 6, 1, 2, 7], 77),
            (vec![1000000, 1, 1000000], 999999000000            )
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::maximum_triplet_value(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
