use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let groups = strs.into_iter().fold(HashMap::new(), |mut acc, it| {
            let hash = it.as_bytes().iter().fold(vec![0; 26], |mut acc, b| {
                acc[(b - b'a') as usize] += 1;
                
                acc
            });

            let entry = acc.entry(hash).or_insert(vec![]);
            entry.push(it);

            acc
        });

        groups.into_values().collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_ok() {
        let scenarios = vec![
            // (
            //     vec![
            //         "eat".to_string(),
            //         "tea".to_string(),
            //         "tan".to_string(),
            //         "ate".to_string(),
            //         "nat".to_string(),
            //         "bat".to_string(),
            //     ],
            //     vec![
            //         vec!["eat".to_string(), "tea".to_string(), "ate".to_string()],
            //         vec!["tan".to_string(), "nat".to_string()],
            //         vec!["bat".to_string()],
            //     ],
            // ),
            (
                vec!["abbbbbbbbbbb".to_string(), "aaaaaaaaaaab".to_string()],
                vec![
                    vec!["abbbbbbbbbbb".to_string()],
                    vec!["aaaaaaaaaaab".to_string()],
                ],
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::group_anagrams(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
