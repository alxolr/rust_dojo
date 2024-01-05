use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut memo = HashMap::new();
        if !nums.is_empty() {
            // we start the sequence at first item
            dp(&mut memo, 0, i32::MIN, 1, &nums)
        } else {
            0
        }
    }
}

fn dp(
    memo: &mut HashMap<(usize, i32, i32), i32>,
    curr: usize,
    last: i32,
    count: i32,
    nums: &[i32],
) -> i32 {
    if let Some(result) = memo.get(&(curr, last, count)) {
        return *result;
    }

    if last >= nums[curr] {
        return count - 1;
    }

    if curr == nums.len() - 1 {
        return count;
    }

    let mut result = i32::MIN;

    for next in (curr + 1)..nums.len() {
        result = result.max(i32::max(
            dp(memo, next, nums[curr], count + 1, nums),
            dp(memo, next, last, count, nums),
        ));
    }

    memo.insert((curr, last, count), result);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_ok() {
        let scenarios = vec![
            (vec![1, 3, 6, 7, 9, 4, 10, 5, 6], 6),
            (vec![10, 9, 2, 5, 3, 7, 101, 18], 4),
            (vec![7, 7, 7, 7, 7, 7, 7], 1),
            (vec![0, 1, 0, 3, 2, 3], 4),
            (vec![4, 10, 4, 3, 8, 9], 3),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::length_of_lis(input);
                assert_eq!(result, expected);

                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
