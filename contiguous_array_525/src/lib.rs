use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut zeros = 0;
        let mut ones = 0;
        let mut diff_map: HashMap<i32, usize> = HashMap::new();
        let mut result = 0;

        for (idx, item) in nums.iter().enumerate() {
            match item {
                &0 => zeros += 1,
                &1 => ones += 1,
                _ => {}
            }

            if diff_map.get(&(ones - zeros)).is_none() {
                diff_map.insert(ones - zeros, idx);
            }

            if zeros == ones {
                result = zeros + ones;
            } else {
                if let Some(prefix_idx) = diff_map.get(&(ones - zeros)) {
                    result = result.max((idx - prefix_idx) as i32)
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
            (vec![0, 0, 1, 0, 0, 0, 1, 1], 6),
            (vec![0, 1], 2),
            (vec![0, 1, 0], 2),
            (vec![0, 1, 0, 1], 4),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::find_max_length(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
