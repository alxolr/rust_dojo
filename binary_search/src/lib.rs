pub struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        nums.binary_search(&target).unwrap_or_else(|x| x) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![
            ((vec![1, 3, 5, 6], 5), 2),
            ((vec![1, 3, 5, 6], 6), 3),
            ((vec![1, 3, 5, 6], 7), 4),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, ((nums, target), expected))| {
                let result = Solution::search_insert(nums, target);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
