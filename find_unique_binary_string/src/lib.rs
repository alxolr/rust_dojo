use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let sets: HashSet<_> = nums
            .iter()
            .map(|s| isize::from_str_radix(s, 2).unwrap())
            .collect();

        let n = nums.len() as isize;

        for i in 0isize..=n {
            if !sets.contains(&i) {
                return format!("{:0width$b}", i, width = n as usize);
            }
        }

        unreachable!("should not reach here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_different_binary_string_ok() {
        let scenarios = vec![
            (vec!["01".to_string(), "10".to_string()], "00".to_string()),
            (vec!["00".to_string(), "01".to_string()], "10".to_string()),
            (
                vec!["111".to_string(), "011".to_string(), "001".to_string()],
                "000".to_string(),
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::find_different_binary_string(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
