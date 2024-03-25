struct Solution;

impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut res = vec![];

        for pos in 0..nums.len() {
            let num = nums[pos].abs();
            let idx = (num - 1) as usize;

            if nums[idx] < 0 {
                res.push(num);
            }

            nums[idx] = -1 * nums[idx].abs()
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec![4, 3, 2, 7, 8, 2, 3, 1], vec![2, 3]),
            (vec![1, 1, 2], vec![1]),
            (vec![1], vec![]),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::find_duplicates(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
