pub struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut freq = vec![0; 26];

        for byte in s.as_bytes() {
            freq[(byte - 97) as usize] += 1;
        }

        for (idx, ch) in s.as_bytes().iter().enumerate() {
            if freq[(ch - 97) as usize] == 1 {
                return idx as i32;
            }
        }

        return -1;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_ok() {
        let scenarios = vec![("leetcode".to_string(), 0)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::first_uniq_char(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
