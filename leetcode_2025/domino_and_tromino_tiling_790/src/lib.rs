const MOD: i32 = 1000_000_007;

impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        if n <= 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        if n == 2 {
            return 2;
        }
        if n == 3 {
            return 5;
        }

        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[1] = 1;
        dp[2] = 2;
        dp[3] = 5;

        for i in 4..=n {
            // dp[i] = 2 * dp[i-1] + dp[i-3]
            dp[i] = ((2 * dp[i - 1]) % MOD + dp[i - 3]) % MOD;
        }

        dp[n] as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(1, 1), (2, 2), (3, 5), (4, 11), (5, 24)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::num_tilings(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
