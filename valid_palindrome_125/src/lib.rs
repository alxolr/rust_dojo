pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chs = s
            .chars()
            .filter(|ch| ch.is_alphanumeric())
            .map(|ch| ch.to_lowercase());

        chs.clone()
            .zip(chs.rev())
            .all(|(a, b)| a.eq(b))
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ("A man, a plan, a canal: Panama".to_string(), true),
            ("race a car".to_string(), false),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::is_palindrome(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
