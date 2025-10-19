///
/// You are given a string s of even length consisting of digits from 0 to 9, and two integers a and
/// b. You can apply either of the following two operations any number of times and in any order on s:
/// - Add a to all odd indices of s (0-indexed). Digits post 9 are cycled back to 0. For example, if s = "3456" and a = 5, s becomes "3951".
/// - Rotate s to the right by b positions. For example, if s = "3456" and b = 1, s becomes "6345".
/// Return the lexicographically smallest string you can obtain by applying the above operations any number of times on s.
/// A string a is lexicographically smaller than a string b (of the same length) if in the first position where a and b differ, string a has a letter that appears earlier in the alphabet than the corresponding letter in b. For example, "0158" is lexicographically smaller than "0190" because the first position they differ is at the third letter, and '5' comes before '9'.
impl Solution {
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        use std::collections::{HashSet, VecDeque};

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut min_result = s.clone();

        queue.push_back(s.clone());
        visited.insert(s);

        while let Some(current) = queue.pop_front() {
            if current < min_result {
                min_result = current.clone();
            }

            // Try operation 1: add a to all odd indices
            let after_op1 = apply_operation_1(&current, a as usize);
            if !visited.contains(&after_op1) {
                visited.insert(after_op1.clone());
                queue.push_back(after_op1);
            }

            // Try operation 2: rotate right by b positions
            let after_op2 = rotate(&current, b as usize);
            if !visited.contains(&after_op2) {
                visited.insert(after_op2.clone());
                queue.push_back(after_op2);
            }
        }

        min_result
    }
}

fn apply_operation_1(s: &str, a: usize) -> String {
    s.chars()
        .flat_map(|x| x.to_digit(10))
        .enumerate()
        .map(|(idx, num)| {
            if idx % 2 != 0 {
                // odd indices
                (num + a as u32) % 10
            } else {
                num
            }
        })
        .map(|x| x.to_string())
        .collect()
}

fn rotate(s: &str, positions: usize) -> String {
    let len = s.len();
    let positions = positions % len; // Handle positions >= len

    // For right rotation by n positions, we skip (len - n) positions from the start
    s.chars().cycle().skip(len - positions).take(len).collect()
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (("5525", 9, 2), "2050"),
            (("74", 5, 1), "24"),
            (("0011", 4, 2), "0011"),
            (("43987654", 7, 3), "00553311"),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result =
                    Solution::find_lex_smallest_string(input.0.to_string(), input.1, input.2);
                assert_eq!(result, expected.to_string());
                println!("  ✓ scenario {}", idx + 1)
            });
    }

    #[test]
    fn test_rotate_ok() {
        assert_eq!(rotate("ABCD", 2), "CDAB".to_string());
        assert_eq!(rotate("1234", 3), "2341".to_string());
        assert_eq!(rotate("5525", 2), "2555".to_string());
    }
}
