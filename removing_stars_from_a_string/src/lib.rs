pub struct Solution;

impl Solution {
    pub fn remove_stars(str: String) -> String {
        let mut stack = vec![];

        for ch in str.chars() {
            if ch == '*' {
                stack.pop();
            } else {
                stack.push(ch);
            }
        }

        stack.into_iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ("leet**cod*e".to_string(), "lecoe".to_string()),
            ("erase*****".to_string(), "".to_string()),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::remove_stars(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
