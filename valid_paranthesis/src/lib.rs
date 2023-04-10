use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];

        let open_brackets = vec!['(', '[', '{'];
        let mut complements = HashMap::new();
        complements.insert(')', '(');
        complements.insert(']', '[');
        complements.insert('}', '{');

        for c in s.chars() {
            if open_brackets.contains(&c) {
                stack.push(c)
            } else {
                let last = stack.pop();
                if last.is_none() {
                    return false;
                }

                if complements.get(&c).unwrap() != &last.unwrap() {
                    return false;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::is_valid("()".to_string()), true);
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
        assert_eq!(Solution::is_valid("(]".to_string()), false);
        assert_eq!(Solution::is_valid("([)]".to_string()), false);
        assert_eq!(Solution::is_valid("{[]}".to_string()), true);
    }
}
