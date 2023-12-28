use std::fmt::format;

struct Solution;

///
/// Observations:
/// 1. We need to implement the compression itself
/// 2. Interesting to notice that we prioritize to delete leftover caracters

impl Solution {
    pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
        1
    }
}

fn run_length_encoding(s: &str) -> String {
    // we will iterate through the characters of the string
    // and store everyting in a buffer (stack) until the next character diverge
    let mut result = String::new();
    let mut buffer = vec![];

    for ch in s.chars() {
        if buffer.is_empty() {
            buffer.push(ch);
        } else {
            if let Some(buffer_ch) = buffer.last() {
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
    }

    // we need to drain the last part of the buffer
    let letters = drain_buffer(&buffer);
    result.push_str(&letters);

    result
}

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
        let scenarios = vec![
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
