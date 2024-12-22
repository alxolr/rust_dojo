use std::collections::{HashMap, HashSet};

use crate::error::Result;
pub struct Solution;

impl Solution {
    pub fn part_1(input: &str) -> Result<u64> {
        // Very straightforward simulation you simulate all numbers and sum the resullt
        let result: u64 = load_input(input)
            .into_iter()
            .map(|secret| (0..2000).fold(secret, |acc, _| simulate(acc)))
            .sum();

        Ok(result)
    }

    pub fn part_2(input: &str) -> Result<u64> {
        let secrets = load_input(input);
        // The whole idea is that I store all ranges into hashmaps, [range]: price
        let secrets_ranges = transform_secrets_to_ranges(secrets);

        // Next stage we optimize by getting only the distinct ranges
        let unique_ranges = secrets_ranges
            .iter()
            .flat_map(|ranges_map| ranges_map.keys())
            .collect::<HashSet<_>>();

        // I compute the max possible value for each distinct range, finding their max
        let max_value = unique_ranges
            .into_iter()
            .map(|range| {
                secrets_ranges
                    .iter()
                    .flat_map(|range_map| range_map.get(range))
                    .sum::<u64>()
            })
            .max()
            .unwrap_or(0);

        Ok(max_value)
    }
}

// For each secret number, I generate a Hashmap of ranges, and their price
// Important to notice each range is identified with the last idx in the range price
fn transform_secrets_to_ranges(secrets: Vec<u64>) -> Vec<HashMap<Vec<i32>, u64>> {
    let secrets_to_ranges = secrets
        .into_iter()
        .map(|secret| {
            // I simulate 2000 iterations
            let (prices, diffs) = (0..2000)
                .scan(secret, |acc: &mut u64, _| {
                    *acc = simulate(*acc);

                    Some(last(*acc))
                })
                .scan(last(secret), |prev, curr| {
                    let diff = curr as i32 - *prev as i32;
                    *prev = curr;

                    Some((curr, diff))
                })
                .fold(
                    (vec![], vec![]),
                    |(mut prices, mut diffs), (price, diff)| {
                        prices.push(price);
                        diffs.push(diff);

                        (prices, diffs)
                    },
                );

            // Once I got the differences and the prices in two distinct arrays I can generate the hashmap
            diffs
                .windows(4)
                .enumerate()
                .fold(HashMap::new(), |mut acc, (id, curr)| {
                    // Important moment we need only the first occurance of the range
                    if !acc.contains_key(curr) {
                        acc.insert(curr.to_vec(), prices[id + 3]);
                    }

                    acc
                })
        })
        .collect();

    secrets_to_ranges
}

fn last(secret: u64) -> u64 {
    secret % 10
}

fn simulate(secret: u64) -> u64 {
    mul(div(mul(secret, 64), 32), 2048)
}

fn mul(secret: u64, by: u64) -> u64 {
    prune(mix(secret, secret * by))
}

fn div(secret: u64, by: u64) -> u64 {
    prune(mix(secret, secret / by))
}

fn prune(secret: u64) -> u64 {
    secret % 16777216
}

fn mix(secret: u64, value: u64) -> u64 {
    secret ^ value
}

fn load_input(input: &str) -> Vec<u64> {
    input.lines().flat_map(|line: &str| line.parse()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() -> Result<()> {
        let input = r#"1
10
100
2024"#;
        assert_eq!(Solution::part_1(input)?, 37327623);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input = r#"1
2
3
2024"#;
        assert_eq!(Solution::part_2(input)?, 23);

        Ok(())
    }
}
