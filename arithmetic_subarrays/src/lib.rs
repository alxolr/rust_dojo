struct Solution;

impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        l.into_iter()
            .zip(r.into_iter())
            .map(|(l, r)| {
                let sub = &nums[l as usize..=r as usize];
                Self::is_arithmetic(sub)
            })
            .collect()
    }

    fn is_arithmetic(nums: &[i32]) -> bool {
        if nums.len() < 2 {
            return false;
        }

        let mut nums = nums.to_vec();
        nums.sort();

        let diff = nums[1] - nums[0];

        nums.windows(2).skip(1).all(|w| {
            let d = w[1] - w[0];
            d == diff
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_arithmetic_subarrays_ok() {
        assert_eq!(
            Solution::check_arithmetic_subarrays(
                vec![4, 6, 5, 9, 3, 7],
                vec![0, 0, 2],
                vec![2, 3, 5]
            ),
            vec![true, false, true]
        );
        assert_eq!(
            Solution::check_arithmetic_subarrays(
                vec![-12, -9, -3, -12, -6, 15, 20, -25, -20, -15, -10],
                vec![0, 1, 6, 4, 8, 7],
                vec![4, 4, 9, 7, 9, 10]
            ),
            vec![false, true, false, false, true, true]
        );
    }
}
