struct Solution;

impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![];
        let mut max = 0;
        let mut map = std::collections::HashMap::new();

        for (i, row) in nums.iter().enumerate() {
            for (j, num) in row.iter().enumerate() {
                let sum = i + j;
                let entry = map.entry(sum).or_insert(vec![]);
                entry.push(num);
                max = max.max(sum);
            }
        }

        for i in 0..=max {
            if let Some(entry) = map.get_mut(&i) {
                if i % 2 == 0 {
                    entry.reverse();
                }
                result.append(entry);
            }
        }

        result.iter().map(|&x| *x).collect()
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (
                vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
                vec![1, 4, 2, 7, 5, 3, 8, 6, 9],
            ),
            (
                vec![
                    vec![1, 2, 3, 4, 5],
                    vec![6, 7],
                    vec![8],
                    vec![9, 10, 11],
                    vec![12, 13, 14, 15, 16],
                ],
                vec![1, 6, 2, 8, 7, 3, 9, 4, 12, 10, 5, 13, 11, 14, 15, 16],
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::find_diagonal_order(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
