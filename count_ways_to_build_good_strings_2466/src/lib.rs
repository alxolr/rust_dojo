pub struct Solution;

const MOD: i32 = 1000_000_007;
impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero_len: i32, one_len: i32) -> i32 {
        let mut dp = vec![0; high as usize + 1];
        dp[0] = 1;

        for length in 1..=high {
            if length >= zero_len {
                dp[length as usize] += dp[(length - zero_len) as usize];
            }

            if length >= one_len {
                dp[length as usize] += dp[(length - one_len) as usize];
            }

            dp[length as usize] %= MOD;
        }

        let mut total_count = 0;
        for length in low..=high {
            total_count += dp[length as usize];
            total_count %= MOD;
        }

        total_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ((3, 3, 1, 1), 8),
            ((2, 3, 1, 2), 5),
            ((200, 200, 10, 1), 764262396),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((low, high, zero, one), expected))| {
                let result = Solution::count_good_strings(low, high, zero, one);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
