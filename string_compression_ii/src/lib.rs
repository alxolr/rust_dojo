#![allow(dead_code)]

use std::collections::HashMap;
struct Solution;

/// This function uses a dynamic programing approach to find the minimum numbers of letters to count
/// The function uses memoization to count fast the total posibility of cases
fn length_compression(memo: &mut HashMap<(i32, String), i32>, s: String, k: i32) -> i32 {
    if let Some(result) = memo.get(&(k, s.clone())) {
        return *result;
    }

    let compressed = run_length_encoding(&s);

    if k == 0 {
        let result = compressed.len() as i32;
        memo.insert((k, s.clone()), result);

        return result;
    }

    // if the original string is the same after compression a short circuit will be to
    // just remove the chars from length and return
    if compressed == s {
        let result = s.len() as i32 - k;
        memo.insert((k, s.clone()), result);

        return result;
    }

    let mut min_result = i32::MAX;
    for (idx, _) in s.chars().enumerate() {
        let mut sub_str = s.clone();
        sub_str.remove(idx);

        let check = length_compression(memo, sub_str, k - 1);
        min_result = min_result.min(check);
    }

    memo.insert((k, s), min_result);

    min_result
}

/// This struct represents a solution to the problem of compressing a string with certain constraints.
/// The `get_length_of_optimal_compression` method calculates the length of the optimal compressed string
/// given the original string `s` and the maximum number of characters `k` that can be deleted.
impl Solution {
    pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
        let mut memo = HashMap::new();

        length_compression(&mut memo, s, k)
    }
}

/// This function performs run-length encoding on a string `s` and returns the compressed string.
/// Run-length encoding replaces consecutive repeated characters with the character followed by the count of repetitions.
/// For example, "aaabbc" would be compressed to "a3b2c".
fn run_length_encoding(s: &str) -> String {
    let mut result = String::new();
    let mut buffer = vec![];

    for ch in s.chars() {
        if buffer.is_empty() {
            buffer.push(ch);
        } else if let Some(buffer_ch) = buffer.last() {
            if buffer_ch == &ch {
                // if the last one is of the same kind we continue pushing
                buffer.push(ch);
            } else {
                let letters = drain_buffer(&buffer);
                result.push_str(&letters);

                buffer = vec![ch];
            }
        }
    }

    // we need to drain the last part of the buffer
    let letters = drain_buffer(&buffer);
    result.push_str(&letters);

    result
}

/// This function drains the buffer and returns the compressed string.
/// If there is only one character in the buffer, it is returned as is.
/// If there are multiple characters, the last character is followed by the count of repetitions.
fn drain_buffer(buffer: &[char]) -> String {
    let mut result = String::new();

    let count = buffer.len();
    if let Some(last) = buffer.last() {
        if count > 1 {
            result.push_str(&format!("{}{}", last, count))
        } else {
            result.push(*last);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_length_encoding_ok() {
        // Test scenarios for the run_length_encoding function
        let scenarios = vec![
            ("", "".to_string()),
            ("aaaaa", "a5".to_string()),
            ("aaabc", "a3bc".to_string()),
            ("abc", "abc".to_string()),
            ("aaabbc", "a3b2c".to_string()),
            ("aaabbccccccc", "a3b2c7".to_string()),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = run_length_encoding(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }

    #[test]
    fn test_get_length_of_optimal_compression_ok() {
        let scenarios = vec![
            (("aaabcccd".to_string(), 2), 4),
            (("aaabbaa".to_string(), 2), 2),
            (("aaaaaaaaaaa".to_string(), 0), 3),
            (("aabaabbcbbbaccc".to_string(), 6), 4),
            (("abcdefghijklmnopqrstuvwxyz".to_string(), 16), 10),
            // (("ccacbaacabaabbcaccbabccacbbac".to_string(), 9), 4),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((s, k), expected))| {
                let result = Solution::get_length_of_optimal_compression(s, k);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
