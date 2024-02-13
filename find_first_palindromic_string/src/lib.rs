pub struct Solution;

impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        words
            .into_iter()
            .filter(|word| Self::is_palindrome(word.as_str()))
            .nth(0)
            .unwrap_or("".to_string())
    }

    fn is_palindrome(word: &str) -> bool {
        let first_half = word.chars().take(word.len() / 2);
        let second_half = word.chars().rev().take(word.len() / 2);

        first_half.eq(second_half)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (
                vec![
                    "abc".to_string(),
                    "car".to_string(),
                    "ada".to_string(),
                    "racecar".to_string(),
                    "cool".to_string(),
                ],
                "ada".to_string(),
            ),
            (
                vec![
                    "xngla".to_string(),
                    "e".to_string(),
                    "itrn".to_string(),
                    "y".to_string(),
                    "s".to_string(),
                    "pfp".to_string(),
                    "rfd".to_string(),
                ],
                "e".to_string(),
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::first_palindrome(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
