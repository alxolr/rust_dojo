struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let dictionary = nums.iter().fold(HashMap::new(), |mut acc, item| {
            let entry = acc.entry(item).or_insert(0);
            *entry += 1;

            acc
        });

        let mut bucket = vec![vec![]; 100000];

        for (key, value) in dictionary {
            bucket[value as usize].push(*key);
        }

        bucket
            .into_iter()
            .rev()
            .filter(|x| !x.is_empty())
            .flat_map(|x| x)
            .take(k as usize)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ((vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]),
            ((vec![1], 1), vec![1]),
            ((vec![3,0,1,0], 1), vec![0]),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((nums, k), expected))| {
                let result = Solution::top_k_frequent(nums, k);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
