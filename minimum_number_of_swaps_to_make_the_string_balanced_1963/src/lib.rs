pub struct Solution;

impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut balance = 0;
        let mut swaps = 0;

        for ch in s.chars() {
            if ch == '[' {
                balance += 1;
            } else {
                balance -= 1;

                if balance < 0 {
                    swaps += 1;
                    balance = 1;
                }
            }
        }

        swaps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ("][][".to_string(), 1),
            ("]]][[[".to_string(), 2),
            ("[]".to_string(), 0),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::min_swaps(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
