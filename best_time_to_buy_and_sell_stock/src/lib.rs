pub struct Solution;

fn compute_solution(value: i32, prices: &[i32], solution: &mut i32) {
    if prices.len() != 0 {
        let local_solution = prices.iter().map(|x| x - value).max().unwrap();

        if *solution < local_solution {
            *solution = local_solution;
        }

        let next_value = prices[0];
        let prices = &prices[1..prices.len()];

        compute_solution(next_value, prices, solution);
    }
}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let first = prices[0];
        let mut solution = i32::MIN;

        let prices = &prices[1..prices.len()];

        compute_solution(first, prices, &mut solution);

        if solution > 0 {
            solution
        } else {
            0
        }
    }

    pub fn max_profit_liniar(prices: Vec<i32>) -> i32 {
        let mut solution = i32::MIN;
        let mut left = 0;
        let mut right = 1;

        while right < prices.len() {
            if prices[left] < prices[right] {
                let local_solution = prices[right] - prices[left];
                solution = solution.max(local_solution);
            } else {
                left = right
            }

            right += 1;
        }

        solution
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::max_profit_liniar(vec![7, 1, 5, 3, 6, 4]);
        assert_eq!(result, 5);
    }
}
