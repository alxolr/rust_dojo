pub struct Solution;

const MOD: i32 = 1_000_000_007;

impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        let mut prev: Vec<i32> = vec![0; k as usize + 1];

        for i in 1..=n {
            let mut cur: Vec<i32> = vec![0; k as usize + 1];
            cur[0] = 1;
            
            for j in 1..=k {
                let cnt = (prev[j as usize] + MOD
                    - if j - i >= 0 { prev[(j - i) as usize] } else { 0 })
                    % MOD;
                cur[j as usize] = (cnt + cur[j as usize - 1]) % MOD;
            }
            prev = cur;
        }

        (prev[k as usize] + MOD - if k > 0 { prev[k as usize - 1] } else { 0 }) % MOD
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_ok() {
        let scenarios = vec![((3, 0), 1), ((3, 1), 2)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((n, k), expected))| {
                let result = Solution::k_inverse_pairs(n, k);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
