struct Solution;
impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let vowels = "AEIOUaeiou";
        let mut result = String::new();

        let mut vowel_count =
            s.chars()
                .filter(|c| vowels.contains(*c))
                .fold([0; 10], |mut acc, c| {
                    let idx = vowels.find(c).unwrap();
                    acc[idx] += 1;
                    acc
                });

        for c in s.chars() {
            if vowels.contains(c) {
                let idx: usize = vowel_count.iter().position(|&x| x > 0).unwrap();
                vowel_count[idx] -= 1;
                result.push(vowels.chars().nth(idx).unwrap());
            } else {
                result.push(c);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_vowels_ok() {
        let scenarios = vec![("lEetcOde".to_string(), "lEOtcede".to_string())];

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
