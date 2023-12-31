use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut dictionary: HashMap<char, (i32, Option<i32>)> = HashMap::new();

        for (idx, ch) in s.chars().enumerate() {
            if let Some(entry) = dictionary.get_mut(&ch) {
                entry.1 = Some(idx as i32);
            } else {
                dictionary.insert(ch, (idx as i32, None));
            }
        }

        let result = dictionary.values().fold(-1, |acc, (start, maybe_end)| {
            let value = if let Some(end) = maybe_end {
                end - start - 1
            } else {
                -1
            };

            acc.max(value)
        });

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ("aa".to_string(), 0),
            ("abca".to_string(), 2),
            ("cbzxy".to_string(), -1),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::max_length_between_equal_characters(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
