pub struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];
        let signs = vec!["+", "-", "*", "/"];

        for token in tokens.iter() {
            if !signs.contains(&token.as_str()) {
                let parsed = token.parse::<i32>().unwrap_or(0);
                stack.push(parsed);
            } else {
                // pop two operands
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();

                let result = match token.as_str() {
                    "+" => left + right,
                    "-" => left - right,
                    "/" => left / right,
                    "*" => left * right,
                    _ => panic!("invalid token"),
                };

                stack.push(result);
            }
        }

        stack[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_ok() {
        let scenarios = vec![
            (
                vec![
                    "2".to_string(),
                    "1".to_string(),
                    "+".to_string(),
                    "3".to_string(),
                    "*".to_string(),
                ],
                9,
            ),
            (
                vec![
                    "4".to_string(),
                    "13".to_string(),
                    "5".to_string(),
                    "/".to_string(),
                    "+".to_string(),
                ],
                6,
            ),
            (
                vec![
                    "10".to_string(),
                    "6".to_string(),
                    "9".to_string(),
                    "3".to_string(),
                    "+".to_string(),
                    "-11".to_string(),
                    "*".to_string(),
                    "/".to_string(),
                    "*".to_string(),
                    "17".to_string(),
                    "+".to_string(),
                    "5".to_string(),
                    "+".to_string(),
                ],
                22,
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::eval_rpn(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
