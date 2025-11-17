impl Solution {
    fn backtrack(acc: &mut Vec<i32>, results: &mut Vec<Vec<i32>>, nums: &Vec<i32>) {
        if acc.len() == nums.len() {
            results.push(acc.clone());

            return;
        }

        for i in 0..nums.len() {
            if acc.contains(&nums[i]) {
                continue;
            }

            acc.push(nums[i]);
            Self::backtrack(acc, results, nums);
            acc.pop();
        }
    }

    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut results = vec![];
        Self::backtrack(&mut vec![], &mut results, &nums);

        results
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let scenarios = vec![(
            vec![1, 2, 3],
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1],
            ],
        )];

        scenarios
            .into_iter()
            .enumerate()
            .for_each(|(idx, (input, expected))| {
                let result = Solution::permute(input);
                assert_eq!(result, expected);
                println!("  ✓ scenario {}", idx + 1)
            });
    }
}
