use std::collections::{HashMap, VecDeque};

pub struct Solution;

impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        let mut alphabet = "abcdefghijklmnopqrstuvwxyz"
            .chars()
            .collect::<VecDeque<char>>();

        let cypher: HashMap<char, char> =
            key.chars()
                .filter(|c| c.is_alphabetic())
                .fold(HashMap::new(), |mut acc, ch| {
                    if acc.get(&ch).is_none() {
                        if let Some(value) = alphabet.pop_front() {
                            acc.insert(ch, value);
                        }
                    }

                    acc
                });

        message
            .chars()
            .map(|ch| match cypher.get(&ch) {
                Some(val) => *val,
                None => ch, // spaces
            })
            .collect::<_>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_ok() {
        let scenarios = vec![(
            (
                "the quick brown fox jumps over the lazy dog".to_string(),
                "vkbs bs t suepuv".to_string(),
            ),
            "this is a secret".to_string(),
        )];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((key, message), expected))| {
                let result = Solution::decode_message(key, message);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
