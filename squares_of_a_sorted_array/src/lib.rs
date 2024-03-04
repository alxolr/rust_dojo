struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; nums.len()];

        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut idx = nums.len();

        while left <= right {
            let left_sqr = nums[left].pow(2);
            let right_sqr = nums[right].pow(2);
            if idx.checked_sub(1).is_some() {
                idx -= 1;
            } else {
                break;
            }

            if left_sqr > right_sqr {
                result[idx] = left_sqr;
                left += 1;
            } else {
                result[idx] = right_sqr;
                if right.checked_sub(1).is_some() {
                    right -= 1;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec![1], vec![1]),
            (vec![-4, -1, 0, 3, 10], vec![0, 1, 9, 16, 100]),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::sorted_squares(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
