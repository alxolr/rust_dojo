use std::mem::swap;

struct Solution;

const MOD: i32 = 1_000_000_007;

impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        let mut zero = 0;
        let mut one = 0;
        let mut two = 1;

        for ch in corridor.chars() {
            if ch == 'S' {
                zero = one;
                swap(&mut one, &mut two);
            } else {
                two = (two + zero) % MOD;
            }
        }

        zero
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_ways_ok() {
        let scenarios = vec![
            ("S".to_string(), 0),
            ("SSPPSPS".to_string(), 3),
            ("PPSPSP".to_string(), 1),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::number_of_ways(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
