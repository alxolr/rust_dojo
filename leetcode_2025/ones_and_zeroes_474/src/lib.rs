impl Solution {
    fn count_one_zeros(s: &str) -> (i32, i32) {
        let len = s.len() as i32;
        let zeros = s.chars().filter(|x| x == &'0').count() as i32;

        (zeros, len - zeros)
    }

    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let len = strs.len();
        let mut dp = vec![vec![vec![0; (n + 1) as usize]; (m + 1) as usize]; len + 1];

        for idx in (0..len).rev() {
            let (zeros, ones) = Self::count_one_zeros(&strs[idx]);

            for zero_cap in 0..=m {
                for one_cap in 0..=n {
                    if zeros <= zero_cap && ones <= one_cap {
                        let take = 1 + dp[idx + 1][(zero_cap - zeros) as usize][(one_cap - ones) as usize];
                        let not_take = dp[idx + 1][zero_cap as usize][one_cap as usize];
                        dp[idx][zero_cap as usize][one_cap as usize] = take.max(not_take);
                    } else {
                        dp[idx][zero_cap as usize][one_cap as usize] = dp[idx + 1][zero_cap as usize][one_cap as usize];
                    }
                }
            }
        }

        dp[0][m as usize][n as usize]
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (
                (
                    vec![
                        "10".to_string(),
                        "0001".to_string(),
                        "111001".to_string(),
                        "1".to_string(),
                        "0".to_string(),
                    ],
                    5,
                    3,
                ),
                4,
            ),
            (
                (
                    vec!["10".to_string(), "0".to_string(), "1".to_string()],
                    1,
                    1,
                ),
                2,
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::find_max_form(input.0, input.1, input.2);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
