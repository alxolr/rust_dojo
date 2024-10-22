use std::mem::swap;

pub struct Solution;

impl Solution {
    fn expand(num: i32) -> Vec<u32> {
        num.to_string()
            .chars()
            .flat_map(|x| x.to_digit(10))
            .collect()
    }

    fn collapse(nums: &Vec<u32>) -> i32 {
        nums.iter()
            .map(|x| x.to_string())
            .collect::<String>()
            .parse()
            .unwrap_or(0)
    }

    pub fn maximum_swap(num: i32) -> i32 {
        let mut nums = Self::expand(num);

        let mut max_seen = vec![0; nums.len()];
        let mut max = i32::MIN;

        for i in (0..nums.len()).rev() {
            max = max.max(nums[i] as i32);
            max_seen[i] = max;
        }

        let mut left = 0;
        let mut right = 1;

        while left < right && right < nums.len() {
            
        }
        

        Self::collapse(&nums)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(12, 21), (2736, 7236), (9973, 9973)];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::maximum_swap(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
