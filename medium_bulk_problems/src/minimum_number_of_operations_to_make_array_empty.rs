use std::collections::HashMap;

fn _min_divisions(memo: &mut HashMap<i32, i32>, n: i32) -> i32 {
    if let Some(result) = memo.get(&n) {
        return *result;
    }

    match n {
        x if x < 0 => return i32::MAX / 2, // we do a big number but not max itself as we may have accidental sum plus overflow
        0 => return 0,
        2 => return 1,
        3 => return 1,
        _ => {
            let min = i32::min(
                1 + _min_divisions(memo, n - 3),
                1 + _min_divisions(memo, n - 2),
            );

            memo.insert(n, min);

            return min;
        }
    }
}

pub struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let dictionary = nums.iter().fold(HashMap::new(), |mut acc, item| {
            let count = acc.entry(item).or_insert(0);
            *count += 1;

            acc
        });

        let mut min_ops = 0;

        let mut memo = HashMap::new();
        for count in dictionary.values() {
            let ops = _min_divisions(&mut memo, *count);

            if ops >= i32::MAX / 2 {
                return -1;
            }

            min_ops += ops;
        }

        min_ops
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_divisions_ok() {
        let scenarios = vec![
            (1, i32::MAX / 2 + 1),
            (2, 1),
            (3, 1),
            (4, 2),
            (7, 3),
            (6, 2),
            (8, 3),
            (9, 3),
            (13, 5),
        ];

        let mut memo = HashMap::new();
        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = _min_divisions(&mut memo, input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }

    #[test]
    fn test_solution_ok() {
        let scenarios = vec![
            (vec![2, 3, 3, 2, 2, 4, 2, 3, 4], 4),
            (vec![2, 1, 2, 2, 3, 3], -1),
            (
                vec![
                    14, 12, 14, 14, 12, 14, 14, 12, 12, 12, 12, 14, 14, 12, 14, 14, 14, 12, 12,
                ],
                7,
            ),
            (vec![19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19, 19], 5),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::min_operations(input);
                assert_eq!(result, expected);

                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
