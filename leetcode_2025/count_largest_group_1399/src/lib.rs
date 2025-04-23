use std::collections::HashMap;


fn digit_sum(item: i32) -> u32 {
    item.to_string()
        .chars()
        .flat_map(|ch| ch.to_digit(10))
        .sum()
}

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let counts = (1..=n).map(digit_sum).fold(HashMap::new(), |mut acc, sum| {
            *acc.entry(sum).or_insert(0) += 1;
            acc
        });

        if counts.is_empty() {
            return 0;
        }

        let max_count = counts.values().max().unwrap();
        counts
            .values()
            .filter(|&&count| count == *max_count)
            .count() as i32
    }
}

pub struct Solution;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(13, 4), (2, 2)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::count_largest_group(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
