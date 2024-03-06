struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = numbers.len() - 1;

        while left <= right {
            if numbers[left] + numbers[right] > target {
                if right == 0 {
                    break;
                }
                right -= 1;
            } else if numbers[left] + numbers[right] < target {
                left += 1;
            } else {
                break;
            }
        }

        vec![(left + 1) as i32, (right + 1) as i32]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ((vec![2, 7, 11, 13], 9), vec![1, 2]),
            ((vec![2, 3, 4], 6), vec![1, 3]),
        ];

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
