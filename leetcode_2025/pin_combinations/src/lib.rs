use std::collections::HashMap;

impl Solution {
    fn split_pin(pin: i32) -> Vec<i32> {
        let mut pin = pin;
        let mut acc = vec![];

        while pin > 0 {
            acc.push(pin % 10);
            pin /= 10;
        }

        acc.reverse();

        acc
    }

    fn join_pin(arr: &[i32]) -> i32 {
        arr.iter()
            .rev()
            .enumerate()
            .fold(0, |mut sum, (ord, item)| {
                sum += *item * 10_i32.pow(ord as u32);

                sum
            })
    }

    fn backtrack(sets: &Vec<&Vec<i32>>, idx: usize, acc: &mut Vec<i32>, result: &mut Vec<i32>) {
        if idx == sets.len() {
            result.push(Self::join_pin(&acc));
            return;
        }

        for item in sets[idx] {
            acc.push(*item);
            Self::backtrack(sets, idx + 1, acc, result);
            acc.pop();
        }
    }

    pub fn pin_combos(pin: i32) -> Vec<i32> {
        let adjacency = vec![
            (0, vec![0, 7, 8]),
            (1, vec![1, 2, 4]),
            (2, vec![1, 2, 3, 5]),
            (3, vec![2, 3, 6]),
            (4, vec![1, 4, 5, 7]),
            (5, vec![2, 4, 5, 6, 8]),
            (6, vec![3, 5, 6, 9]),
            (7, vec![0, 4, 7, 8]),
            (8, vec![0, 5, 7, 8, 9]),
            (9, vec![6, 8, 9]),
        ]
        .into_iter()
        .collect::<HashMap<_, _>>();

        let sets: Vec<&Vec<i32>> = Self::split_pin(pin)
            .into_iter()
            .flat_map(|digit| adjacency.get(&digit))
            .collect::<Vec<_>>();

        let mut result = Vec::new();
        Self::backtrack(&sets, 0, &mut Vec::new(), &mut result);

        result
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join_pins_ok() {
        let scenarios = vec![
            (vec![1, 2, 3, 4], 1234),
            (vec![3, 2, 3, 4], 3234),
            (vec![3, 2, 3, 4, 5, 6], 323456),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::join_pin(&input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }

    #[test]
    fn test_split_pin_ok() {
        let scenarios = vec![
            (1234, vec![1, 2, 3, 4]),
            (3234, vec![3, 2, 3, 4]),
            (323456, vec![3, 2, 3, 4, 5, 6]),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::split_pin(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }

    #[test]
    fn it_works() {
        let scenarios = vec![(1234, vec![])];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::pin_combos(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
