pub struct Solution;

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let all_capitals = word.to_uppercase();

        if all_capitals == word {
            true
        } else {
            let word_iter = word.chars();
            let all_other_is_lower = word_iter.skip(1).filter(|c| c.is_uppercase()).count() == 0;

            all_other_is_lower
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ("USA".to_string(), true),
            ("FlaG".to_string(), false),
            ("Google".to_string(), true),
            ("g".to_string(), true),
            ("leetcode".to_string(), true),
        ];

        scenarios.into_iter().for_each(|(word, expected)| {
            let result = Solution::detect_capital_use(word);

            assert_eq!(result, expected);
        });
    }
}
