use std::collections::{HashMap, HashSet};
struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut complements: HashSet<i32> = HashSet::with_capacity(nums.len());
        let mut index_map = HashMap::new();

        for (idx, elem) in nums.iter().enumerate() {
            let complement = target - elem;

            if complements.contains(elem) {
                let start = target - elem;
                let start_position = index_map.get(&start).unwrap();

                return vec![*start_position as i32, idx as i32];
            }

            complements.insert(complement);
            index_map.insert(elem, idx);

        }

        panic!("unreachable code")
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![((vec![2, 7, 11, 15], 9), vec![0, 1])];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((nums, target), expected))| {
                let result = Solution::two_sum(nums, target);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
