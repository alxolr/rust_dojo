pub struct Solution;

impl Solution {
    fn reverse(s: &str) -> String {
        s.chars().rev().collect()
    }

    pub fn reverse_parentheses(s: String) -> String {
        let mut stack = vec![];
        let mut current = String::new();

        for c in s.chars() {
            match c {
                '(' => {
                    stack.push(current.clone());
                    current.clear();
                }
                ')' => {
                    current = stack.pop().unwrap() + &Self::reverse(&current);
                }
                _ => {
                    current.push(c);
                }
            }
        }

        current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ("(abcd)".to_string(), "dcba".to_string()),
            ("(u(love)i)".to_string(), "iloveu".to_string()),
            ("(ed(et(oc))el)".to_string(), "leetcode".to_string()),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::reverse_parentheses(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
