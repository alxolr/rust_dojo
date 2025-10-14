use std::collections::HashMap;

impl Solution {
    pub fn maximum_total_damage(powers: Vec<i32>) -> i64 {
        // Step 1: Count frequencies of each power level
        let mut frequencies = HashMap::new();
        for power in powers {
            *frequencies.entry(power).or_insert(0) += 1;
        }

        // Step 2: Convert to sorted vector of (damage, count) pairs
        let mut spells: Vec<(i32, i32)> = frequencies.into_iter().collect();
        spells.sort_by_key(|&(damage, _)| damage);
        
        let n = spells.len();
        if n == 0 {
            return 0;
        }

        // Step 3: Precompute next valid index for each spell
        // next_valid[i] = smallest j > i where spells[j].damage > spells[i].damage + 2
        let mut next_valid = vec![n; n]; // Initialize with n (out of bounds)
        for i in 0..n {
            for j in (i + 1)..n {
                if spells[j].0 > spells[i].0 + 2 {
                    next_valid[i] = j;
                    break;
                }
            }
        }

        // Step 4: Tabular DP - fill from right to left
        // dp[i] = maximum damage we can get from spells[i..n]
        let mut dp = vec![0i64; n + 1]; // dp[n] = 0 (base case)
        
        for i in (0..n).rev() { // Fill from right to left
            let (damage, count) = spells[i];
            
            // Option 1: Take current spell
            let take_current = (damage as i64) * (count as i64) + dp[next_valid[i]];
            
            // Option 2: Skip current spell
            let skip_current = dp[i + 1];
            
            // Take the maximum
            dp[i] = i64::max(take_current, skip_current);
        }

        return dp[0]; // Answer is maximum damage from all spells
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(vec![1, 1, 3, 4], 6), (vec![7, 1, 6, 6], 13)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::maximum_total_damage(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
