use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let freqs = s.chars().fold(HashMap::new(), |mut acc, b| {
            let entry = acc.entry(b).or_insert(0);
            *entry += 1;

            acc
        });

        let mut byte_freqs = s
            .chars()
            .map(|a| (a, *freqs.get(&a).unwrap_or(&0)))
            .collect::<Vec<_>>();

        byte_freqs.sort_by(|a, b| {
            if a.1 == b.1 {
                a.0.cmp(&b.0)
            } else {
                b.1.cmp(&a.1)
            }
        });

        byte_freqs.iter().map(|a| a.0).collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_ok() {
        let scenarios = vec![
            ("tree".to_string(), "eetr".to_string()),
            ("cccaaa".to_string(), "aaaccc".to_string()),
            ("Aabb".to_string(), "bbAa".to_string()),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::frequency_sort(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
