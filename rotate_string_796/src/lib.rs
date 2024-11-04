pub struct Solution;

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        let full_cycle = format!("{}{}", s, s);

        full_cycle.contains(&goal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (("abcdef".to_string(), "abcdef".to_string()), true),
            (("abcde".to_string(), "bcdea".to_string()), true),
            (("abcde".to_string(), "abced".to_string()), false),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((s1, s2), expected))| {
                let result = Solution::rotate_string(s1, s2);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
