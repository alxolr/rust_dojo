use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }

        let dictionary = vec![
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]
        .into_iter()
        .collect::<HashMap<char, i32>>();

        let mut latest: i32 = 0;
        let mut sum = 0;

        for ch in s.chars().rev() {
            if let Some(value) = dictionary.get(&ch) {
                if latest > *value {
                    sum -= value;
                } else {
                    sum += value;
                }

                latest = *value;
            }
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_content_children_ok() {
        let scenarios = vec![
            ("III".to_string(), 3),
            ("LVIII".to_string(), 58),
            ("MCMXCIV".to_string(), 1994),
        ];

        "test".to_string().split_whitespace()

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (s, expected))| {
                let result = Solution::roman_to_int(s);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
