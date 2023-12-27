struct Solution;

const MOD: i32 = 1_000_000_007;

impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_rolls_to_target_ok() {
        let scenarios = vec![((1, 6, 3), 1), ((2, 6, 7), 6), ((30, 30, 500), 222616187)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((n, k, target), expected))| {
                let result = Solution::num_rolls_to_target(n, k, target);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
