pub struct Solution;

impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut result = vec![];
        let mut min_freq = [std::usize::MAX; 26]; // Initialize with maximum value

        // For each word, calculate the frequency of each letter
        for word in words.iter() {
            let mut freq = [0; 26];
            for byte in word.as_bytes() {
                let char_idx = (byte - 'a' as u8) as usize;
                freq[char_idx] += 1;
            }

            // Update the minimum frequency for each letter
            for i in 0..26 {
                min_freq[i] = min_freq[i].min(freq[i]);
            }
        }

        // For each letter, if its minimum frequency is more than 0, add it to the result
        for letter_idx in 0..26 {
            for _ in 0..min_freq[letter_idx] {
                result.push(((letter_idx as u8 + 'a' as u8) as char).to_string())
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
        let scenarios = vec![
            (
                vec![
                    "bella".to_string(),
                    "label".to_string(),
                    "roller".to_string(),
                ],
                vec!["e".to_string(), "l".to_string(), "l".to_string()],
            ),
            (
                vec!["cool".to_string(), "lock".to_string(), "cook".to_string()],
                vec!["c".to_string(), "o".to_string()],
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::common_chars(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
