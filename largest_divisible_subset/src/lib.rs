use std::collections::HashMap;

struct Solution;

impl Solution {
    fn dfs(
        memo: &mut HashMap<(usize, i32), Vec<i32>>,
        curr_idx: usize,
        prev: i32,
        nums: &[i32],
    ) -> Vec<i32> {
        if curr_idx == nums.len() {
            return vec![];
        }

        if let Some(result) = memo.get(&(curr_idx, prev)) {
            return result.to_owned();
        }

        let mut result: Vec<i32> = Self::dfs(memo, curr_idx + 1, prev, nums);

        if nums[curr_idx] % prev == 0 {
            let mut tmp = vec![nums[curr_idx]];
            tmp.append(&mut Self::dfs(memo, curr_idx + 1, nums[curr_idx], nums));

            result = if result.len() > tmp.len() {
                result
            } else {
                tmp
            };
        }

        memo.insert((curr_idx, prev), result.clone());

        result
    }

    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();

        let mut memo = HashMap::new();

        Self::dfs(&mut memo, 0, 1, &nums)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec![1, 2, 3], vec![1, 2]),
            (vec![1, 2, 4, 8], vec![1, 2, 4, 8]),
            (vec![3, 4, 16, 8], vec![4, 8, 16]),
            (
                vec![5, 9, 18, 54, 108, 540, 90, 180, 360, 720],
                vec![9, 18, 90, 180, 360, 720],
            ),
            (
                vec![
                    1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768,
                    65536, 131072, 262144, 524288, 1048576, 2097152, 4194304, 8388608, 16777216,
                    33554432, 67108864, 134217728, 268435456, 536870912, 1073741824,
                ],
                vec![
                    1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768,
                    65536, 131072, 262144, 524288, 1048576, 2097152, 4194304, 8388608, 16777216,
                    33554432, 67108864, 134217728, 268435456, 536870912, 1073741824,
                ],
            ),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::largest_divisible_subset(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
