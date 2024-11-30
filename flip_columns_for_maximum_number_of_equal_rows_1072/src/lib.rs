use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let mut map = HashMap::new();
        let mut max = 0;

        for row in matrix.iter() {
            let mut key = row.clone();
            let mut flip = row.clone();

            for i in 0..row.len() {
                flip[i] = 1 - flip[i];
            }

            if flip < key {
                key = flip;
            }

            let count = map.entry(key).or_insert(0);
            *count += 1;

            if *count > max {
                max = *count;
            }
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec![vec![0, 1], vec![1, 1]], 1),
            (vec![vec![0, 1], vec![1, 0]], 2),
            (vec![vec![0, 0, 0], vec![0, 0, 1], vec![1, 1, 0]], 2),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::max_equal_rows_after_flips(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
