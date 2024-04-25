pub struct Solution;

impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut s = s;

        let mut operations = 0;

        while s != "1" {
            if s.ends_with('0') {
                s.pop();
            } else {
                s = Solution::add_one(&s);
            }

            operations += 1;
        }

        operations
    }

    fn add_one(s: &str) -> String {
        let mut remainder = '1';
        let mut result: Vec<char> = vec![];

        for ch in s.chars().rev() {
            match (ch, remainder) {
                ('1', '1') => {
                    result.push('0');
                    remainder = '1';
                }
                ('1', _) => {
                    result.push('1');
                }
                (_, '1') => {
                    result.push('1');
                    remainder = '0';
                }
                _ => {
                    result.push(remainder);
                }
            }
        }

        if remainder == '1' {
            result.push('1');
        }

        result.iter().rev().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_one_ok() {
        let scenarios = vec![
            ("10".to_string(), "11".to_string()),
            ("101".to_string(), "110".to_string()),
            ("110".to_string(), "111".to_string()),
            ("111".to_string(), "1000".to_string()),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::add_one(&input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }

    #[test]
    fn it_works() {
        let scenarios = vec![
            ("1101".to_string(), 6),
            ("10".to_string(), 1),
            ("1".to_string(), 0),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::num_steps(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
