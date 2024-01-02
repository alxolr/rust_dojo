use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut max_count = 0;

        let dictionary = nums.iter().fold(HashMap::new(), |mut acc, item| {
            let entry = acc.entry(item).or_insert(0);
            *entry += 1;

            max_count = max_count.max(*entry);

            acc
        });

        let mut result = vec![vec![]; max_count];

        for (item, count) in dictionary {
            for i in 0..count {
                result[i].push(*item)
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_ok() {
        let scenarios = vec![
            (
                vec![1, 3, 4, 1, 2, 3, 1],
                vec![vec![1, 3, 4, 2], vec![1, 3], vec![1]],
            ),
            (vec![1, 2, 3, 4], vec![vec![1, 2, 3, 4]]),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::find_matrix(input);

                assert_eq!(result.len(), expected.len());

                let mut result: Vec<i32> = result.into_iter().flatten().collect();
                result.sort();

                let mut expected: Vec<i32> = expected.into_iter().flatten().collect();
                expected.sort();

                assert_eq!(result, expected);

                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
