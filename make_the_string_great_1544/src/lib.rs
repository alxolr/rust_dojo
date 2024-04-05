pub struct Solution;

impl Solution {
    pub fn make_good(s: String) -> String {
        let mut stack: Vec<char> = vec![];

        for ch in s.chars() {
            if let Some(last) = stack.last() {
                if last.eq_ignore_ascii_case(&ch) && last != &ch {
                    stack.pop();
                    continue;
                }
            }

            stack.push(ch);
        }

        stack.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ("leEeetcode".to_string(), "leetcode".to_string()),
            ("abBAcC".to_string(), "".to_string()),
            ("aA".to_string(), "".to_string()),
            ("Aa".to_string(), "".to_string()),
            ("a".to_string(), "a".to_string()),
            ("A".to_string(), "A".to_string()),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::make_good(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
