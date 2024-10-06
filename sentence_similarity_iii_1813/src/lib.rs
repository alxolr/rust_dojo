pub struct Solution;

impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        let s1_words: Vec<&str> = sentence1.split_ascii_whitespace().collect();
        let s2_words: Vec<&str> = sentence2.split_ascii_whitespace().collect();

        let s1_len = s1_words.len();
        let s2_len = s2_words.len();

        let mut i = 0;
        let mut j = 0;

        // find how many matching words from the start
        while i < s1_len && i < s2_len && s1_words[i] == s2_words[i] {
            i += 1;
        }

        // find how many matching words from the end
        // but make sure to not double count them
        while i + j < s1_len
            && i + j < s2_len
            && s1_words[s1_len - 1 - j] == s2_words[s2_len - 1 - j]
        {
            j += 1;
        }

        let total_words = i + j;
        total_words == s1_len || total_words == s2_len
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
                    "Hello Jane".to_string(),
                    "Hello my name is Jane".to_string(),
                ),
                true,
            ),
            (("Eating right now".to_string(), "Eating".to_string()), true),
            (
                ("Frog cool".to_string(), "Frogs are cool".to_string()),
                false,
            ),
            (
                ("Ogn WtWj HneS".to_string(), "Ogn WtWj HneS".to_string()),
                true,
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((s1, s2), expected))| {
                let result = Solution::are_sentences_similar(s1, s2);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
