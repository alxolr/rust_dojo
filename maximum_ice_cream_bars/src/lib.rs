struct Solution;

impl Solution {
    pub fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
        let mut coins = coins;
        let mut costs = costs.clone();
        costs.sort();
        
        let mut ice_creams = 0;

        for idx in 0..costs.len() {
            if costs[idx] <= coins {
                ice_creams += 1;
                coins -= costs[idx];
            } else {
                break;
            }
        }

        ice_creams as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scenarios() {
        let scenarios = vec![
            ((vec![1, 3, 2, 4, 1], 7), 4),
            ((vec![10, 6, 8, 7, 7, 8], 5), 0),
            ((vec![1, 6, 3, 1, 2, 5], 20), 6),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::max_ice_cream(input.0, input.1);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
