pub struct Solution;

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut open_count = 0;
        let mut close_count = 0;
        let len = s.len() - 1;

        for idx in 0..=len {
            match &s[idx..=idx] {
                "(" | "*" => open_count += 1,
                _ => open_count -= 1,
            }

            match &s[len - idx..=len - idx] {
                ")" | "*" => close_count += 1,
                _ => close_count -= 1,
            }

            if open_count < 0 || close_count < 0 {
                return false;
            }
        }

        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ("(*)".to_string(), true),
            ("()".to_string(), true),
            ("(*))".to_string(), true),
            ("())".to_string(), false),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::check_valid_string(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
