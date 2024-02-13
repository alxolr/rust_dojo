pub struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let distinct1 = nums1.into_iter().collect::<HashSet<i32>>();
        let distinct2 = nums2.into_iter().collect::<HashSet<i32>>();
        let distinct3 = nums3.into_iter().collect::<HashSet<i32>>();

        distinct1
            .into_iter()
            .chain(distinct2.into_iter())
            .chain(distinct3.into_iter())
            .fold(HashMap::new(), |mut acc, item| {
                let entry = acc.entry(item).or_insert(0);
                *entry += 1;

                acc
            })
            .into_iter()
            .filter(|(_, v)| *v >= 2)
            .map(|(k, _)| k)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![((vec![1, 1, 3, 2], vec![2, 3], vec![3]), vec![3, 2])];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((nums1, nums2, nums3), expected))| {
                let result = Solution::two_out_of_three(nums1, nums2, nums3);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
