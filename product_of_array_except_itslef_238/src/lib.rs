struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![1; nums.len()];
        let mut l: usize = 0;
        let mut r = nums.len() - 1;

        let mut lv: i32 = 1;
        let mut rv = 1;
        loop {
            result[l] = result[l] * lv;
            result[r] = result[r] * rv;

            rv = rv * nums[r];
            lv = lv * nums[l];

            if r == 0 {
                break;
            }
            l += 1;
            r -= 1;
        }

        return result;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            (vec![1, 2, 3, 4], vec![24, 12, 8, 6]),
            (vec![-1, 1, 0, -3, 3], vec![0, 0, 9, 0, 0]),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::product_except_self(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
