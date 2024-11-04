pub struct Solution;

impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let words = sentence
            .split_whitespace()
            .map(|word| {
                let first = word.chars().next().unwrap();
                let last = word.chars().last().unwrap();

                (first, last)
            })
            .collect::<Vec<_>>();

        let first = &words[0].0;
        let mut prev = &words[0].1;
        let last = &words[words.len() - 1].1;

        for (curr_first, curr_last) in words.iter().skip(1) {
            if prev != curr_first {
                return false;
            }

            prev = curr_last;
        }

        first == last
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ("leetcode exercises sound delightful".to_string(), true),
            ("leetcode".to_string(), false),
            ("leetcode easy".to_string(), false),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::is_circular_sentence(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
