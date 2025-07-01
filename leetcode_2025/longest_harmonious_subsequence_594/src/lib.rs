// this is a sliding window algorith
// we will have a left and right starting at 0
// we track max_len
// we will move to the right
// we track max
// we track min
// if the diff is exactly one we check if len is bigger than max len
//    if yes than we update max_len

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut max_len = 0;

        let mut left = 0;
        let mut right = 0;

        let mut min = nums[0];
        let mut max = nums[0];

        while left <= right && left < nums.len() && right < nums.len() {
            right += 1;

            min = min.min(nums[right]);
            max = max.max(nums[right]);

            if max - min == 1 {
                max_len = max_len.max(right - left);
            } else {
                left += 1;
            }
        }

        max_len as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec![1, 3, 2, 2, 5, 2, 3, 7], 5),
            (vec![1, 2, 3, 4], 2),
            (vec![1, 1, 1, 1], 0),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::find_lhs(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
