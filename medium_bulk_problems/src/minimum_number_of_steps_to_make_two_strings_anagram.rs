use std::collections::HashMap;

pub struct Solution;

fn char_frequency(s: &str) -> HashMap<char, i32> {
    s.chars().fold(HashMap::new(), |mut acc, ch| {
        let entry = acc.entry(ch).or_insert(0);
        *entry += 1;

        acc
    })
}

impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let freq1 = char_frequency(&s);
        let freq2 = char_frequency(&t);

        freq2
            .iter()
            .map(|(ch, freq2_count)| {
                let freq1_count = freq1.get(ch).unwrap_or(&0);
                let result = freq2_count - freq1_count;

                if result > 0 {
                    result
                } else {
                    0
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_ok() {
        let scenarios = vec![
            (("bab".to_string(), "aba".to_string()), 1),
            (("leetcode".to_string(), "practice".to_string()), 5),
            (("anagram".to_string(), "mangaar".to_string()), 0),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((s, t), expected))| {
                let result = Solution::min_steps(s, t);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
