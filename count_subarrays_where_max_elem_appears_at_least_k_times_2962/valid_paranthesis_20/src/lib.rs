pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        let bracket_pairs: HashMap<char, char> = [(')', '('), (']', '['), ('}', '{')]
            .iter()
            .cloned()
            .collect();

        for bracket in s.chars() {
            match bracket_pairs.get(&bracket) {
                Some(&closing_bracket) => {
                    if stack.pop() != Some(closing_bracket) {
                        return false;
                    }
                }
                None => stack.push(bracket),
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ("((".to_string(), false),
            ("{[]}".to_string(), true),
            ("()".to_string(), true),
            ("(})".to_string(), false),
            ("()[]{}".to_string(), true),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::is_valid(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
