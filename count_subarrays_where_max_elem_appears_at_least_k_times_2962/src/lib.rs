struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let max_elem = *(nums.iter().max().unwrap_or(&0));
        let mut max_freq = 0;
        let mut ans = 0;

        let mut start = 0;
        for end in 0..nums.len() {
            if nums[end] == max_elem {
                max_freq += 1;
            }

            while max_freq == k {
                if nums[start] == max_elem {
                    max_freq -= 1;
                }

                start += 1;
            }

            ans += start;
        }

        ans as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![((vec![1, 3, 2, 3, 3], 2), 6), ((vec![1, 4, 2, 3], 3), 0)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((nums, k), expected))| {
                let result = Solution::count_subarrays(nums, k);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
