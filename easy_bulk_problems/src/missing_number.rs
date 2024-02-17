pub struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let sequence = nums.iter().fold(vec![0; nums.len() + 1], |mut acc, it| {
            acc[*it as usize] += 1;

            acc
        });

        let pos = sequence.iter().position(|n| *n == 0).unwrap_or_default();

        pos as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_ok() {
        let scenarios = vec![(vec![3, 0, 1], 2)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::missing_number(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
