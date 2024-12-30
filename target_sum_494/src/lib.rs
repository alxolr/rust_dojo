use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut memoization = HashMap::with_capacity(2usize.pow(nums.len() as u32));
        let count = calculate_ways(&mut memoization, 0, target, &nums).unwrap();

        count as i32
    }
}

fn calculate_ways(
    memo: &mut HashMap<(usize, i32), Option<usize>>,
    idx: usize,
    target: i32,
    nums: &[i32],
) -> Option<usize> {
    if let Some(result) = memo.get(&(idx, target)) {
        return *result;
    }

    if idx == nums.len() {
        if target == 0 {
            return Some(1);
        } else {
            return None;
        }
    }

    let count = [
        calculate_ways(memo, idx + 1, target - nums[idx], nums),
        calculate_ways(memo, idx + 1, target + nums[idx], nums),
    ]
    .into_iter()
    .flatten()
    .sum();

    memo.insert((idx, target), Some(count));

    Some(count)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ((vec![1, 1, 1], 1), 3),
            ((vec![1, 1, 1, 1, 1], 3), 5),
            ((vec![1], 1), 1),
            ((vec![1, 0], 1), 2),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((nums, target), expected))| {
                let result = Solution::find_target_sum_ways(nums, target);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
