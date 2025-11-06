use std::{
    collections::{HashMap, HashSet},
};

#[derive(PartialEq, Eq, Debug)]
struct Node(i32, i32);

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // First compare by frequency (higher frequency first)
        // Then by value (higher value first) for ties
        match self.1.cmp(&other.1) {
            std::cmp::Ordering::Equal => self.0.cmp(&other.0),
            other_ordering => other_ordering,
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let find_sum = |nums: &[i32]| -> i32 {
            let counts = nums.iter().fold(HashMap::new(), |mut acc, it| {
                let entry = acc.entry(it).or_insert(0);
                *entry += 1;

                acc
            });

            let mut sorted: Vec<_> = counts.iter().map(|(num, count)| Node(**num, *count)).collect();
            sorted.sort_by(|a, b| b.cmp(a)); // Sort in descending order

            let biggest = sorted
                .iter()
                .take(x as usize)
                .map(|x| x.0)
                .collect::<HashSet<_>>();

            nums.iter().filter(|it| biggest.contains(it)).sum::<i32>()
        };

        nums.windows(k as usize).map(find_sum).collect()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ((vec![1, 1, 2, 2, 3, 4, 2, 3], 6, 2), vec![6, 10, 12]),
            ((vec![3, 8, 7, 8, 7, 5], 2, 2), vec![11, 15, 15, 15, 12]),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::find_x_sum(input.0, input.1, input.2);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
