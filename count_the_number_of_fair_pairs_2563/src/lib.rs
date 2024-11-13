pub struct Solution;

impl Solution {
    pub fn count_fair_pairs(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        let mut count = 0;

        let mut nums = nums;
        nums.sort(); // O (nlogn)

        for (id, num) in nums.iter().enumerate() {
            let low = Self::lower_bound(&nums, id + 1, nums.len() - 1, lower - num);
            let high = Self::lower_bound(&nums, id + 1, nums.len() - 1, upper - num + 1);

            if high - low > 0 {
                count += high - low;
            }
        }

        count as i64
    }

    fn lower_bound(nums: &[i32], mut low: usize, mut high: usize, item: i32) -> usize {
        while low <= high {
            let mid = low + ((high - low) / 2);

            if nums[mid] >= item {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }

        low
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ((vec![0, 1, 7, 4, 4, 5], 3, 6), 6),
            ((vec![1, 7, 9, 2, 5], 11, 11), 1),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((nums, lower, upper), expected))| {
                let result = Solution::count_fair_pairs(nums, lower, upper);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
