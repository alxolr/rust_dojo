pub struct Solution;

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut stack = vec![];
        let mut max_depth = 0;

        for curr in s.chars() {
            match curr {
                ')' => {
                    stack.pop().unwrap();
                }
                '(' => {
                    stack.push(curr);

                    max_depth = max_depth.max(stack.len());
                }
                _ => {}
            }
        }

        max_depth as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ("(1+(2*3)+((8)/4))+1".to_string(), 3),
            ("(1)+((2))+(((3)))".to_string(), 3),
            ("()".to_string(), 1),
            ("".to_string(), 0),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::max_depth(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
