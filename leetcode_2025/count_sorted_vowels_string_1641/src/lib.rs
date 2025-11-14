impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![vec![1; 5]; n as usize];

        for i in 1..n {
            for letter in 0..5 {
                dp[i][letter] = dp[i - 1].iter().skip(letter).sum();
            }
        }

        dp[n - 1].iter().sum()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(1, 5), (2, 15), (3, 35)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::count_vowel_strings(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
