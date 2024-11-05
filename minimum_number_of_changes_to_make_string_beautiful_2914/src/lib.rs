pub struct Solution;

impl Solution {
    pub fn min_changes(s: String) -> i32 {
        s.as_bytes()
            .chunks(2)
            .filter(|chunk| chunk[0] != chunk[1])
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ("1001".to_string(), 2),
            ("0000".to_string(), 0),
            ("11000111".to_string(), 1),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::min_changes(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
