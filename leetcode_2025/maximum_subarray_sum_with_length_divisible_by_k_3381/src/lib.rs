impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut min_remainders_sum = vec![i64::MAX / 2; k as usize];
        min_remainders_sum[0] = 0;

        let mut total = 0i64;
        let mut res = i64::MIN;

        for (idx, num) in nums.iter().enumerate() {
            total += *num as i64;
            let length = idx + 1;
            let remainder_idx = length % k as usize;

            res = i64::max(res, total - min_remainders_sum[remainder_idx] as i64);
            min_remainders_sum[remainder_idx] = i64::min(min_remainders_sum[remainder_idx], total);
        }

        res as i64
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ((vec![1, 2], 1), 3),
            ((vec![-1, -2, -3, -4, -5], 4), -10),
            ((vec![-5, 1, 2, -3, 4], 2), 4),
        ];

        for (idx, (input, expected)) in scenarios.into_iter().enumerate() {
            assert_eq!(Solution::max_subarray_sum(input.0, input.1), expected);
            println!("Scenario {} ok", idx + 1);
        }
    }
}
