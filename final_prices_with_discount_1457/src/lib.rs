pub struct Solution;

impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        prices
            .iter()
            .enumerate()
            .map(|(order, price)| {
                prices
                    .iter()
                    .skip(order + 1)
                    .find(|discount| *discount <= price)
                    .map_or(*price, |discount| *price - *discount)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(vec![8, 4, 6, 2, 3], vec![4, 2, 4, 2, 3])];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::final_prices(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
