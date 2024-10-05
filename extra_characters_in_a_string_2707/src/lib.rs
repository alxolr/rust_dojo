use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    fn dp(cache: &mut HashMap<usize, i32>, idx: usize, s: &str, dictionary: &HashSet<&str>) -> i32 {
        if let Some(result) = cache.get(&idx) {
            return *result;
        }

        if idx == s.len() {
            return 0;
        }

        let mut res = 1 + Self::dp(cache, idx + 1, s, dictionary); // skip the current character

        for j in idx..s.len() {
            let prefix = &s[idx..j + 1];

            if dictionary.contains(prefix) {
                res = res.min(Self::dp(cache, j + 1, s, dictionary));
            }
        }

        cache.insert(idx, res);

        res
    }

    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        // Transform dictionary Vec<String> into HashSet to O(1) check if the word exists
        let dictionary = dictionary
            .iter()
            .map(|s| s.as_str())
            .collect::<HashSet<_>>();

        let mut cache = HashMap::new();
        Self::dp(&mut cache, 0, s.as_str(), &dictionary)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (
                (
                    "leetscode".to_string(),
                    vec![
                        "leet".to_string(),
                        "code".to_string(),
                        "leetcode".to_string(),
                    ],
                ),
                1,
            ),
            (
                (
                    "sayhelloworld".to_string(),
                    vec!["hello".to_string(), "world".to_string()],
                ),
                3,
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((s, dic), expected))| {
                let result = Solution::min_extra_char(s, dic);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
