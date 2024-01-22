use std::collections::HashSet;
pub struct Solution;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let expected_set: HashSet<i32> = (1..=nums.len()).map(|x| x as i32).collect();
        let given_set: HashSet<i32> = nums.iter().cloned().collect();

        let missing_set = &expected_set - &given_set;
        let missing_value = missing_set.into_iter().next().unwrap();

        let mut duplicate = None;

        let mut seen = HashSet::new();
        for item in nums {
            if seen.contains(&item) {
                duplicate = Some(item);
                break;
            } else {
                seen.insert(item);
            }
        }

        vec![duplicate.unwrap(), missing_value]
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_ok() {
        let scenarios = vec![
            (vec![1, 2, 2, 4], vec![2, 3]),
            (vec![1, 1], vec![1, 2]),
            (vec![2, 3, 2], vec![2, 1]),
        ];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::find_error_nums(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
