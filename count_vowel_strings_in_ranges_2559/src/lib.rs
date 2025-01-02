pub struct Solution;

fn is_vowel_word(word: &str) -> bool {
    let vowels = b"aeiou";
    let bytes = word.as_bytes();

    vowels.contains(&bytes[0]) && vowels.contains(&bytes[bytes.len() - 1])
}

impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let vowels_words: Vec<i32> = words
            .iter()
            .map(|word| if is_vowel_word(word) { 1 } else { 0 })
            .collect();

        queries
            .into_iter()
            .map(|query| {
                let start = query[0] as usize;
                let end = query[1] as usize;
                vowels_words[start..=end].iter().sum()
            })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(
            (
                vec![
                    "aba".to_string(),
                    "bcb".to_string(),
                    "ece".to_string(),
                    "aa".to_string(),
                    "e".to_string(),
                ],
                vec![vec![0, 2], vec![1, 4], vec![1, 1]],
            ),
            vec![2, 3, 0],
        )];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((words, queries), expected))| {
                let result = Solution::vowel_strings(words, queries);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
