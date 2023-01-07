struct Solution;

fn rotation_sum(prices: &[i32], idx: usize) -> i32 {
    let mut gas_tank = prices[idx];
    let mut it = if idx + 1 >= prices.len() { 0 } else { idx + 1 };

    while it != idx {
        if prices[it] < 0 {
            if gas_tank + prices[it] < 0 {
                return -1;
            }
        }

        gas_tank += prices[it];
        it = if it + 1 >= prices.len() { 0 } else { it + 1 }
    }

    gas_tank
}

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let prices = gas
            .iter()
            .zip(cost.iter())
            .map(|(g, c)| g - c)
            .collect::<Vec<_>>();
        let sum: i32 = prices.iter().sum();

        if sum < 0 {
            return -1;
        }

        for idx in 0..prices.len() {
            if prices[idx] >= 0 && gas[idx] != 0 {
                let sum = rotation_sum(&prices, idx);
                if sum >= 0 {
                    return idx as i32;
                }
            }
        }

        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_test_scenarios() {
        let scenarios = vec![
            ((vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]), 3),
            ((vec![2, 3, 4], vec![3, 4, 3]), -1),
            ((vec![5, 8, 2, 8], vec![6, 5, 6, 6]), 3),
            ((vec![2], vec![2]), 0),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((gas, cost), expected))| {
                let result = Solution::can_complete_circuit(gas, cost);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
