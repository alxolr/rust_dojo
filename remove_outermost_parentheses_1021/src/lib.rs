use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut result = String::new();
        let mut deque: VecDeque<char> = VecDeque::new();
        let mut open_parenthesis = 0;
        let mut closed_parenthesis = 0;

        for ch in s.chars() {
            deque.push_back(ch);
            match ch {
                '(' => open_parenthesis += 1,
                _ => closed_parenthesis += 1,
            }

            if closed_parenthesis == open_parenthesis {
                deque.pop_front();
                deque.pop_back();

                let primitive = deque.iter().collect::<String>();
                result.push_str(&primitive);
                open_parenthesis = 0;
                closed_parenthesis = 0;
                deque.clear();
            }
        }

        result.push_str(&deque.into_iter().collect::<String>());

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![("(()())(())".to_string(), "()()()".to_string())];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::remove_outer_parentheses(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
