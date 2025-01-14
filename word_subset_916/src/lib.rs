pub struct Solution;

fn are_vectors_subset(v1: &[i32], v2: &[i32]) -> bool {
    v1.iter().zip(v2.iter()).all(|(left, right)| left >= right)
}

impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let w1_dictionaries = words1.iter().fold(vec![], |mut acc, word| {
            let w_dictionary = word.bytes().fold(vec![0; 26], |mut acc, ch| {
                acc[(ch - b'a') as usize] += 1;
                acc
            });

            acc.push(w_dictionary);

            acc
        });

        let w2_dictionary = words2.iter().fold(vec![0; 26], |acc, word| {
            let word_count = word.bytes().fold(vec![0; 26], |mut acc, ch| {
                acc[(ch - b'a') as usize] += 1;

                acc
            });

            word_count
                .into_iter()
                .zip(acc.iter())
                .map(|(left, right)| left.max(*right))
                .collect()
        });

        w1_dictionaries
            .iter()
            .enumerate()
            .filter(|(_, dictionary)| are_vectors_subset(dictionary, &w2_dictionary))
            .map(|(idx, _)| words1[idx].clone())
            .collect()
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
                    vec![
                        "amazon".to_string(),
                        "apple".to_string(),
                        "facebook".to_string(),
                        "google".to_string(),
                        "leetcode".to_string(),
                    ],
                    vec!["e".to_string(), "o".to_string()],
                ),
                vec![
                    "facebook".to_string(),
                    "google".to_string(),
                    "leetcode".to_string(),
                ],
            ),
            (
                (
                    vec![
                        "amazon".to_string(),
                        "apple".to_string(),
                        "facebook".to_string(),
                        "google".to_string(),
                        "leetcode".to_string(),
                    ],
                    vec!["l".to_string(), "e".to_string()],
                ),
                vec![
                    "apple".to_string(),
                    "google".to_string(),
                    "leetcode".to_string(),
                ],
            ),
            (
                (
                    vec![
                        "amazon".to_string(),
                        "apple".to_string(),
                        "facebook".to_string(),
                        "google".to_string(),
                        "leetcode".to_string(),
                    ],
                    vec!["l".to_string(), "ee".to_string()],
                ),
                vec!["leetcode".to_string()],
            ),
            (
                (
                    vec![
                        "amazon".to_string(),
                        "apple".to_string(),
                        "facebook".to_string(),
                        "google".to_string(),
                        "leetcode".to_string(),
                    ],
                    vec!["lo".to_string(), "eo".to_string()],
                ),
                vec!["google".to_string(), "leetcode".to_string()],
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((w1, w2), expected))| {
                let result = Solution::word_subsets(w1, w2);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
