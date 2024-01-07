struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut last = nums[0];
        let mut deduplicated_vec = vec![last];

        for i in 1..nums.len() {
            if last != nums[i] {
                deduplicated_vec.push(nums[i]);
                last = nums[i];
            }
        }

        let len = deduplicated_vec.len();

        for i in 0..deduplicated_vec.len() {
            nums[i] = deduplicated_vec[i];
        }

        len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_ok() {
        let scenarios = vec![
            (vec![1, 1, 2], 2),
            (vec![0,0,1,1,1,2,2,3,3,4], 5),

        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (mut input, expected))| {
                let result = Solution::remove_duplicates(&mut input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
