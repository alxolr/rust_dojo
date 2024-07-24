pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut frequency = nums
            .iter()
            .fold(HashMap::new(), |mut acc, it| {
                let entry = acc.entry(it).or_insert(0);
                *entry += 1;

                acc
            })
            .into_iter()
            .collect::<Vec<_>>();

        frequency.sort_by(|a, b| {
            if a.1.eq(&b.1) {
                return b.0.cmp(a.0);
            } else {
                a.1.cmp(&b.1)
            }
        });

        frequency
            .into_iter()
            .flat_map(|(a, b)| vec![*a; b])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec![1, 1, 2, 2, 2, 3], vec![3, 1, 1, 2, 2, 2]),
            (vec![-1,1,-6,4,5,-6,1,4,1], vec![5,-1,4,4,-6,-6,1,1,1]),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::frequency_sort(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
