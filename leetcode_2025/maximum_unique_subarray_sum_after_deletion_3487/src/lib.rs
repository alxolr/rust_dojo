use std::collections::HashSet;

impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut sorted = nums;
        sorted.sort_by(|a, b| b.cmp(a));

        let mut seen = HashSet::new();
        seen.insert(sorted[0]);
        let mut max_sum = sorted[0];
        let mut sum = sorted[0];

        for num in sorted.into_iter().skip(1) {
            if !seen.contains(&num) {
                sum += num;
                seen.insert(num);
            }

            max_sum = max_sum.max(sum);
        }

        max_sum
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec![1, 2, 3, 4, 5], 15),
            (vec![1, 2, -1, -2, 1, 0, -1], 3),
            (vec![-100], -100),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::max_sum(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
