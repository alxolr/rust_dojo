impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        use std::collections::{HashSet, HashMap};

        fn devowel(s: &str) -> String {
            s.chars()
                .map(|c| match c.to_ascii_lowercase() {
                    'a' | 'e' | 'i' | 'o' | 'u' => '*',
                    x => x,
                })
                .collect()
        }

        let mut exact = HashSet::new();
        let mut case_insensitive = HashMap::new();
        let mut vowel_error = HashMap::new();

        for word in &wordlist {
            exact.insert(word.clone());
            let lower = word.to_ascii_lowercase();
            case_insensitive.entry(lower.clone()).or_insert(word.clone());
            let devowelled = devowel(&lower);
            vowel_error.entry(devowelled).or_insert(word.clone());
        }

        queries
            .into_iter()
            .map(|query| {
                if exact.contains(&query) {
                    return query;
                }
                let lower = query.to_ascii_lowercase();
                if let Some(word) = case_insensitive.get(&lower) {
                    return word.clone();
                }
                let devowelled = devowel(&lower);
                if let Some(word) = vowel_error.get(&devowelled) {
                    return word.clone();
                }
                "".to_string()
            })
            .collect()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(
            (
                vec!["KiTe", "kite", "hare", "Hare"]
                    .into_iter()
                    .map(String::from)
                    .collect(),
                vec![
                    "kite", "Kite", "KiTe", "Hare", "HARE", "Hear", "hear", "keti", "keet", "keto",
                ]
                .into_iter()
                .map(String::from)
                .collect(),
            ),
            vec![
                "kite", "KiTe", "KiTe", "Hare", "hare", "", "", "KiTe", "", "KiTe",
            ]
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>(),
        )];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((wordlist, queries), expected))| {
                let result = Solution::spellchecker(wordlist, queries);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
