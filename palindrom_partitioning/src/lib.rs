/// Given a string s, partition s such that every substring
/// of the partition is a palindrome.
/// Return all possible palindrome partitioning of s.

pub struct Solution;

fn is_palindrom(s: &str) -> bool {
    &(s.clone().chars().rev().collect::<String>()) == s
}

fn dfs(res: &mut Vec<Vec<String>>, s: &str, start: usize, current_list: &mut Vec<String>) {
    if start >= s.len() {
        res.push(current_list.clone());
    }

    for end in start..s.len() {
        let sub_str = &s[start..end + 1];

        if is_palindrom(sub_str) {
            current_list.push(sub_str.to_string());
            dfs(res, s, end + 1, current_list);

            current_list.pop();
        }
    }
}

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut res = vec![];
        let mut curr_list = vec![];

        dfs(&mut res, &s, 0, &mut curr_list);

        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (
                "aab".to_string(),
                vec![
                    vec!["a".to_string(), "a".to_string(), "b".to_string()],
                    vec!["aa".to_string(), "b".to_string()],
                ],
            ),
            ("a".to_string(), vec![vec!["a".to_string()]]),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::partition(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
