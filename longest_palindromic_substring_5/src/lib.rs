use std::collections::HashMap;

pub struct Solution;

fn is_palindrome(s: &str) -> bool {
    s.bytes()
        .zip(s.bytes().rev())
        .all(|(left, right)| left == right)
}

fn max_palindrome<'a>(memo: &mut HashMap<&'a str, Option<&'a str>>, s: &'a str) -> Option<&'a str> {
    if is_palindrome(s) {
        return Some(s);
    }

    let palindrome = [
        max_palindrome(memo, &s[1..]),            // try without the right part
        max_palindrome(memo, &s[0..s.len() - 1]), // try without the left most
    ]
    .into_iter()
    .flatten()
    .max_by(|left, right| left.len().cmp(&right.len()));

    memo.insert(s, palindrome);

    palindrome
}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let palindrome = max_palindrome(&mut HashMap::new(), &s);

        palindrome.unwrap().to_string()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome_ok() {
        let scenarios = vec![("aba", true), ("aban", false), ("aa", true), ("ac", false)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = is_palindrome(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }

    #[test]
    fn it_works() {
        let scenarios = vec![
            ("babad".to_string(), "bab".to_string()),
            ("ana".to_string(), "ana".to_string()),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::longest_palindrome(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
