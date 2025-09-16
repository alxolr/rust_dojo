
impl Solution {
    pub fn sort_vowels(s: String) -> String {
        fn is_vowel(c: char) -> bool {
            matches!(c, 'A' | 'E' | 'I' | 'O' | 'U' | 'a' | 'e' | 'i' | 'o' | 'u')
        }

        let mut chars: Vec<char> = s.chars().collect();
        let mut vowel_indices: Vec<usize> = vec![];
        let mut vowels = vec![];

        for (i, &c) in chars.iter().enumerate() {
            if is_vowel(c) {
                vowel_indices.push(i);
                vowels.push(c);
            }
        }

        vowels.sort_by(|a, b| (*a as u8).cmp(&(*b as u8)));

        for (idx, &vowel) in vowel_indices.iter().zip(vowels.iter()) {
            chars[*idx] = vowel;
        }

        chars.into_iter().collect()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ("lEetcOde".to_string(), "lEOtcede".to_string()),
            ("lYmpH".to_string(), "lYmpH".to_string()),
            ("aeiou".to_string(), "aeiou".to_string()),
            ("".to_string(), "".to_string()),
            ("aA".to_string(), "Aa".to_string()),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::sort_vowels(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
