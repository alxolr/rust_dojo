use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let unique_chars: HashSet<char> = s.chars().collect();
        let mut unique_combos = 0;

        for ch in unique_chars {
            let start_idx = s.find(ch).unwrap();
            let end_idx = s.rfind(ch).unwrap();

            if start_idx == end_idx {
                continue;
            }

            let sub_str = &s[start_idx + 1..end_idx];
            let unique_chars: HashSet<char> = sub_str.chars().collect();
            unique_combos += unique_chars.len();
        }

        unique_combos as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_palindromic_subsequence_ok() {
        let scenarios = vec![
            ("aabca".to_string(), 3),
            ("adc".to_string(), 0),
            ("bbcbaba".to_string(), 4),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::count_palindromic_subsequence(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
