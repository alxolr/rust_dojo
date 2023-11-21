struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        let mut arr = vec![];

        for n in nums {
            arr.push(n - Self::rev(n));
        }

        let mut dic = HashMap::new();
        let mut ans = 0;
        const MOD: i32 = 1_000_000_007;

        for n in arr {
            let entry = dic.entry(n).or_insert(0);

            ans = (ans + *entry) % MOD;
            *entry += 1;
        }

        ans
    }

    fn rev(n: i32) -> i32 {
        n.to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i32>()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_nice_pairs_ok() {
        let scenarios = vec![(vec![42, 11, 1, 97], 2), (vec![13, 10, 35, 24, 76], 4)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::count_nice_pairs(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
