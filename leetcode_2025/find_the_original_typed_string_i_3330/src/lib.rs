impl Solution {
    pub fn possible_string_count(word: String) -> i32 {
        let chars = word.chars().collect::<Vec<_>>();
        let last = chars[chars.len() - 1];

        let possible_words = chars
            .iter()
            .rev()
            .skip(1)
            .scan(last, |acc, current| {
                let is_same = acc == current;
                *acc = *current;
                Some(if is_same { 1 } else { 0 })
            })
            .sum::<i32>();

        possible_words + 1
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ("abbcccc".to_string(), 5),
            ("abcd".to_string(), 1),
            ("aaaa".to_string(), 4),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::possible_string_count(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
