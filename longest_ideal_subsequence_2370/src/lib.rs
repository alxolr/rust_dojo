pub struct Solution;

impl Solution {
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        let (k, mut dp) = (k as usize, vec![0; 256]);

        for b in s.bytes() {
            let b = b as usize;
            dp[b] = 1 + ((b - k)..=(b + k)).fold(0, |val, i| val.max(dp[i]));
        }
        
        *dp.iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(("acfgbd".to_string(), 2), 4), (("abcd".to_string(), 3), 4)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((s, k), expected))| {
                let result = Solution::longest_ideal_string(s, k);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
