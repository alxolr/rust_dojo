use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut result = "".to_string();
        let mut freq = s.as_bytes().iter().fold(HashMap::new(), |mut acc, ch| {
            let entry = acc.entry(ch).or_insert(0);
            *entry += 1;

            acc
        });

        for b in order.as_bytes() {
            let count = freq.get(b).unwrap_or(&0);
            for _ in 0..*count {
                result.push(*b as char);
                freq.remove(b);
            }
        }

        for (entry, count) in freq {
            for _ in 0..count {
                result.push(*entry as char);
            }
        }

        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(("cba".to_string(), "abcd".to_string()), "cbad")];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((order, s), expected))| {
                let result = Solution::custom_sort_string(order, s);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
