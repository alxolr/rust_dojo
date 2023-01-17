use std::cmp;
struct Solution;

impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        let mut zeros = 0;
        let mut ones = 0;
        
        for c in s.chars() {
            if c == '0' {
                zeros = cmp::min(ones, zeros + 1)
            } else {
                ones += 1
            }
        }
        zeros
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let scenarios = vec![
            ("00110".to_string(), 1),
            ("010110".to_string(), 2),
            ("00011000".to_string(), 2),
            ("10011111110010111011".to_string(), 5),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::min_flips_mono_incr(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
