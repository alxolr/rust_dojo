pub struct Solution;

fn generate(open: usize, closed: usize, acc: &Vec<char>) -> Vec<String> {
    if open == 0 && closed == 0 {
        return vec![acc.iter().collect()];
    }

    let mut result = Vec::new();

    if open > 0 {
        let mut new_acc = acc.clone();
        new_acc.push('(');
        result.extend(generate(open - 1, closed, &new_acc));
    }

    if closed > open {
        let mut new_acc = acc.clone();
        new_acc.push(')');
        result.extend(generate(open, closed - 1, &new_acc));
    }

    result
}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let pairs = n as usize;
        let results = generate(pairs, pairs, &vec![]);

        results
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (1, vec!["()".to_string()]),
            (2, vec!["(())".to_string(), "()()".to_string()]),
            (
                3,
                vec![
                    "((()))".to_string(),
                    "(()())".to_string(),
                    "(())()".to_string(),
                    "()(())".to_string(),
                    "()()()".to_string(),
                ],
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::generate_parenthesis(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
