impl Solution {
    fn count_one_zeros(s: &str) -> (i32, i32) {
        let len = s.len() as i32;
        let zeros = s.chars().filter(|x| x == &'0').count() as i32;

        (zeros, len - zeros)
    }

    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut dp = vec![vec![0; (n + 1) as usize]; (m + 1) as usize];

        for string in strs.iter() {
            let (zeros, ones) = Self::count_one_zeros(string);

            // CRITICAL: Loop backwards through m and n!
            for m_left in (zeros..=m).rev() {
                for n_left in (ones..=n).rev() {
                    dp[m_left as usize][n_left as usize] = i32::max(
                        dp[m_left as usize][n_left as usize],                    // skip
                        1 + dp[(m_left - zeros) as usize][(n_left - ones) as usize], // take
                    );
                }
            }
        }

        dp[m as usize][n as usize]
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
