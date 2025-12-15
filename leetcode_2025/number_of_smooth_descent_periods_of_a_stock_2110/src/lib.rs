impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        let mut count_days = 0;
        let prices_len = prices.len();
        let mut left = 0;

        while left < prices_len {
            let mut right = left;

            while right + 1 < prices_len && prices[right + 1] == prices[right] - 1 {
                right += 1;
            }

            count_days += combos((right - left) + 1);
            left = right + 1;
        }

        count_days as i64
    }
}

fn combos(n: usize) -> usize {
    n * (n + 1) / 2
}

pub struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec![3, 2, 1, 4], 7),
            (vec![8, 6, 7, 7], 4),
            (vec![1], 1),
            (vec![12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 4, 3, 10, 9, 8, 7], 68),
        ];

        for (id, (input, expected)) in scenarios.into_iter().enumerate() {
            assert_eq!(Solution::get_descent_periods(input), expected);
            println!("Scenario {} ok", id + 1);
        }
    }
}
