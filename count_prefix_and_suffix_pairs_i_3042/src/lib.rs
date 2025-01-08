pub struct Solution;

fn is_prefix_suffix(s1: &str, s2: &str) -> bool {
    if s2.len() < s1.len() {
        return false;
    }

    let s2_prefix = &s2[0..s1.len()];
    let s2_suffix = &s2[s2.len() - s1.len()..];

    s1 == s2_prefix && s1 == s2_suffix
}

impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        words
            .iter()
            .enumerate()
            .flat_map(|(i, word)| {
                words
                    .iter()
                    .enumerate()
                    .filter(move |(j, _)| i < *j)
                    .map(move |(_, other_word)| (word.clone(), other_word.clone()))
            })
            .filter(|(word1, word2)| is_prefix_suffix(word1, word2))
            .count() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (
                vec![
                    "a".to_string(),
                    "aba".to_string(),
                    "ababa".to_string(),
                    "aa".to_string(),
                ],
                4,
            ),
            (
                vec![
                    "pa".to_string(),
                    "papa".to_string(),
                    "ma".to_string(),
                    "mama".to_string(),
                ],
                2,
            ),
            (vec!["abab".to_string(), "ab".to_string()], 0),
            (vec!["bb".to_string(), "bb".to_string()], 1),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::count_prefix_suffix_pairs(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
